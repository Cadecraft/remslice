/// Stores the REM data
pub struct RemData {
    recent_version: String,
    edit_date: String,
    morning_star: bool
}

impl RemData {
    /// Create a new RemData
    pub fn new(recent_version: &str, edit_date: &str, morning_star: bool) -> RemData {
        RemData {
            recent_version: recent_version.to_string(),
            edit_date: edit_date.to_string(),
            morning_star
        }
    }

    /// Represent the Data as a one-line string
    pub fn to_string(&self) -> String {
        format!(
            "R: v{}, E: {}, M: {}",
            self.recent_version,
            self.edit_date,
            if self.morning_star { "[success]" } else { "[failure]" }
        )
    }

    /// Clone
    pub fn clone(&self) -> RemData {
        RemData {
            recent_version: self.recent_version.clone(),
            edit_date: self.edit_date.clone(),
            morning_star: self.morning_star
        }
    }
}
