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
                // Execute the current rem alias
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
                utils::copy_to_clipboard(&self.state.to_copy_val);
                if self.state.to_copy_val.chars().count() > 6 {
                    println!("Yanked string starting with '{}'", &self.state.to_copy_val[..4]);
                } else {
                    println!("Yanked string '{}'", self.state.to_copy_val);
                }
            },
            "paste" | "p" => {
                // Try to paste whatever is in the clipboard val
                match utils::paste_from_clipboard() {
                    Some(contents) => {
                        println!("{}", contents);
                    },
                    _ => {
                        println!("Couldn't paste the clipboard contents");
                    }
                }
            },
            "pasterun!" | "pr!" => {
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

    /// Run action: alias (return whether should quit)
    fn run_al(&mut self, alias: String) -> bool {
        match self.state.config.get_shell_alias(&alias) {
            Some(alias) => {
                // Run the alias if possible, then quit if successful and desired
                let res = utils::run_command(&alias.command);
                if res && alias.quit_after_running {
                    return true;
                }
            },
            _ => {
                println!("The shell alias doesn't exist");
            }
        }
        // TODO: better indicator of "quit" or "not quit" (use an enum?)
        false
    }

    /// Run action: alias list
    fn run_al_ls(&self) {
        // Display all aliases
        println!("All shell aliases added:");
        println!("{}", self.state.config.display_shell_aliases());
        println!("All rem aliases added:");
        println!("{}", self.state.config.display_rem_aliases());
    }

    /// Run action: print
    fn run_print(&mut self) {
        // Print
        for (i, line) in self.state.file_loaded.lines().enumerate() {
            println!("   {:5} {}", i + 1, line);
        }
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

    /// Run action: line
    fn run_line(&mut self, query: String) {
        // The line number in the query
        match query.parse::<usize>() {
            Ok(linenum) => {
                if linenum < 1 || linenum > self.state.file_loaded.lines().count() {
                    println!("Enter a line number from 1 to {}", self.state.file_loaded.lines().count());
                    return;
                }
                // Print the line
                println!("   {:5} {}", linenum, self.state.file_loaded.lines().collect::<Vec<&str>>()[linenum - 1]);
            },
            _ => {
                println!("Enter a line number from 1 to {}", self.state.file_loaded.lines().count());
                return;
            }
        }
    }

    /// Run action: todo add
    fn run_tda(&mut self, s: String) {
        // Append to the end of todos
        if utils::append_to_file(&self.state.config.get_todo_path(), &format!("- {}", s)) {
            println!("Todo added successfully");
        } else {
            println!("Todo could not be added");
        }
    }

    /// Run action: todo top (up until the given number of headers, default 1)
    fn run_tdt(&mut self, count: u32) {
        // Get the end of todos
        match utils::read_file(&self.state.config.get_todo_path()) {
            Some(contents) => {
                // Print the end of the file up until the first hash symbol
                let mut res = String::new();
                let lines = contents.lines().collect::<Vec<&str>>();
                let mut headers_seen = 0;
                self.state.todos_ids.clear();
                let mut currid = "a".to_string();
                for i in (0..lines.len()).rev() {
                    let mut final_line = false;
                    if lines[i].starts_with("##") {
                        headers_seen += 1;
                        if headers_seen >= count {
                            final_line = true;
                        }
                    }
                    // Track this line's ID
                    self.state.todos_ids.insert(currid.clone(), i + 1);
                    // Line goes above res (because iterating in reverse)
                    res = format!("{:3}{:5} {}\n{}", currid, i + 1, lines[i], res);
                    if final_line {
                        break;
                    }
                    currid = utils::generate_next_id(currid.clone());
                }
                println!("{}", res);
            },
            _ => {
                println!("Todo file could not be accessed");
            }
        }
    }

    /// Clear a todo based on its ID
    fn run_tdc(&self, id: String) {
        let linenum: usize = match self.state.todos_ids.get(&id) {
            Some(l) => {
                *l
            },
            _ => {
                println!("ID does not exist");
                return;
            }
        };
        // Clear the todo
        match utils::read_file(&self.state.config.get_todo_path()) {
            Some(contents) => {
                let mut lines = contents.lines().collect::<Vec<&str>>();
                // Check bounds
                if linenum < 1 || linenum > lines.len() {
                    println!("Line number pointed to is out of bounds");
                    return;
                }
                let target: String = lines[linenum - 1].to_string();
                let res: String = utils::strikethrough_text(&target);
                // Print successful result
                println!("   {:5} {}", linenum, res);
                // Update the contents lines
                lines[linenum - 1] = &res;
                let mut newcontents = String::new();
                for line in lines {
                    newcontents.push_str(&format!("{}\n", line));
                }
                // Overwrite the file with the new contents
                utils::write_to_file(&self.state.config.get_todo_path(), &newcontents);
            },
            _ => {
                println!("Todo file could not be accessed");
                return;
            }
        }
    }

    /// Run action: todo edit
    fn run_tde(&self, s: String) {
        if utils::edit_last_line_of_file(&self.state.config.get_todo_path(), &format!("- {}", s), false) {
            println!("- {}", s);
        } else {
            println!("Topmost todo could not be edited");
        }
    }

    /// Run action: todo append edit
    fn run_tdae(&self, s: String) {
        let formatted_to_append: String = match s.chars().next().unwrap_or(' ') {
            ',' | ';' | '-' | '.' => {
                // Punctuation, so include it
                s.clone()
            },
            _ => {
                format!(" {}", s)
            }
        };
        if utils::edit_last_line_of_file(&self.state.config.get_todo_path(), &formatted_to_append, true) {
            println!("Appended to the topmost todo");
        } else {
            println!("Topmost todo could not be edited");
        }
    }

    /// Run action: todo new day
    fn run_tdn(&mut self) {
        // Append the day to the end of todos
        if utils::append_to_file(&self.state.config.get_todo_path(), &format!("## {}", utils::get_date_only_formatted())) {
            println!("New day added successfully");
        } else {
            println!("Todo could not be added");
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
