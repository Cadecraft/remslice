use crate::remdata;
use crate::config::Config;
use std::collections::hash_map::HashMap;

pub struct RemState {
    pub rem_data: remdata::RemData,
    pub ping_count: u32,
    pub to_copy_val: String,
    pub file_loaded: String,
    /// Store the ID (string of lowercase letters) and corresponding line NUMBER (not index)
    pub todos_ids: HashMap<String, usize>,
    pub config: Config
}
