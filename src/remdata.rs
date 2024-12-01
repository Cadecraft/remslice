/// Stores the REM data
pub struct RemData {
    recent_version: String,
    edit_date: String,
    morning: bool
}

impl RemData {
    /// Create a new RemData
    pub fn new(recent_version: &str, edit_date: &str, morning: bool) -> RemData {
        RemData {
            recent_version: recent_version.to_string(),
            edit_date: edit_date.to_string(),
            morning
        }
    }

    /// Represent the Data as a one-line string
    pub fn to_string(&self) -> String {
        format!(
            "R: v{}, E: {}, M: {}",
            self.recent_version,
            self.edit_date,
            if self.morning { "[success]" } else { "[failure]" }
        )
    }

    /// Clone
    pub fn clone(&self) -> RemData {
        RemData {
            recent_version: self.recent_version.clone(),
            edit_date: self.edit_date.clone(),
            morning: self.morning
        }
    }

    /// Get the recent verison
    pub fn get_r(&self) -> String {
        self.recent_version.clone()
    }

    /// Get the edit date
    pub fn get_e(&self) -> String {
        self.edit_date.clone()
    }

    /// Get the morning value
    pub fn get_m(&self) -> bool {
        self.morning
    }
}
