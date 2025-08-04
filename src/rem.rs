use crate::remdata;
use crate::utils;
use crate::config::Config;
use crate::remfetch;
use crate::command;
use crate::remstate;
use std::collections::hash_map::HashMap;

/// The data and methods for Rem
pub struct Rem {
    state: remstate::RemState
}

impl Rem {
    /// Create a new Rem
    pub fn new(rem_data: remdata::RemData) -> Rem {
        Rem {
            state: remstate::RemState {
                rem_data,
                ping_count: 0,
                to_copy_val: "[empty]".to_string(),
                file_loaded: String::new(),
                todos_ids: HashMap::new(),
                config: Config::new()
            }
        }
    }

    /// Respond to a raw user-inputted string and return whether the program should quit
    pub fn respond_to_input(&mut self, input: String, recursion_level: i32) -> Option<command::CommandResult> {
        // Ensure we aren't in an infinite loop
        const MAX_RECURSION_LEVEL: i32 = 100;
        if recursion_level > MAX_RECURSION_LEVEL {
            println!("Infinitely recursive command encountered (recursed over {MAX_RECURSION_LEVEL} times)");
            return None
        }
        // TODO: refactor all into run_command
        let res = command::run_command(&input, &mut self.state);
        if res.is_some() {
            return res;
        }
        // Couldn't run the command verbatim, so check rem aliases
        // TODO: refactor this?
        let parsed: Vec<String> = Self::parse_input(&input);
        let first_arg = Self::argument_at_index(&parsed, 0);
        match self.state.config.get_rem_alias_value(first_arg) {
            Some(val) => {
                self.run_rem_alias(&val, recursion_level + 1)
            }
            _ => {
                println!("?");
                None
            }
        }

        /*match first_arg {
            "score" => {
                // Score
                self.run_score();
            },
            "version" | "ver" => {
                // Simple version information
                println!("REMSLICE ({})", self.rem_data.to_string())
            },
            "remfetch" => {
                // Remfetch
                println!("{}", remfetch::remfetch(&self.rem_data));
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
            "help" => {
                // Help
                self.run_help();
            },
            "wipe" => {
                // Wipe
                self.run_wipe_screen();
            },
            "pwd" if parsed.len() == 1 => {
                // Print the current working directory
                println!("{}", utils::get_current_working_dir());
            },
            "tip" | "b" if parsed.len() >= 3 => {
                // Tip and grep
                self.run_tip(parsed[1].clone(), Some(utils::trailing_portion_of_input(&input, 3).to_lowercase()));
            },
            "tip" | "b" if parsed.len() == 2 => {
                // Tip
                self.run_tip(parsed[1].clone(), None);
            },
            "tip-ls" => {
                // List all tips and their directories
                self.run_tip_ls();
            },
            "grep" if parsed.len() >= 2 => {
                // Grep (case-insensitive)
                self.run_grep(utils::trailing_portion_of_input(&input, 2).to_lowercase());
            },
            "line" if parsed.len() >= 2 => {
                // Print the line number
                self.run_line(utils::trailing_portion_of_input(&input, 2).to_lowercase());
            },
            "tda" if parsed.len() >= 2 => {
                // Add a todo
                self.run_tda(utils::trailing_portion_of_input(&input, 2));
            },
            "tdt" => {
                // Display the top of the todo list (1 level)
                self.run_tdt(1);
            },
            "tdt2" => {
                // Display the top of the todo list (2 levels)
                self.run_tdt(2);
            },
            "tdc" if parsed.len() == 2 => {
                // Clear a todo based on its ID
                self.run_tdc(parsed[1].clone());
            },
            "tde" if parsed.len() >= 2 => {
                // Edit the latest todo
                self.run_tde(utils::trailing_portion_of_input(&input, 2));
            },
            "tdae" if parsed.len() >= 2 => {
                // Append-edit the latest todo
                self.run_tdae(utils::trailing_portion_of_input(&input, 2));
            },
            "tdn" => {
                // Add a new day to the todo log
                self.run_tdn();
            },
            "al" => {
                // Run the command represented by an alias
                if self.run_al(parsed[1].clone()) {
                    // Should quit
                    return true;
                }
            },
            "al-ls" => {
                // List all aliases and their commands
                self.run_al_ls();
            }
            "print" => {
                // Print the file
                self.run_print();
            },
            "copy" | "y" => {
                // Try to copy whatever is in the copy val
            },
            "paste" | "p" => {
                // Try to paste whatever is in the clipboard val
            },
            "pasterun!" | "pr!" => {
                // DEPRECATED
                // Try to run whatever is in the clipboard
                match utils::paste_from_clipboard() {
                    Some(contents) => {
                        println!("Running: `{}`", contents);
                        if self.respond_to_input(contents, recursion_level + 1) {
                            // Should quit
                            return true;
                        }
                    },
                    _ => {
                        println!("Couldn't paste the clipboard contents");
                    }
                }
            },
            "exit" | "quit" | "q" => {
                // Exit immediately
                // TODO: allow other return types (enum for this function's return)
                return true;
            },
            "time" => {
                // Get the current time
                let output = utils::get_time_formatted();
                self.state.to_copy_val = output.clone();
                println!("{}", output);
            },
            _ => {
                // No match out of existing commands
                // Check all rem aliases
                match self.state.config.get_rem_alias_value(first_arg) {
                    Some(val) => {
                        // Execute the current rem alias
                        if self.run_rem_alias(&val, recursion_level + 1) {
                            // Should quit
                            return true;
                        }
                    }
                    _ => {
                        println!("?");
                    }
                }
            }
        }
        false*/
    }

