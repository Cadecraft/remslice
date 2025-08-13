use crate::command::{ ArgsLim, Command, CommandResult };
use crate::utils;
use crate::feature;
use crate::remfetch;
use std::sync::LazyLock;

// Store these commands lazily so they are only accessed on the first call
pub static REM_COMMANDS: LazyLock<Vec<Command>> = LazyLock::new(|| {vec![
    Command::new(
        utils::string_vec!["score"], ArgsLim::None,
        |_args, state| {
            feature::run_score(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["version", "ver"], ArgsLim::None,
        |_args, state| {
            println!("REMSLICE ({})", state.rem_data.to_string());
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["remfetch"], ArgsLim::None,
        |_args, state| {
            println!("{}", remfetch::remfetch(&state.rem_data));
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["bye"], ArgsLim::None,
        |_args, _state| {
            println!("bye!");
            utils::await_enter();
            CommandResult::EndProgram
        }
    ),
    Command::new(
        utils::string_vec!["ping"], ArgsLim::None,
        |_args, state| {
            state.ping_count += 1;
            println!("pong (x{})", state.ping_count);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["help"], ArgsLim::None,
        |_args, _state| {
            println!("A detailed list of all commands can be found in `README.md`;");
            println!("please check it out for the features and cool stuff!");
            println!("- `exit`/`quit`/`q` - exit immediately");
            println!("- `version`/`ver` - display simple version information");
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["wipe", "clear"], ArgsLim::None,
        |_args, _state| {
            // Print enough times that the screen gets filled
            for _i in 0..100 {
                println!();
            }
            println!("The screen is clear!");
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["pwd"], ArgsLim::None,
        |_args, _state| {
            println!("{}", utils::get_current_working_dir());
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tip", "b"], ArgsLim::EndlessLastArg(2),
        |args, state| {
            // Tip and grep
            feature::run_tip(state, &args[0], Some(&args[1]));
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tip", "b"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            // Tip only
            feature::run_tip(state, &args[0], None);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tip-ls"], ArgsLim::None,
        |_args, state| {
            println!("All tips added:");
            println!("{}", state.config.display_tips());
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["grep"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            feature::run_grep(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["line"], ArgsLim::Fixed(1),
        |args, state| {
            feature::run_line(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tda"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            feature::run_tda(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tdt"], ArgsLim::None,
        |_args, state| {
            feature::run_tdt(state, 1);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tdt"], ArgsLim::Fixed(1),
        |args, state| {
            // TODO: refactor this kind of check into something on the Command level?
            // (i.e. every Command, in run, would type-check its argument)
            match args[0].parse::<u32>() {
                Ok(count) => {
                    feature::run_tdt(state, count);
                },
                _ => {
                    println!("Please enter a non-negative number");
                }
            };
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tdt2"], ArgsLim::None,
        |_args, state| {
            // A specific command name for backwards compatability only
            feature::run_tdt(state, 2);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tdc"], ArgsLim::Fixed(1),
        |args, state| {
            feature::run_tdc(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tde"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            feature::run_tde(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tdae"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            feature::run_tdae(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["tdn"], ArgsLim::None,
        |_args, state| {
            feature::run_tdn(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["ted"], ArgsLim::None,
        |_args, state| {
            feature::run_ted(state)
        }
    ),
    Command::new(
        utils::string_vec!["al"], ArgsLim::Fixed(1),
        |args, state| {
            // Return the result from the alias, since aliases might be quitting
            feature::run_al(state, &args[0])
        }
    ),
    Command::new(
        utils::string_vec!["al-ls"], ArgsLim::None,
        |_args, state| {
            feature::run_al_ls(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["print"], ArgsLim::None,
        |_args, state| {
            feature::run_print(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["copy", "y"], ArgsLim::None,
        |_args, state| {
            utils::copy_to_clipboard(&state.to_copy_val);
            if state.to_copy_val.chars().count() > 6 {
                println!("Yanked string starting with '{}'", &state.to_copy_val[..4]);
            } else {
                println!("Yanked string '{}'", state.to_copy_val);
            }
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["paste", "p"], ArgsLim::None,
        |_args, _state| {
            match utils::paste_from_clipboard() {
                Some(contents) => {
                    println!("{}", contents);
                },
                _ => {
                    println!("Couldn't paste the clipboard contents");
                }
            }
            CommandResult::Nominal
        }
    ),
    Command::new(
        utils::string_vec!["q", "exit", "quit"], ArgsLim::None,
        |_args, _state| {
            CommandResult::EndProgram
        }
    ),
    Command::new(
        utils::string_vec!["time"], ArgsLim::None,
        |_args, state| {
            let output = utils::get_time_formatted();
            state.to_copy_val = output.clone();
            println!("{}", output);
            CommandResult::Nominal
        }
    ),
]});
