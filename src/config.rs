/// Stores a tip key/value pair
struct Pair {
    key: String,
    value: String
}

#[derive(Clone)]
pub struct ShellAlias {
    pub key: String,
    pub command: String,
    pub quit_after_running: bool
}

/// Stores a rem config based on the remrc file
pub struct Config {
    tips: Vec<Pair>,
    shell_aliases: Vec<ShellAlias>,
    rem_aliases: Vec<Pair>,
    pub todo_path: String,
    score_positive: Vec<String>,
    score_negative: Vec<String>,
    pub score_divby: f32,
    pub score_formula_number: String,
    pub ted_command_prefix: String,
    pub ted_command_suffix: String,
}

impl Config {
    /// Make a new Config (user config will be loaded when commands are executed by remstate)
    pub fn new() -> Config {
        Config {
            tips: Vec::new(),
            shell_aliases: Vec::new(),
            rem_aliases: Vec::new(),
            todo_path: "default_todos.md".to_string(),
            score_positive: Vec::new(),
            score_negative: Vec::new(),
            score_divby: 5.0,
            score_formula_number: "1".to_string(),
            ted_command_prefix: "gvim + ".to_string(),
            ted_command_suffix: "".to_string()
        }
    }

    pub fn add_tip(&mut self, key: &str, value: &str) {
        self.tips.push(Pair {
            key: key.to_string(),
            value: value.to_string()
        });
    }

    pub fn add_shell_alias(&mut self, key: &str, command: &str, quit_after_running: bool) {
        self.shell_aliases.push(ShellAlias {
            key: key.to_string(),
            command: command.to_string(),
            quit_after_running
        });
    }

    pub fn add_rem_alias(&mut self, key: &str, value: &str) {
        self.rem_aliases.push(Pair {
            key: key.to_string(),
            value: value.to_string()
        });
    }

    pub fn add_score_factor(&mut self, descr: String, positive: bool) {
        if positive {
            self.score_positive.push(descr);
        } else {
            self.score_negative.push(descr);
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

    /// Get the information about a shell alias matching a key
    pub fn get_shell_alias(&self, search_for: &str) -> Option<ShellAlias> {
        for alias in &self.shell_aliases {
            if alias.key == search_for {
                return Some(alias.clone());
            }
        }
        return None;
    }

    /// Get the value of a rem alias matching a key
    pub fn get_rem_alias_value(&self, search_for: &str) -> Option<String> {
        for alias in &self.rem_aliases {
            if alias.key == search_for {
                return Some(alias.value.clone());
            }
        }
        return None;
    }

    /// Display all shell aliases
    pub fn display_shell_aliases(&self) -> String {
        let mut res = String::new();
        for alias in &self.shell_aliases {
            res.push_str(&format!(
                "   {}{} : {}\n",
                alias.key,
                if alias.quit_after_running { " (Q)" } else { "" },
                alias.command
            ));
        }
        return res;
    }

    /// Display all rem aliases
    pub fn display_rem_aliases(&self) -> String {
        let mut res = String::new();
        for alias in &self.rem_aliases {
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
}
