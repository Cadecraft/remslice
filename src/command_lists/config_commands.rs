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
    Command::new(
        utils::string_vec!["rem_alias"], ArgsLim::EndlessLastArg(2),
        |args, state| {
            // Add a rem alias
            let usercommand = &args[1];
            state.config.add_rem_alias(args[0].trim(), usercommand);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["todo"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            state.config.todo_path = args[0].clone();
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["ted_command_prefix"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            state.config.ted_command_prefix = args[0].clone();
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["score_p"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            state.config.add_score_factor(args[0].clone(), true);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["score_n"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            state.config.add_score_factor(args[0].clone(), false);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["score_divby"], ArgsLim::Fixed(1),
        |args, state| {
            let userdivby = &args[0];
            match userdivby.parse::<f32>() {
                Ok(res) => {
                    state.config.score_divby = res;
                },
                _ => {
                    // Error
                    // TODO: return an actual error command result
                }
            };
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["score_formula_number"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            state.config.score_formula_number = args[0].clone();
            CommandResult::Nominal
        }
    ),
]});
