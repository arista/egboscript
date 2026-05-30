use std::fs;
use std::process::Command;

#[test]
fn compile_copies_input_to_output() {
    let dir = tempdir();
    let input = dir.join("in.txt");
    let output = dir.join("out.txt");
    let contents = "hello, egbo\nline two\n";
    fs::write(&input, contents).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_egbo"))
        .arg("compile")
        .arg(&input)
        .arg(&output)
        .status()
        .unwrap();
    assert!(status.success(), "egbo exited with {status}");

    let written = fs::read_to_string(&output).unwrap();
    assert_eq!(written, contents);
}

fn tempdir() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!(
        "egbo-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&p).unwrap();
    p
}
