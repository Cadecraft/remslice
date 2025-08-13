use crate::command::{ Command, ArgsLim, CommandResult };
use crate::utils;
use crate::remfetch;
use crate::feature;
use std::sync::LazyLock;

/*/// The result of a config command: whether it was successful or failed
pub enum ConfigCommandResult {
    Error(String),
    Nominal
}

// TODO: could a Command actually be used here, and just have the actual
// ^ remState, including the config, passed to it??
// That would change where Commands are loaded but could actually be way more idiomatic
pub type ConfigCommandRunFn = fn(args: &Vec<String>, config: &mut config::Config) -> ConfigCommandResult;

// TODO: refactor to share ArgsLim (ideally in a better place than utils.rs)
enum ArgsLim {
    /// There are this many arguments total, and the last of them can be any string with spaces
    /// i.e. if we have 3 args and input is "A B C D   E" -> ["A", "B", "C D   E"]
    EndlessLastArg(i32),
    /// The number of arguments must be precisely this value
    Fixed(i32),
    /// There must be no arguments
    None
}

struct ConfigCommand {
    names: Vec<String>,
    /// The number and structure of arguments that the command expects
    args_lim: ArgsLim,
    pub run: ConfigCommandRunFn
}

impl ConfigCommand {
    pub fn new(names: Vec<String>, args_lim: ArgsLim, run: ConfigCommandRunFn) -> ConfigCommand {
        ConfigCommand {
            names,
            args_lim,
            run
        }
    }

    /// Whether a user's inputted command matches this command's structure
    pub fn matches(&self, name: &str, num_args: i32) -> bool {
        self.names.iter().any(|s| s == name) && match self.args_lim {
            ArgsLim::EndlessLastArg(needed_args) => {
                num_args >= needed_args
            },
            ArgsLim::Fixed(needed_args) => {
                num_args == needed_args
            },
            ArgsLim::None => {
                num_args == 0
            }
        }
    }

    /// Parse the input properly
    // Ex. "mycommand A B Endless arg as str" -> ["A", "B", "Endless argument as one string"]
    /// This assumes that the command already matches (and thus will not check argument counts)
    pub fn parse_input(&self, full_input: &str) -> Vec<String> {
        // TODO: impl ignoring spaces and keeping case within quotes, handling backslashes, etc.
        match self.args_lim {
            ArgsLim::EndlessLastArg(needed_args) => {
                // Since spaces may be included in the final endless argument, we only want to split the start
                // TODO: ignore case and trim for all args except the last one
                full_input.splitn(needed_args as usize + 1, ' ').skip(1).map(|s| s.to_string()).collect()
            },
            _ => {
                // TODO: ignore case and trim for all args except the last one
                full_input.split(' ').skip(1).map(|s| s.to_string()).collect()
            }
        }
    }
}*/

// Store these config commands lazily so they are only accessed on the first call
static CONFIG_COMMAND_LIST: LazyLock<Vec<Command>> = LazyLock::new(|| {vec![
    Command::new(
        utils::string_vec!["tip"], ArgsLim::EndlessLastArg(2),
        |args, state| {
            // Add a tip
            let userpath = &args[1];
            state.config.add_tip(args[0].trim(), userpath);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["shell_alias"], ArgsLim::EndlessLastArg(2),
        |args, state| {
            // Add a shell alias
            let usercommand = &args[1];
            state.config.add_shell_alias(args[0].trim(), usercommand, false);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["shell_alias_quitting"], ArgsLim::EndlessLastArg(2),
        |args, state| {
            // Add a shell alias that quits after running
            let usercommand = &args[1];
            state.config.add_shell_alias(args[0].trim(), usercommand, true);
            CommandResult::Nominal
        }
    ),
]});
