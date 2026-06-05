# Analysis of LANGUAGE_DESIGN.md

## Niche assessment

The claimed niche — "TypeScript-like surface, bytecode VM, statically-bounded memory, no GC, incremental execution" — is real and underserved. The closest neighbors:

- **Lua / eLua** — small VM, embeddable, but heap-allocated and GC'd. No static memory bound.
- **MicroPython** — same story; not predictable.
- **Wasm (with restricted toolchain)** — bounded linear memory, but you write C/Rust/AssemblyScript and ship a runtime. Not approachable.
- **Rust no_std** — bounded if you avoid `alloc`, but the learning curve is the opposite of "low barrier."
- **eBPF** — bounded execution, statically verified, but intentionally non-Turing-complete and not a general scripting language.
- **Wren, Gravity, Squirrel** — embedded scripting languages, all GC'd.
- **Roblox Luau** — typed Lua dialect; closer in spirit but still GC'd.

So the gap is genuine: *typed, scripting-feel, no GC, statically bounded, embeddable*. The pitch is credible.

The two existential risks are (1) the constraints make the language so awkward that people pick Lua + accept GC instead, and (2) static memory bounds become so loose in practice that the guarantee loses operational value. Both are worth watching as the design progresses.

## The "max memory known at compile time" guarantee

This is the load-bearing claim. A few things worth pinning down before too much else is built:

1. **What does "max memory" include?** Globals + per-frame stack × max call depth + pool capacity × element size + constant pool + VM bookkeeping. Each should be a separate, reportable number in the compiler's memory budget report.
2. **Tightness vs. soundness.** The bound only needs to be *sound* (never exceeded), not *tight*. But a loose bound erodes the value prop. Worth tracking headroom — for typical programs, how close does actual usage get to the static bound?
3. **Stack frames are the subtle one.** No recursion fixes the depth, but frame *size* depends on locals + temporaries + spilled values. The compiler needs a clear story for "this function's frame is N bytes" and the call graph analysis to sum them along the longest path.

Recommend: make the compiler emit a memory budget report as a first-class artifact, not an afterthought.

## No recursion

Right call for this design. Two follow-ups:

- **Mutual recursion** must also be banned, which means the compiler needs full call-graph analysis (already needed for max-stack-depth). Cheap to enforce once you have the graph.
- **Bounded recursion** (recurse at most N times) is sometimes proposed as an escape hatch. I'd resist it — slippery slope and the static analysis gets hairy. If users need tree traversal, give them an explicit stack-in-a-pool pattern.
- **Indirect calls / function pointers** complicate the call graph. If they're allowed at all, they likely need to be typed in a way that bounds the set of possible callees (e.g., a closed enum of function references), so the graph stays analyzable.

## Memory and ownership model

The design has converged on a coherent point that's worth writing down precisely. Two value categories, one runtime mechanism for liveness, one runtime mechanism for variant stability.

### Pools, not arenas

The original doc said "arena" but described pool semantics: one type per pool, fixed slot count, individual objects allocated and freed (not bulk-freed), refcounting on handles. That's a pool (or slab allocator), not an arena. Renaming clarifies the model and tightens the memory-bound story: a pool of N `T`s costs exactly `N * sizeof(T) + free-list bookkeeping`, with no fragmentation or allocation-pattern-dependent overhead.

The "no circular references" rule reads cleanly in pool terms: in the type graph, no struct/pool/enum type can transitively reach itself through a field whose type is a refcounted reference (handle or payload ref). DAG check on the type graph at compile time.

### Lifetime categories

Every value lives in one of two regimes:

1. **Lexically scoped values** — globals (program lifetime), function locals and arguments (scope lifetime), and the inlined sub-fields of either. The compiler knows where they begin and end.
2. **Pool-allocated values** — created at runtime, freed when refcount hits zero. Lifetime is dynamic but bounded by pool capacity.

