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
    todo_path: String
}

impl Config {
    /// Make a new Config
    pub fn new() -> Config {
        let mut c = Config {
            remrc_path: utils::get_config_path(),
            tips: Vec::new(),
            todo_path: "default_todos.md".to_string()
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
                            // TODO: test file paths with spaces
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
                        "todo" if parsed.len() >= 2 => {
                            // Set the todo path
                            // TODO: test file paths with spaces
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
        // TODO: ensure spaces are preserved in the path arguments (since they're the last argument, it should work out)
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
}
