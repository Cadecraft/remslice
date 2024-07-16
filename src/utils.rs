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
