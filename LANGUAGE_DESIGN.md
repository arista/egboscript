# Language Design

Egboscript is a programming language targeted for use in constrained environments, such as embedded systems or plug-in hosts.  Its main feature is that it can be compiled to a bytecode format in which its maximum runtime memory requirements are absolutely known.  This allows hosts and developers to determine if a program can safely "fit" in a particular environment.

Its memory model does not rely on garbage collection, allowing its memory use to be predictable and deterministic.  Its virtual machine can be directed to run in incremental steps, allowing the host to limit the execution time granted to a running script.

The language is designed to be simple to pick up, reducing the barrier for entry into embedded systems or plug-in development.  But it also many modern features that allow for expressive and type-safe program development.

As far as I'm aware, this language occupies a niche that isn't quite filled by any other language.

## Approach

The script uses TypeScript as its design starting point, borrowing much of its syntax and concepts.  It does not, however, use any TypeScript or JavaScript runtime technology, so it is not constrained by any of the limitations in that language.

The main self-imposed limitation is that the maximum memory usage be known at compile time.  To achieve this, the language is designed with the following constraints:

* No recursive function calls
    * This places a deterministic limit on stack size
    * This may allow some static analysis that wouldn't be possible otherwise.  For example, it may be possible to trace the "pedigree" of all inputs and outputs in order to check for certain cnostraints at runtime
* No general heap allocation
    * Structures are created as global variables (or stack?  see discussion below)
    * type-specific "Arenas" are available but must be declared with the maximum # of items they can hold
    * Structs and arrays are available, but arrays must declare the maximum # of items, and structures can't be recursive
* Limited String construction
    * A corrolary to "no heap allocation"
    * String literals in code are represented in a constants table
    * Dynamic string construction still being considered
        * Template literals, in which the "${...} sections must specify maximum length thereby constraining the length of the overall string
        * Perhaps add String to the structs and arrays, where a String "slot" can be declared with a maximum size

## Pointers and Stack structures

Pointers introduce complexity, but are difficult to avoid.  They are an important abstraction that we want programs to be able to use.  They are also related to the ability to pass structures around, which is another desirable feature.

So what are some options:

* No pointers at all
    * No pointers allowed at all
    * All references must start from a global struct and "drill down" to whatever data is needed
    * Not sure how this would work with arena-based allocation
    * Seems like it would be super-annyoing to use
    
* Pointers allowed, but not in data structures
    * Data structures can have nested structures, but they're all stored "inline"
    * Pointers can only be to global data structures
    * No circular references are possible, so reference-counting can be used for freeing arena resources
    * When arenas allocate, they return pointers, but those are still effectively pointers to global data
        * those pointers are reference counted
        * implies that the compiler needs to support RAII
    * Code can use pointers, including pointers to nested items in structures.  Pointers can be stack variables, function arguments and return values, etc.
    * Usage is simple - in a struct declaration, any use of a type other than a primitive is an inlined type, and in code any use of such a type is a pointer.  Code doesn't need to make distinctions between "pass by value" vs. "pass by reference".
    * Pointers to enums, or data within enums, is tricky (basically a generalization of null pointers)
        * Perhaps a runtime mechanism "locks" such enums while a nested pointer is in play (using the same RAII required for arenas), so that the enum can't change to a different variant
        * Perhaps static code analysis can catch cases of trying to change an enum while a nested pointer is held.

The second seems like a reasonable compromise.  The question is whether this will be too limiting for developers.  Are there really useful cases for storing pointers in structs and arrays that would be terribly missed?  Are there some compromises that could be made?

* No circular references - structures can hold pointers, but can't declare fields could directly or indirectly lead to a recursive reference
    * Allows reference counting to still work

## A Far-Out Thought

Is it possible to do away with arenas altogether, and give the illusion of a heap but still understand maximum memory usage through static analysis?

If we are still able to use RAII and no recursive function calls, then in theory there is a limit to the number of places you could actually store pointers to objects, which could serve as an arena hint.  Is it possible to statically analyze this?

## Other thoughts


With the abaove constraints in mind, how can we squeeze as much power as we can into the language.  Can we, for example, include:

* function closures
    * perhaps this is possible under certain circumstances
        * "inlining" a function and passing it to another function should be possible 
        
* enums (and by implication, nullable values)
    * rust-style enums are quite handy, if they can be accessed in a non-complex way (full match syntax and destructuring might be a bit much)
* concurrency - while yet to be designed, concurrent programming will likely be a feature, with the equivalent of async functions.
    * still to be determined if full-on Promises will be first-class concepts in the language
    * regardless, will need primitives for running several functions and waiting for them to complete, or for one to complete, etc.
    * would be nice to avoid having to use "async/await".  Perhaps static analysis automatically determines if a function needs to be "colored" this way
    * still must be able to statically determine the maximum number of these running, so we can determine max memory usage
        * perhaps use RAII for these - a function can't be left to run "on its own" without some reference holding on to it (the concurrency primitives would count for this)
        * or perhaps use arenas for these, although that's more of a burden on the developer
* iterators and generators - it would be nice if these were possible
    * means allocating stack frames
    * similar to the issues around concurrency in terms of understanding max memory usage

How can we make the language more approachable?

* Allow for a "non-strict" mode, where:
    * type declarations are not required for struct fields, for function declarations
    * places where sizes must be declared (arena sizes, array sizes, string sizes) will have defaults