    /// Run action: help
    fn run_help(&mut self) {
        println!("A detailed list of all commands can be found in `README.md`;");
        println!("please check it out for the features and cool stuff!");
        println!("- `exit`/`quit`/`q` - exit immediately");
        println!("- `version`/`ver` - display simple version information");
    }

    /// Run action: score calculation
    fn run_score(&mut self) {
        // Get based on config
        let divide_by: f32 = self.state.config.score_divby();
        let formula_number: &str = &self.state.config.score_formula_number();
        // Obtain relevant information
        println!("Today's questions:");
        let mut daily_score_disp = format!("Daily Score (Formula {}) = (", formula_number);
        let mut total_score: f32 = 0.0;
        for cat in self.state.config.score_positive() {
            println!("{}", cat);
            let uin = utils::get_user_input_decimal(0.0, 1.0);
            total_score += uin;
            daily_score_disp.push_str(&format!(" + {:.2}", uin));
        }
        for cat in self.state.config.score_negative() {
            println!("{}", cat);
            let uin = utils::get_user_input_decimal(0.0, 1.0);
            total_score -= uin;
            daily_score_disp.push_str(&format!(" - {:.2}", uin));
        }
        // Calculate and format
        total_score /= divide_by;
        daily_score_disp.push_str(&format!(") / {} = {:.2}", divide_by, total_score));
        self.state.to_copy_val = daily_score_disp.clone();
        // Create the score report
        println!("Today's daily score:");
        println!("{}", daily_score_disp);
        // Options: copy, continue, restart, edit
        println!("To copy the report, enter `copy`");
    }

    /// Run action: wipe the screen
    fn run_wipe_screen(&self) {
        // Print enough times that the screen gets filled
        for _i in 0..100 {
            println!();
        }
        println!("The screen is clear!");
    }

    /// Run action: grep
    fn run_grep(&mut self, query: String) {
        // Search the file for lines including it
        let mut success: bool = false;
        println!("Searching...");
        for (i, line) in self.state.file_loaded.lines().enumerate() {
            // Match?
            if line.to_lowercase().find(&query).is_some() {
                // Found
                println!("   {:5} {}", i + 1, line);
                success = true;
            }
        }
        if !success {
            println!("I found no results in the file.");
        }
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
    fn parse_input(input: &str) -> Vec<String> {
        // Split by spaces, strip, remove unnecessary characters
        // TODO: impl myself to ignore spaces and keep case within quotes, handle backslashes, etc.
        let splitted: Vec<&str> = input.split(' ').collect::<Vec<&str>>();
        let mut res: Vec<String> = Vec::new();
        for arg in splitted {
            res.push(arg.trim().to_lowercase().to_string());
        }
        res
    }

    /// Run a rem alias, returning whether to quit
    fn run_rem_alias(&mut self, alias: &str, recursion_level: i32) -> Option<command::CommandResult> {
        self.respond_to_input(alias.to_string(), recursion_level + 1)
    }
}
