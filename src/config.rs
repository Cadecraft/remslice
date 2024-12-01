use crate::utils;

/// Stores a tip key/value pair
struct TipPair {
    key: String,
    value: String
}

/// Stores a rem config based on the remrc file
pub struct Config {
    remrc_path: String,
    tips: Vec<TipPair>,
    shell_aliases: Vec<TipPair>,
    todo_path: String,
    score_positive: Vec<String>,
    score_negative: Vec<String>,
    score_divby: f32,
    score_formula_number: String
}

impl Config {
    /// Make a new Config
    pub fn new() -> Config {
        let mut c = Config {
            remrc_path: utils::get_config_path(),
            tips: Vec::new(),
            shell_aliases: Vec::new(),
            todo_path: "default_todos.md".to_string(),
            score_positive: Vec::new(),
            score_negative: Vec::new(),
            score_divby: 5.0,
            score_formula_number: "1".to_string()
        };
        c.load();
        c
    }

    /// Get the todo path
    pub fn get_todo_path(&self) -> String {
        self.todo_path.clone()
    }

    /// Load the config from the remrc
    pub fn load(&mut self) -> bool {
        // Read the file
        match utils::read_file(&self.remrc_path) {
            Some(contents) => {
                // Parse contents
                for line in contents.lines() {
                    if line.trim().is_empty() || line.trim().chars().nth(0).unwrap() == '#' {
                        // Empty line or comment
                        continue;
                    }
                    // Parse this line
                    let parsed: Vec<&str> = line.trim().split(" ").collect::<Vec<&str>>();
                    match parsed[0].trim() {
                        "tip" if parsed.len() >= 3 => {
                            // Add a tip
                            let mut spacegaps = 0;
                            let mut userpath = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 2 {
                                    userpath.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            self.tips.push(TipPair {
                                key: parsed[1].trim().to_string(),
                                value: userpath
                            });
                        },
                        "shell_alias" if parsed.len() >= 3 => {
                            // Add an alias
                            let mut spacegaps = 0;
                            let mut usercommand = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 2 {
                                    usercommand.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            self.shell_aliases.push(TipPair {
                                key: parsed[1].trim().to_string(),
                                value: usercommand
                            });
                        },
                        "todo" if parsed.len() >= 2 => {
                            // Set the todo path
                            let mut spacegaps = 0;
                            let mut userpath = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 1 {
                                    userpath.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            self.todo_path = parsed[1].trim().to_string()
                        },
                        "score_p" if parsed.len() >= 2 => {
                            // Add a positive score category
                            let mut spacegaps = 0;
                            let mut userstring = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 1 {
                                    userstring.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            self.score_positive.push(userstring);
                        },
                        "score_n" if parsed.len() >= 2 => {
                            // Add a negative score category
                            let mut spacegaps = 0;
                            let mut userstring = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 1 {
                                    userstring.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            self.score_negative.push(userstring);
                        },
                        "score_divby" if parsed.len() >= 2 => {
                            // Set the score division by
                            let mut spacegaps = 0;
                            let mut userdivby = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 1 {
                                    userdivby.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            match userdivby.parse::<f32>() {
                                Ok(res) => {
                                    self.score_divby = res;
                                },
                                _ => {
                                    // Error
                                }
                            };
                        },
                        "score_formula_number" if parsed.len() >= 2 => {
                            // Set the score division by
                            let mut spacegaps = 0;
                            let mut usernum = String::new();
                            for c in line.trim().chars() {
                                if spacegaps >= 1 {
                                    usernum.push(c);
                                } else if c == ' ' {
                                    spacegaps += 1;
                                }
                            }
                            self.score_formula_number = usernum;
                        },
                        _ => {
                            // None
                        }
                    }
                }
                // Success
                return true;
            },
            _ => {
                // Failed
                return false;
            }
        }
    }

    /// Get the value of a tip starting with a key
    pub fn get_tip_value(&self, start: &str) -> Option<String> {
        for tip in &self.tips {
            if tip.key.starts_with(start) {
                return Some(tip.value.clone());
            }
        }
        return None;
    }

    /// Display all tips
    pub fn display_tips(&self) -> String {
        let mut res = String::new();
        for tip in &self.tips {
            res.push_str(&format!("   {} : {}\n", tip.key, tip.value));
        }
        return res;
    }

    /// Get the value of an alias matching a key
    pub fn get_shell_alias_value(&self, search_for: &str) -> Option<String> {
        for alias in &self.shell_aliases {
            if alias.key == search_for {
                return Some(alias.value.clone());
            }
        }
        return None;
    }

    /// Display all aliases
    pub fn display_shell_aliases(&self) -> String {
        let mut res = String::new();
        for alias in &self.shell_aliases {
            res.push_str(&format!("   {} : {}\n", alias.key, alias.value));
        }
        return res;
    }

    /// Get all positive score categories
    pub fn score_positive(&self) -> Vec<String> {
        self.score_positive.clone()
    }

    /// Get all negative score categories
    pub fn score_negative(&self) -> Vec<String> {
        self.score_negative.clone()
    }

    /// Get the score value to divide by
    pub fn score_divby(&self) -> f32 {
        self.score_divby
    }

    /// Get the score formula number/name
    pub fn score_formula_number(&self) -> String {
        self.score_formula_number.clone()
    }
}
