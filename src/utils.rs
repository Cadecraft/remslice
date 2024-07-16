// Utilities for remslice

use std::io::{stdin, stdout, Write};

/// Get the user's input
pub fn get_user_input_line() -> String {
    print!("> ");
    stdout().flush().unwrap();
    let mut uin = String::new();
    stdin().read_line(&mut uin).expect("ERR: failed to read line");
    uin.trim().to_string()
}

// TODO: printing with colors (crossterm?)

/// Await for the user's enter press
pub fn await_enter() {
    print!("> [enter]");
    stdout().flush().unwrap();
    let mut uin = String::new();
    stdin().read_line(&mut uin).expect("ERR: failed to read line");
}
