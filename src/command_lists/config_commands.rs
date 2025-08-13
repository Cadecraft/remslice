use crate::command::{ ArgsLim, Command, CommandResult };
use crate::utils;
use std::sync::LazyLock;

// Store these commands lazily so they are only accessed on the first call
pub static CONFIG_COMMANDS: LazyLock<Vec<Command>> = LazyLock::new(|| {vec![
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
