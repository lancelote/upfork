use std::io;
use std::process::{Command, Output, exit};
use std::io::Write;

// ToDo: move ToDos to GitHub issues
// ToDo: read Rust CLI book https://rust-cli.github.io/book/index.html
// ToDo: proper CLI (e.g. --help)
// ToDo: handle errors
// ToDo: handle the case when the process was terminated by a signal - hence no exit code
// ToDo: add missing upstream with https://github.com/whoever/whatever.git
// ToDo: is it the best choice to use `.output()` on commands?
// ToDo: tests?
// ToDo: a more reasonable exit code
// ToDo: add pre-commit hooks
// ToDo: read https://doc.rust-lang.org/std/process/struct.Command.html in details
// ToDo: add `describe` command
// ToDo: setup CI (build new versions, test, lint)
// ToDo: installation instruction
// ToDo: extract the duplicate logic into a separate function
fn main() {
    let is_reset = std::env::args().any(|x| x == "--reset");

    fetch();
    checkout();

    if is_reset {
        reset();
        force_push();
    } else {
        rebase();
    }

    println!("... done");
}

fn fetch() {
    println!("... fetching upstream");
    let output = Command::new("git")
        .args(&["fetch", "upstream"])
        .output()
        .expect("failed to run fetch");
    assert_failure(output);
}

fn checkout() {
    println!("... checkouting master");
    let output = Command::new("git")
        .args(&["checkout", "master"])
        .output()
        .expect("failed to run checkout");
    assert_failure(output);
}

fn rebase() {
    println!("... rebasing on upstream/master");
    let output = Command::new("git")
        .args(&["rebase", "upstream/master"])
        .output()
        .expect("failed to run rebase");
    assert_failure(output);
}

fn reset() {
    println!("... hard resetting on upstream/master");
    let output = Command::new("git")
        .args(&["reset", "--hard", "upstream/master"])
        .output()
        .expect("failed to run reset");
    assert_failure(output);
}

fn force_push() {
    println!("... force pushing to origin master");
    let output = Command::new("git")
        .args(&["push", "origin", "master", "--force"])
        .output()
        .expect("failed to run push");
    assert_failure(output);
}

fn assert_failure(output: Output) {
    if output.status.code().unwrap() != 0 {
        io::stderr().write_all(&output.stderr).unwrap();
        exit(1);
    }
}
