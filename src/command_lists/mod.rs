mod rem_commands;
mod config_commands;
use crate::command;

pub fn get_rem_commands() -> &'static Vec<command::Command> {
    &rem_commands::REM_COMMANDS
}

pub fn get_config_commands() -> &'static Vec<command::Command> {
    &config_commands::CONFIG_COMMANDS
}
