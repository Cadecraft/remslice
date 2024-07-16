// Utilities for remslice

use std::io::{stdin, stdout, Write};
use std::env;

use cli_clipboard;

/// Get the user's input
pub fn get_user_input_line() -> String {
    print!("> ");
    stdout().flush().unwrap();
    let mut uin = String::new();
    stdin().read_line(&mut uin).expect("ERR: failed to read line");
    uin.trim().to_string()
}

/// Get a user's inputted decimal number
pub fn get_user_input_decimal(num_min: f32, num_max: f32) -> f32 {
    loop {
        let uin = get_user_input_line();
        let parsed = uin.parse::<f32>();
        match parsed {
            Ok(res) => {
                if res >= num_min && res <= num_max {
                    return res;
                } else {
                    println!("Please enter a valid number from {}..={}", num_min, num_max);
                }
            },
            _ => {
                println!("Please enter a valid number");
            }
        }
    }
}

// TODO: printing with colors (crossterm?)

/// Await for the user's enter press
pub fn await_enter() {
    print!("> [enter]");
    stdout().flush().unwrap();
    let mut uin = String::new();
    stdin().read_line(&mut uin).expect("ERR: failed to read line");
}

/// Get the current working directory
pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => {
            path.into_os_string().into_string().unwrap()
        },
        _ => {
            "FAILED".to_string()
        }
    }
}

/// Copy a string to the clipboard
pub fn copy_to_clipboard(s: &str) -> bool {
    match cli_clipboard::set_contents(s.to_owned()) {
        Ok(_) => {
            // Success
            true
        },
        Err(_err) => {
            // Failure
            // TODO: handle?
            false
        }
    }
}

