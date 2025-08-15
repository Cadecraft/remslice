use crate::remstate;
use crate::utils;

pub enum CommandResult {
    Nominal,
    Error(String),
    EndProgram
}

pub type CommandRunFn = fn(args: &Vec<String>, state: &mut remstate::RemState) -> CommandResult;

pub enum ArgsLim {
    /// There are this many arguments total, and the last of them can be any string with spaces
    /// i.e. if we have 3 args and input is "A B C D   E" -> ["A", "B", "C D   E"]
    EndlessLastArg(i32),
    /// The number of arguments must be precisely this value
    Fixed(i32),
    /// There must be no arguments
    None
}

pub struct Command {
    names: Vec<String>,
    /// The number and structure of arguments that the command expects
    args_lim: ArgsLim,
    pub run: CommandRunFn
}

impl Command {
    pub fn new(names: Vec<String>, args_lim: ArgsLim, run: CommandRunFn) -> Command {
        Command {
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
}

/// Find the matching command based on the user's parsed input and run it
pub fn run_command(
    full_input: &str,
    state: &mut remstate::RemState,
    command_list: &Vec<Command>
) -> Option<CommandResult> {
    let processed = utils::process_input(&full_input);
    if processed.is_none() {
        return None;
    }
    let (num_args, command_name) = processed.unwrap();
    let found = command_list.iter().find(|&x| x.matches(&command_name, num_args));
    match found {
        Some(command) => {
            // Run the command, parsing the input properly for the number of arguments
            let parsed = command.parse_input(full_input);
            let res = (command.run)(&parsed, state);
            Some(res)
        },
        _ => {
            None
        }
    }
}
