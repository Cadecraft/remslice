/// Stores a tip key/value pair
struct TipPair {
    key: String,
    value: String
}

/// Stores a rem config based on the remrc file
pub struct Config {
    remrc_path: String,
    tips: Vec<TipPair>
}

impl Config {
    // TODO: allow user to define the remrc path
    /// Make a new Config
    pub fn new() -> Config {
        let mut c = Config {
            remrc_path: "C:/Cade/Scripts/Tools_CLI/remrc.txt".to_string(),
            tips: Vec::new()
        };
        c.load();
        c
    }

    /// Load the config from the remrc
    pub fn load(&mut self) {
        // TODO: read
        // TODO: load tips
        self.tips.push(TipPair {
            key: "vimtoremember".to_string(),
            value: "C:/Cade/PDFs/Utility/ToRememberDocs/VimToRemember.md".to_string()
        });
        self.tips.push(TipPair {
            key: "shortcutstoremember".to_string(),
            value: "C:/Cade/PDFs/Utility/ToRememberDocs/ShortcutsToRemember.md".to_string()
        });
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
