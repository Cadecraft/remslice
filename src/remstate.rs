use crate::remdata;
use crate::config::Config;
use crate::command;
use crate::command_lists;
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

impl RemState {
    /// Return a new RemState with the config loaded
    pub fn new(rem_data: remdata::RemData) -> RemState {
        let mut res = RemState {
            rem_data,
            ping_count: 0,
            to_copy_val: "[empty]".to_string(),
            file_loaded: String::new(),
            todos_ids: HashMap::new(),
            config: Config::new()
        };
        // TODO: get the input file
        // TODO: run for each line of input
        // TODO: handle/report any errors or problems loading the input
        command::run_command("tip test C:/Cade/Temp/testrem.txt", &mut res, command_lists::get_config_commands());
        res
    }
}
