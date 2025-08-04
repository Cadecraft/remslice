use crate::remstate;
use crate::utils;
use crate::remfetch;
use crate::feature;
use std::sync::LazyLock;

pub enum CommandResult {
    Nominal,
    EndProgram
}

pub type CommandRunFn = fn(args: &Vec<String>, state: &mut remstate::RemState) -> CommandResult;

enum ArgsLim {
    /// There are this many arguments total, and the last of them can be any string with spaces
    /// i.e. if we have 3 args and input is "A B C D   E" -> ["A", "B", "C D   E"]
    EndlessLastArg(i32),
    /// The number of arguments must be precisely this value
    Fixed(i32),
    None
    // Idea: `Range(i32, i32)`: the number of arguments could fall between the minimum and maximum
}

struct Command {
    names: Vec<String>,
    /// The maximum number of arguments. If None, the min_args-th argument and beyond is one infinite string
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
    /// This assumes that the command matches (and thus will not check argument counts)
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

macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// Store these commands lazily so they are only accessed on the first call
static COMMAND_LIST: LazyLock<Vec<Command>> = LazyLock::new(|| {vec![
    Command::new(
        string_vec!["score"], ArgsLim::None,
        |_args, state| {
            feature::run_score(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["version", "ver"], ArgsLim::None,
        |_args, state| {
            println!("REMSLICE ({})", state.rem_data.to_string());
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["remfetch"], ArgsLim::None,
        |_args, state| {
            println!("{}", remfetch::remfetch(&state.rem_data));
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["bye"], ArgsLim::None,
        |_args, _state| {
            println!("bye!");
            utils::await_enter();
            CommandResult::EndProgram
        }
    ),
    Command::new(
        string_vec!["ping"], ArgsLim::None,
        |_args, state| {
            state.ping_count += 1;
            println!("pong (x{})", state.ping_count);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["help"], ArgsLim::None,
        |_args, _state| {
            println!("A detailed list of all commands can be found in `README.md`;");
            println!("please check it out for the features and cool stuff!");
            println!("- `exit`/`quit`/`q` - exit immediately");
            println!("- `version`/`ver` - display simple version information");
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["wipe", "clear"], ArgsLim::None,
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
        string_vec!["pwd"], ArgsLim::None,
        |_args, _state| {
            println!("{}", utils::get_current_working_dir());
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tip", "b"], ArgsLim::EndlessLastArg(2),
        |args, state| {
            // Tip and grep
            feature::run_tip(state, &args[0], Some(&args[1]));
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tip", "b"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            // Tip only
            feature::run_tip(state, &args[0], None);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tip-ls"], ArgsLim::None,
        |_args, state| {
            println!("All tips added:");
            println!("{}", state.config.display_tips());
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["grep"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            feature::run_grep(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["line"], ArgsLim::Fixed(1),
        |args, state| {
            feature::run_line(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tda"], ArgsLim::EndlessLastArg(1),
        |args, state| {
            feature::run_tda(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tdt"], ArgsLim::None,
        |_args, state| {
            feature::run_tdt(state, 1);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tdt"], ArgsLim::Fixed(1),
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
        string_vec!["tdt2"], ArgsLim::None,
        |_args, state| {
            // A specific command name for backwards compatability only
            feature::run_tdt(state, 2);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tdc"], ArgsLim::Fixed(1),
        |args, state| {
            feature::run_tdc(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tde"], ArgsLim::Fixed(1),
        |args, state| {
            feature::run_tde(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tdae"], ArgsLim::Fixed(1),
        |args, state| {
            feature::run_tdae(state, &args[0]);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["tdn"], ArgsLim::None,
        |_args, state| {
            feature::run_tdn(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["al"], ArgsLim::Fixed(1),
        |args, state| {
            // Return the result from the alias, since aliases might be quitting
            feature::run_al(state, &args[0])
        }
    ),
    Command::new(
        string_vec!["al-ls"], ArgsLim::None,
        |_args, state| {
            feature::run_al_ls(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["print"], ArgsLim::None,
        |_args, state| {
            feature::run_print(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["copy", "y"], ArgsLim::None,
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
        string_vec!["paste", "p"], ArgsLim::None,
        |_args, state| {
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
        string_vec!["q", "exit", "quit"], ArgsLim::None,
        |_args, _state| {
            CommandResult::EndProgram
        }
    ),
    Command::new(
        string_vec!["time"], ArgsLim::None,
        |_args, state| {
            let output = utils::get_time_formatted();
            state.to_copy_val = output.clone();
            println!("{}", output);
            CommandResult::Nominal
        }
    ),
]});

/// Return the number of space-separated arguments (not including the command name) and the command name
fn process_input(full_input: &str) -> Option<(i32, String)> {
    // TODO: impl ignoring spaces and keeping case within quotes, handling backslashes, etc.
    let splitted: Vec<&str> = full_input.split(' ').collect::<Vec<&str>>();
    match splitted.len() {
        0 => return None,
        _ => Some(((splitted.len() as i32) - 1, splitted[0].to_string()))
    }
}

/// Find the matching command based on the user's parsed input and run it
pub fn run_command(full_input: &str, state: &mut remstate::RemState) -> Option<CommandResult> {
    let processed = process_input(&full_input);
    if processed.is_none() {
        return None;
    }
    let (num_args, command_name) = processed.unwrap();
    let found = COMMAND_LIST.iter().find(|&x| x.matches(&command_name, num_args));
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
