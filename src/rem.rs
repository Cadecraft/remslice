use crate::remdata;
use crate::utils;

/// The data and methods for Rem
pub struct Rem {
    rem_data: remdata::RemData,
    ping_count: u32
}

impl Rem {
    /// Create a new Rem
    pub fn new(rem_data: remdata::RemData) -> Rem {
        Rem {
            rem_data,
            ping_count: 0
        }
    }

    /// Respond to a raw user-inputted string and return whether the program should quit
    pub fn respond_to_input(&mut self, input: String) -> bool {
        // Parse
        let parsed: Vec<String> = Self::parse_input(input);
        // Respond based on the input
        match Self::argument_at_index(&parsed, 0) {
            "score" => {
                // Score
                self.run_score();
            },
            "version" | "ver" => {
                // Version
                // TODO: impl
                println!("REMSLICE ({})", self.rem_data.to_string())
            },
            "bye" => {
                // Message and quit
                println!("bye!");
                utils::await_enter();
                return true;
            },
            "ping" => {
                // Ping
                self.ping_count += 1;
                println!("pong! (x{})", self.ping_count);
            },
            "quit" | "exit" => {
                // Quit immediately
                // TODO: allow other return types (enum for this function's return)
                return true;
            },
            _ => {
                // No match
                // TODO: impl
                println!("?");
            }
        }
        false
    }

    /// Run a procedure: score calculation
    fn run_score(&mut self) {
        // TODO: impl
        // TODO: different scoring systems?
        // Obtain relevant information
        // Create the score report
        // Options: copy, continue, restart, edit
    }

    /// Get the argument at an index of the input
    fn argument_at_index(parsed: &Vec<String>, i: usize) -> &str {
        if i >= parsed.len() {
            ""
        } else {
            &parsed[i]
        }
    }

    /// Parse the input
    fn parse_input(input: String) -> Vec<String> {
        // Split by spaces, strip, remove unnecessary characters
        // TODO: impl myself to ignore spaces and keep case within quotes, handle backslashes, etc.
        let splitted: Vec<&str> = input.split(' ').collect::<Vec<&str>>();
        let mut res: Vec<String> = Vec::new();
        for arg in splitted {
            res.push(arg.trim().to_lowercase().to_string());
        }
        res
    }
}