There are no stack-allocated structs. All structs are pool-allocated and accessed through refcounted handles. This is a deliberate ergonomic choice: it keeps value-vs-reference semantics out of the user's mental model entirely. No `&` or `*` operators, no escape analysis for "when does a struct copy become a pointer pass," no per-call-site decisions for the developer. The cost is that every struct allocation requires a pool with declared capacity, even for short-lived bundles; the upside is dramatic simplification of the language surface.

### Refcounted values

Two kinds of refcounted values, mechanically uniform, semantically distinct:

- **Pool handle** — a refcounted reference to a slot in a pool. Pins the slot's allocation: while any handle exists, the slot is not reused. Storable anywhere (struct fields, function arguments, locals).
- **Payload ref** — a refcounted reference into the payload of a specific enum variant. Pins the enum's variant: while any payload ref exists, the enum's variant cannot be changed (mutation traps at the mutation site). Also storable anywhere.

Both follow the same RAII discipline:

- Creating the reference (handle on pool allocation, payload ref on match-arm binding or by deriving from another payload ref) increments a counter.
- Copying the reference into another slot increments the counter.
- Destroying the slot (lexical scope exit, struct destructor when its own refcount hits zero) decrements.

The same machinery covers locals, function arguments, and struct fields. The compiler does not need to track reference lineage through function calls — the counter lives on the referent (the pool slot or the enum), not on the references — so propagation across calls is free. The static work reduces to:

- Inc/dec codegen at scope and field-storage boundaries (standard RAII).
- A type-graph DAG check (no field-cycle through refcounted-reference types — including both handle and payload-ref fields).
- A counter check at variant-change sites (for enums).

That's modest. None of it requires lifetime parameters, mut/non-mut annotations, flow-sensitive borrow checking, or effect tracking.

### Why this works without mut/non-mut

A reasonable critique is that "match-arm binding protects the payload from variant changes" usually rests on Rust-style aliasing rules (`&T` excludes `&mut T`). Without that distinction, the static side can't prove safety. The runtime mechanism replaces it: the enum's borrow counter is incremented by the binding's existence, not by reasoning about who can write. Anyone (script code in nested calls, native code via FFI, the host between slices) that tries to change the variant while the counter is positive traps at the mutation site. The check is local — it inspects the target enum's counter — so no global analysis is required.

## Enum payload references in detail

Because this was the central design question, it's worth writing the model down end-to-end:

- A `match` arm with a payload binding produces a payload ref. The binding's existence increments the enum's borrow counter; the binding going out of scope decrements it.
- Payload refs can be passed as function arguments, returned, copied into other locals, stored in struct fields (including persistent pool-allocated structs). Each storage location follows the same inc-on-store, dec-on-drop discipline.
- Variant-change operations (assigning a new variant to the enum) check the counter first. If positive, the operation traps at the mutation site. This gives precise error attribution: the failure points at *who tried to change the variant*, not at the holder of a stale reference.
- Stale payload refs are structurally impossible — the variant cannot have changed while the ref exists, because the counter held it in place. There is no "stale deref" failure mode.

Two specific scenarios worth flagging:

**Long-lived locks via persistent storage.** A payload ref stored in a pool-allocated struct field holds the enum's variant locked for the lifetime of that struct. This is a real design pattern (transaction-style "I'm in variant X for the duration") but also a footgun risk (forgotten ref pinning an enum forever). Debug-mode tooling should be able to report "lock held by N references; most recent acquired at file:line" to make non-local failures diagnosable.

**Cycle prevention.** The no-cycles DAG check must include payload-ref-typed fields, not just handle fields. Otherwise constructions like `enum E::A(handle_to_S)` paired with `struct S { ref: PayloadRef<E::A> }` produce cycles that prevent both the struct and the variant from ever being released. Widening the existing static check covers this.

This design was reached by working through and rejecting two alternatives:

- **Generation tags on enums** (variant change always succeeds, stale deref traps): rejected because the trap fires far from the cause, hurting debuggability.
- **Disallowing payload refs in any struct field**: rejected because it would prevent legitimate uses of short-lived bundles to group variables, and the runtime lock mechanism is uniform enough that the distinction adds no safety.

