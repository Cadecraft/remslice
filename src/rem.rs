use crate::remdata;
use crate::utils;
use crate::config::Config;

/// The data and methods for Rem
pub struct Rem {
    rem_data: remdata::RemData,
    ping_count: u32,
    to_copy_val: String,
    file_loaded: String,
    config: Config
}

impl Rem {
    /// Create a new Rem
    pub fn new(rem_data: remdata::RemData) -> Rem {
        Rem {
            rem_data,
            ping_count: 0,
            to_copy_val: "[empty]".to_string(),
            file_loaded: String::new(),
            config: Config::new()
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
            "wipe" => {
                // Wipe
                self.run_wipe_screen();
            },
            "cd" if parsed.len() == 1 => {
                // Print the current working directory
                println!("{}", utils::get_current_working_dir());
            },
            "tip" | "b" if parsed.len() >= 3 => {
                // Tip and grep
                self.run_tip(parsed[1].clone(), Some(parsed[2].clone()));
            },
            "tip" | "b" if parsed.len() == 2 => {
                // Tip
                self.run_tip(parsed[1].clone(), None);
            },
            "grep" if parsed.len() >= 2 => {
                // Grep
                self.run_grep(parsed[1].clone());
            },
            "print" => {
                // Print the file
                println!("{}", self.file_loaded);
            },
            "copy" => {
                // Try to copy whatever is in the copy val
                utils::copy_to_clipboard(&self.to_copy_val);
            },
            "exit" | "quit" | "q" => {
                // Exit immediately
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

    /// Run action: score calculation
    fn run_score(&mut self) {
        // TODO: impl
        // TODO: different scoring systems?
        // Defaults
        let categories_positive: Vec<&str> = vec![
            "% exercise completed",
            "% Path agenda completed",
            "Quality LeetCode",
            "% posture",
            "# good conv / 3.0",
            "Qualitative eval"
        ];
        let categories_negative: Vec<&str> = vec![
            "Hrs YT/news/reels scrolling",
            "Hrs unproductive Discord",
            "# ..."
        ];
        let divide_by: f32 = 5.0;
        let formula_number: &str = "1";
        // Obtain relevant information
        println!("Today's questions:");
        let mut daily_score_disp = format!("Daily Score (Formula {}) = (", formula_number);
        let mut total_score: f32 = 0.0;
        for cat in categories_positive {
            println!("{}", cat);
            let uin = utils::get_user_input_decimal(0.0, 1.0);
            total_score += uin;
            daily_score_disp.push_str(&format!(" + {:.2}", uin));
        }
        for cat in categories_negative {
            println!("{}", cat);
            let uin = utils::get_user_input_decimal(0.0, 1.0);
            total_score -= uin;
            daily_score_disp.push_str(&format!(" - {:.2}", uin));
        }
        total_score /= divide_by;
        daily_score_disp.push_str(&format!(") / {} = {:.2}", divide_by, total_score));
        self.to_copy_val = daily_score_disp.clone();
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

    /// Run action: tip
    fn run_tip(&mut self, key: String, grepval: Option<String>) {
        // Search for the given file and display it, so a tip can be found
        match self.config.get_tip_value(&key) {
            Some(tip_value) => {
                // Open and load the file, if possible
                match utils::read_file(&tip_value) {
                    Some(thecontents) => {
                        // Load the file
                        self.file_loaded = thecontents.clone();
                        println!("The file at {} is loaded into the buffer.", tip_value);
                        match grepval {
                            // Automatically grep
                            Some(query) => {
                                self.run_grep(query);
                            },
                            _ => {
                                println!("Consider using `grep` or `print`");
                            }
                        }
                    },
                    _ => {
                        println!("The file pointed to doesn't exist");
                    }
                }
            },
            _ => {
                // Failed
                println!("The tip doesn't exist");
            }
        }
    }

    /// Run action: grep
    fn run_grep(&mut self, query: String) {
        // Search the file for lines including it
        let mut success: bool = false;
        println!("Searching...");
        for (i, line) in self.file_loaded.lines().enumerate() {
            // Match?
            if line.to_lowercase().find(&query).is_some() {
                // Found
                println!("{:5} {}", i, line);
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
