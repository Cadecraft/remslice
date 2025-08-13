use crate::remdata;
use crate::config::Config;
use crate::command;
use crate::command_lists;
use crate::utils;
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
        match utils::read_file(&utils::get_config_path()) {
            Some(contents) => {
                for line in contents.lines() {
                    if line.trim().is_empty() || line.trim().chars().nth(0).unwrap() == '#' {
                        // Empty line or comment
                        continue;
                    }
                    let command_res = command::run_command(&line, &mut res, command_lists::get_config_commands());
                    // TODO: handle errors here?
                }
            },
            _ => {
                // Failed to load file: log somewhere?
                // TODO: log somewhere? Handle/report any errors?
            }
        }
        res
    }
}
