// Utilities for remslice

use std::io::{stdin, stdout, Write};
use std::env;
use std::fs;
use std::env::consts::OS;
use std::process::Command;

use cli_clipboard;
use chrono;
use home;

// Keep the line ending '\n' for consistency in editing files
#[allow(dead_code)]
const LINE_ENDING: &'static str = "\n";

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

/// Get a string from the clipboard
pub fn paste_from_clipboard() -> Option<String> {
    match cli_clipboard::get_contents() {
        Ok(contents) => {
            // Success
            Some(contents)
        },
        Err(_err) => {
            // Failure
            // TODO: handle?
            None
        }
    }
}

/// Get the contents of a file given its path, if possible
pub fn read_file(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(thepath) => {
            Some(thepath)
        },
        _ => {
            None
        }
    }
}

/// Append to a file given its path, if possible, and return whether successful
pub fn append_to_file(path: &str, to_write: &str) -> bool {
    let file =  fs::OpenOptions::new().write(true).append(true).open(path);
    match file {
        Ok(mut fileval) => {
            // Append the new line
            if let Err(_e) = writeln!(fileval, "{}", to_write) {
                // TODO: handle error better
                false
            } else {
                true
            }
        },
        _ => {
            // Failure
            false
        }
    }
}

/// Edit the last line in a file given its path, if possible, and return whether successful
/// If append is set, then the new last line is added onto the old last line; otherwise, it replaces it
pub fn edit_last_line_of_file(path: &str, new_last_line: &str, append: bool) -> bool {
    // Get the contents
    match read_file(path) {
        Some(contents) => {
            // Create the new contents
            let lines = contents.lines().collect::<Vec<&str>>();
            if lines.is_empty() {
                return false;
            }
            let mut newcontents = String::new();
            for (i, line) in lines.iter().enumerate() {
                if i != lines.len() - 1 {
                    newcontents.push_str(&format!("{}\n", line));
                }
            }
            // Add the last line
            if append {
                newcontents.push_str(&format!("{}{}\n", lines.last().unwrap_or(&""), new_last_line));
            } else {
                newcontents.push_str(&format!("{}\n", new_last_line));
            }
            // Write the new contents
            write_to_file(path, &newcontents)
        },
        _ => {
            false
        }
    }
}

/// Write to a file given its path, if possible, and return whether successful
pub fn write_to_file(path: &str, to_write: &str) -> bool {
    match fs::write(path, to_write) {
        Ok(_theres) => {
            true
        },
        _ => {
            false
        }
    }
}

/// Get the current local time
fn get_time() -> chrono::DateTime<chrono::Local> {
    chrono::Local::now()
}

/// Get the current local time, formatted
pub fn get_time_formatted() -> String {
    let thetime = get_time();
    thetime.format("%Y/%m/%d %H:%M").to_string()
}

/// Get the current date only, formatted
pub fn get_date_only_formatted() -> String {
    let thetime = get_time();
    thetime.format("%Y/%m/%d").to_string()
}

/// Get the current operating system
pub fn get_os() -> String {
    OS.to_string()
}

/// Run a command, print an informational message, and return whether successful
pub fn run_command(command: &str) -> bool {
    match OS {
        "windows" => {
            // Use powershell
            match Command::new("powershell").args(["-c", command]).spawn() {
                Ok(_theres) => {
                    println!("Command executed via powershell");
                    true
                },
                _ => {
                    println!("Failed to execute command via powershell");
                    false
                }
            }
        },
        _ => {
            // Use sh
            match Command::new("sh").args(["-c", command]).spawn() {
                Ok(_theres) => {
                    println!("Command executed via sh");
                    true
                },
                _ => {
                    println!("Failed to execute command via sh");
                    false
                }
            }
        }
    }
}

/// Get the current config directory (for the .remrc file)
pub fn get_config_path() -> String {
    // Return the home dir, plus .remrc
    match home::home_dir() {
        Some(mut path) if !path.as_os_str().is_empty() => {
            path.push(".remrc");
            path.into_os_string().into_string().unwrap()
        },
        _ => {
            String::new()
        }
    }
}

/// Generate a new ID based on the prior one ("a" -> "zzz")
pub fn generate_next_id(currid: String) -> String {
    // Either increment final character or add a new one
    let mut chars = currid.chars().collect::<Vec<char>>();
    let mut carry = true;
    for i in (0..chars.len()).rev() {
        // Increase?
        if carry {
            if chars[i] == 'z' {
                carry = true;
                chars[i] = 'a';
            } else {
                // Increase this one
                chars[i] = std::char::from_u32(chars[i] as u32 + 1).unwrap_or('a');
                carry = false;
            }
        }
    }
    if carry {
        chars.insert(0, 'a');
    }
    let mut res = String::new();
    for c in chars {
        res.push(c);
    }
    res
}

/// Get the trailing nth section of a line of input
/// (everything after the (n - 1)th word of the argument)
/// Ex. if n = 2, get everything after the first word
pub fn trailing_portion_of_input(line: &str, n: i32) -> String {
    // After the (n - 1)th word, get everything else
    let mut space_gaps: i32 = 0;
    let mut res = String::new();
    for c in line.trim().chars() {
        if space_gaps >= (n - 1) {
            res.push(c);
        } else if c == ' ' {
            space_gaps += 1;
        }
    }
    res
}

/// Strikethrough a line of text in markdown
/// Either clear (strikethrough) or unclear (remove strikethrough), depending on whether already cleared
pub fn strikethrough_text(target: &str) -> String {
    let mut res = String::new();
    if target.find('~').is_some() {
        // Unclear (remove strikethrough)
        for c in target.chars() {
            if c != '~' {
                res.push(c);
            }
        }
    } else {
        // Clear (strikethrough)
        // Contents should look like: "- the contents" -> "- ~~the contents~~"
        let mut chars_after_first_dash = -1;
        for c in target.chars() {
            if chars_after_first_dash == 2 {
                res.push_str("~~");
            }
            if c == '-' {
                if chars_after_first_dash == -1 {
                    chars_after_first_dash = 0;
                }
            }
            if chars_after_first_dash != -1 {
                chars_after_first_dash += 1;
            }
            res.push(c);
        }
        if chars_after_first_dash == -1 {
            // No dash was found
            res.insert_str(0, "~~");
        }
        res.push_str("~~");
    }
    res
}
