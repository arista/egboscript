use egbo::{model, model_builder, model_json, parser, peg_parser};

// Build a model and hand back both it and the file ptr, for accessor tests.
fn build(src: &str) -> (model::Model, model::ItemPtr<model::FileData>) {
    let mut pp = peg_parser::PegParserImpl::new(src);
    let p = parser::Parser::new();
    let parsed = p.file(&mut pp).expect("parse failed");
    let mut m = model::Model::new();
    let file = model_builder::add_file_to_model(&"test.egbo".to_string(), &parsed, &mut m);
    (m, file)
}

fn dump(src: &str) -> String {
    let (m, file) = build(src);
    model_json::file_to_json(&m, file, &model_json::DumpOpts::default()).to_string()
}

#[test]
fn dumps_function_with_binary_expr() {
    let out = dump("function add(a, b) { return a + b; }");
    println!("{out}");

    assert!(out.contains("\"kind\": \"File\""));
    assert!(out.contains("\"kind\": \"FunctionDeclStatement\""));
    assert!(out.contains("\"name\": \"add\""));
    assert!(out.contains("\"kind\": \"FunctionDeclArg\""));
    assert!(out.contains("\"kind\": \"ReturnStatement\""));
    assert!(out.contains("\"kind\": \"BinaryExpression\""));
    assert!(out.contains("\"op\": \"Plus\""));
    assert!(out.contains("\"kind\": \"IdentifierExpression\""));
}

#[test]
fn dumps_var_decl_with_optionals() {
    let out = dump("let x = 1;");
    println!("{out}");

    assert!(out.contains("\"kind\": \"VarDeclStatement\""));
    assert!(out.contains("\"let_or_const\": \"Let\""));
    assert!(out.contains("\"name\": \"x\""));
    assert!(out.contains("\"kind\": \"IntLiteral\""));
    assert!(out.contains("\"value\": 1"));
    // No suffix on the literal -> null
    assert!(out.contains("\"suffix\": null"));
}

#[test]
fn span_resolves_for_handles() {
    let src = "let x = 1;";
    let (m, file) = build(src);

    // Every concrete handle gets span() for free via Item<T>.
    let f = m.item(file);
    let f_span = f.span().expect("file should have a span");
    assert!(f_span.span.start <= f_span.span.end);

    // Walk to the var decl through the enum handle; it has span() too.
    let stmt = f.items().next().expect("one item");
    let model::FileItem::Statement(s) = stmt else { panic!("expected a statement") };
    let span = s.span().expect("statement should have a span");
    // "let x = 1;" spans the whole source.
    assert_eq!(span.span.start, 0);
    assert_eq!(&src[span.span.start..span.span.end], "let x = 1;");
}

#[test]
fn span_flag_controls_json_output() {
    let (m, file) = build("let x = 1;");

    // Default: no span fields anywhere.
    let plain = model_json::file_to_json(&m, file, &model_json::DumpOpts::default()).to_string();
    assert!(!plain.contains("\"span\""));

    // With spans: every node carries a SourceLocation object.
    let with = model_json::file_to_json(&m, file, &model_json::DumpOpts::with_spans()).to_string();
    println!("{with}");
    assert!(with.contains("\"span\": {"));
    assert!(with.contains("\"source_file\":"));
    assert!(with.contains("\"start\": 0"));
    assert!(with.contains("\"end\":"));
}