### Static analysis as a complement to the runtime lock

The original design doc raised whether static analysis could catch enum-variant-change violations at compile time. With the runtime lock as the committed mechanism, the question is now about whether (and how far) static analysis should layer on top. Three flavors:

**As a hard compile-time error** — don't do this. To be sound, the analysis has to be conservative; conservative means false positives; false positives mean rejecting code that would run fine. It re-introduces the borrow-checker complexity the runtime lock was designed to avoid, and creates two semantic systems (static rules + runtime checks) that can drift apart. The whole appeal of the runtime model is that there's one rule.

**As a lint / warning** — worth doing, in a narrow scope. The runtime check stays authoritative, but the compiler emits warnings for cases it can prove are violations. Restrict the lint to *unconditional* violations:

- Within a single function, between a match-arm binding and a direct assignment to the same enum, no intervening function call: trivial AST-level check, zero false positives.
- A function whose body unconditionally violates regardless of which path runs: also cheap.
- "Possibly violates" cases (depends on branches, depends on what called functions do): leave to runtime. Don't flag.

The rule of thumb: warn only when the violation would *always* happen, never when it *might* happen. This catches embarrassing bugs early without committing to a sound static system.

**As an optimization (elide unnecessary counter operations)** — pure win when it works. If the compiler can prove nothing inside a match arm could change the matched enum's variant (no assignments to that enum, no calls to functions that could reach it), the counter inc/dec at the arm boundaries can be elided. Same for the mutation-site check: if no payload ref to this enum can exist at this program point, skip the counter read. These optimizations don't affect semantics — correct programs behave identically — they just remove overhead. Worth investing in once there's measurable code to optimize.

The model is the same as how Rust treats `RefCell`: runtime is authoritative, static analysis helps where it's cheap and unambiguous, no attempt to make the static system replace the runtime one.

## FFI shape

The host language story (Rust as the likely first target) falls out of the model:

- The host holds **handles**, not raw pointers. Crossing the FFI boundary bumps refcounts; releasing the handle decrements. Same lifecycle as Python's C API (`Py_INCREF`/`Py_DECREF`), but with the bookkeeping discipline baked into the binding rather than the host's responsibility.
- Accessing enum payloads happens via either **value-copy reads** (`read_variant_A(handle) -> Option<VariantAData>`) or **scoped callbacks** (`with_variant_A(handle, |payload| { ... })`). The callback variant takes the enum's borrow lock for the duration of the call; the host pointer is only valid inside the closure.
- Variant mutations are their own calls (`set_variant_B(handle, data)`). If the enum is borrowed (script side or host side), the call traps with a useful message.
- The host never gets long-lived raw pointers across FFI calls. The runtime guarantees nothing about pointers across API calls, only about handles. This is a much simpler contract to document and uphold.

This shape maps directly onto Rust host idioms (`RefCell::borrow_mut`-style guards), which makes the FFI feel native to Rust developers rather than foreign.

## Stale handles

In the model as described, **stale handles are structurally impossible**. The proof is small:

- Refcounting prevents premature free (slot stays live while any handle exists).
- Strong typing prevents reinterpretation (a `Handle<Foo>` only derefs into the Foo pool).
- No cycles → refcounts actually reach zero correctly.
- No unsafe casts → handles can't be smuggled through integers and rehydrated.
- Payload-ref variant pinning prevents variant-level staleness via the same counter mechanism.

This invariant should be written down prominently in the design doc, because future features could quietly break it:

- **Weak references** (handle that doesn't increment refcount) can go stale; would need generational tags.
- **Serialization / snapshot-restore** would require revalidation at load time.
- **FFI escape hatches** that hand out raw pointers across calls would defeat the proof.
- **Multi-threaded execution** would require atomic refcounts or per-task pools.

None of these are in the current design. Writing the invariant down forces any future addition to either preserve it or explicitly opt out, rather than silently degrading the guarantee.

## Concurrency model

The original doc raised concurrency as "yet to be designed" and asked whether async/await coloring could be avoided. With the rest of the model now settled, the concurrency design follows directly: **no function coloring, no first-class Promises, structured-concurrency primitives only.**

### No coloring

The compiler determines suspending-ness from the call graph — a function can suspend iff it calls a suspending primitive or any function that can suspend. This is the same closed-world analysis already required for max-stack-depth, so it's free. Developers don't write `async`; the compiler infers it. Adding a suspending call to a previously-non-suspending function doesn't ripple through call sites as a refactoring penalty.

This is the Go / Trio / Kotlin-coroutines model, validated at scale. The friction with colored functions in JS/Rust comes mostly from large async-combinator ecosystems, which embedded scripting doesn't need.

### No first-class Promises, no `await`

Once Promises aren't exposed as values, `await` has nothing to do. Every call either suspends until completion or doesn't — the caller has no choice to defer. This removes a whole concept from the language surface. The common patterns that justify explicit `await` in other languages move to structured primitives instead:

| Pattern | Promise-based form | Egboscript form |
|---|---|---|
| Wait for all | `await Promise.all([a, b])` | `parallel { a(); b() }` |
| First to complete | `await Promise.race([a, b])` | `race { a(); b() }` |
| Dynamic fan-out | `await Promise.all(items.map(f))` | `parallel_for max=N item in items { f(item) }` |
| Timeout | `Promise.race([task, delay])` | `race { task(); delay(5s) }` |
| Async iteration | `for await (const x of stream)` | regular `for` over a suspending iterator |
| Cancellation | abort signal on Promise | propagates from enclosing block on failure or race-completion |

What this gives up:

- **Returning "a thing that completes later" as a value.** Callers can't hold a token to await elsewhere; they wait inline or structure work into a concurrent block. Matches the structured-concurrency philosophy.
- **User-defined combinators.** No first-class futures means no custom `Promise.all`. Acceptable — the language provides the primitives, the user composes with them.
- **Fire-and-forget tasks.** Every task is owned by an enclosing block. The "logger running alongside main work" pattern becomes the outermost program structure — `parallel { logger(); main() }` — rather than a detached spawn. Stylistic shift but no capability loss.

### The three primitives

- `parallel { a(); b(); c() }` — runs N statically-known branches concurrently, returns when all complete. Slot cost: N frames.
- `race { a(); b(); c() }` — runs N branches concurrently, returns when the first completes (others cancelled). Slot cost: N frames.
- `parallel_for max=N item in items { body(item) }` — bounded-concurrency fan-out over a collection. Slot cost: N frames regardless of `items.length`. Without this, dynamic fan-out is impossible; with it, the structured model covers essentially the full practical surface.

Pool sizing falls out directly from the structured primitives — no inference needed. Each `parallel` block declares its task count syntactically; `parallel_for` declares it via `max=`. The compiler tabulates suspended-frame pool sizes by summing across concurrently-live blocks in the call graph.

### Cancellation and error propagation

The model needs to define what happens to sibling tasks when one branch fails or a `race` resolves. Trio's answer (and the convergent answer across structured-concurrency designs) is:

- A failure in one branch cancels its siblings and propagates out.
- A `race` winner cancels its siblings.
- Cancellation points are the suspending primitives — a cancelled task wakes at its next suspension, observes cancellation, and unwinds cleanly through its scope (releasing locks, decrementing refcounts via the existing RAII paths).

This needs concrete design — the surface for "observing cancellation" in a suspending primitive, the exact unwind semantics — but it's a known-tractable problem and the rest of the model composes with it cleanly (RAII handles resource release on unwind).

### Single-threaded VM

A single-threaded VM with cooperative scheduling at suspension points is the right substrate. It makes refcounts and lock-counter operations non-atomic (free), keeps the runtime model simple, and matches the incremental-execution story already in the design. True parallelism across cores is not the goal — concurrent waiting on I/O / events is.

## The "far-out thought" — heap illusion via static analysis

Worth investigating, but defer. Inferring pool capacities from program structure (rather than developer declarations) is more tractable than the original "inferred arenas" version — it reduces to "how many live `T`s can exist simultaneously?", a counting problem on handle storage slots rather than a flow-sensitive arena-extent problem. Still nontrivial, but plausible as a v2 ergonomic improvement.

For now, ship explicit pool capacities. Revisit if real users find the declaration overhead painful.

## Feature feasibility

| Feature | Verdict | Notes |
|---|---|---|
| Rust-style enums (sum types) | **Yes** | Worth the complexity — they pay for nullable safety, error handling, and state machines all at once. Match arms with payload bindings work via the lock-on-borrow mechanism described above. |
| Closures | **Constrained yes** | Non-escaping closures (passed down, never stored) fit the model naturally. Storing closures requires the closure's captures live in a pool slot, which works but pushes the user into thinking about capture lifetimes. |
| Concurrency | **Yes, structured-only** | See the dedicated concurrency-model section. Three primitives (`parallel`, `race`, `parallel_for`), no function coloring, no first-class Promises, no `await`, no detached spawn. Single-threaded VM with cooperative scheduling. |
| Iterators | **Yes** | Iterator *protocol* (a struct with `next()`) is free — no special VM support. |
| Generators | **Yes, with a pool slot** | Each generator's stack frame lives in a pool slot of declared capacity. The cost is visible (one slot per concurrently-live generator instance) but bounded. Worth offering. |
| Non-strict mode | **Skeptical** | Defaults for sizes erode the central guarantee silently. Better path to approachability: great error messages and loud defaults, not hidden ones. |

## Strings

Still the biggest day-one usability question, and the pool model makes the shape clearer:

- **String slots with declared max size** in struct fields — yes, do this. Conceptually `pool<string<N>, K>` falls out naturally.
- **Template literals with bounded interpolation** — make the bound part of the type (`string<64>`).
- **Be explicit about encoding** (likely UTF-8) and whether "max size" means bytes or codepoints. This matters more than it sounds.
- **String concatenation** needs a story. `a + b` where `a: string<32>` and `b: string<16>` → `string<48>`? Type inference on string sizes will get interesting.
- A small stdlib of fixed-buffer primitives (formatters, parsers) should thread the size discipline through end-to-end.

## Open questions

Now that the ownership model is settled, the remaining design surface narrows:

1. **VM memory layout at a high level.** Globals region, stack region, pools, constants — exact sizes and how they're computed for the budget report.
2. **Worked examples.** A small embedded app (MIDI plugin, sensor poller) written in the proposed syntax would expose ergonomic problems faster than further design discussion.
3. **Error model.** No exceptions (heap implications). Result-style enums for recoverable errors, runtime traps for invariant violations (lock-held mutations, out-of-pool allocations)?
4. **Module / compilation unit story.** Hosts that load multiple plugins need cross-module memory accounting.
5. **Cancellation semantics.** The structured concurrency model is settled; the exact mechanics of how cancellation observes/unwinds through suspending primitives needs concrete design.
6. **Debug tooling for the runtime lock mechanism.** "Who's holding the lock" diagnostics are important enough that they should be a first-class part of the runtime, not an afterthought.

## Overall

The design has a clear, defensible thesis, and the constraints fall out of it consistently. The recent work clarified the central memory model: pool-allocated values with refcounted handles, lexically-scoped references with no stack structs, and a uniform runtime lock mechanism for enum payload references. This replaces classical borrow checking with a simpler runtime mechanism while preserving the safety properties.

The remaining risks are ergonomic: getting strings, concurrency surface, and pool-capacity declaration to feel light enough that a developer reaches for this over Lua. A small end-to-end vertical slice (parser → bytecode → VM → one nontrivial example program) is now the right next investment — the trade-offs in the design are well-defined enough that real code will expose the remaining frictions faster than continued discussion.
