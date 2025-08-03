use crate::remstate;
use crate::remdata;
use crate::utils;
use crate::config::Config;
use crate::remfetch;
use crate::command;
use crate::feature;
use std::sync::LazyLock;

pub enum CommandResult {
    Nominal,
    EndProgram
}

pub type CommandRunFn = fn(args: &Vec<String>, state: &mut remstate::RemState) -> CommandResult;

enum ArgsLim {
    /// There are this many arguments total, and the last of them can be any string with spaces
    // i.e. if we have 3 args and input is "A B C D   E F" -> ["A", "B", "C D   E F"]
    EndlessLastArg(i32),
    // The number of arguments must be precisely this value
    Fixed(i32),
    // The number of arguments can fall between the minimum and maximum
    Range(i32,i32)
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
            }
            ArgsLim::Range(min_args, max_args) => {
                num_args >= min_args && num_args <= max_args
            }
        }
    }

    /// Parse the input properly: return [Arg1, Arg2, "Endless argument as one string"]
    /// This assumes that the command matches
    pub fn parse_input(&self, full_input: &str) -> Vec<String> {
        match self.args_lim {
            ArgsLim::EndlessLastArg(needed_args) => {
                // Since spaces may be included in the final endless argument, more complicated
                full_input.splitn(needed_args as usize, ' ').map(|s| s.to_string()).collect()
            },
            _ => {
                // Since we assume every space separates an argument, we can simply split
                full_input.split(' ').map(|s| s.to_string()).collect()
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
        string_vec!["score"], ArgsLim::Fixed(0),
        |_args, state| {
            feature::run_score(state);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["version", "ver"], ArgsLim::Fixed(0),
        |_args, state| {
            println!("REMSLICE ({})", state.rem_data.to_string());
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["remfetch"], ArgsLim::Fixed(0),
        |_args, state| {
            println!("{}", remfetch::remfetch(&state.rem_data));
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["bye"], ArgsLim::Fixed(0),
        |_args, _state| {
            println!("bye!");
            utils::await_enter();
            CommandResult::EndProgram
        }
    ),
    Command::new(
        string_vec!["ping"], ArgsLim::Fixed(0),
        |_args, state| {
            state.ping_count += 1;
            println!("pong (x{})", state.ping_count);
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["help"], ArgsLim::Fixed(0),
        |_args, _state| {
            println!("A detailed list of all commands can be found in `README.md`;");
            println!("please check it out for the features and cool stuff!");
            println!("- `exit`/`quit`/`q` - exit immediately");
            println!("- `version`/`ver` - display simple version information");
            CommandResult::Nominal
        }
    ),
    Command::new(
        string_vec!["wipe"], ArgsLim::Fixed(0),
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
        string_vec!["pwd"], ArgsLim::Fixed(0),
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
        string_vec!["tip-ls"], ArgsLim::Fixed(0),
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
        string_vec!["q", "exit", "quit"], ArgsLim::Fixed(0),
        |_args, _state| {
            CommandResult::EndProgram
        }
    ),
    Command {
        names: vec![String::from("tda")],
        args_lim: ArgsLim::EndlessLastArg(1),
        run: |args, _state| {
            println!("DBG: you entered: {}", args[0]);
            CommandResult::Nominal
        }
    },
]});

/// Return the number of space-separated arguments (not including the command name) and the command name
fn process_input(full_input: &str) -> Option<(i32, String)> {
    // TODO: impl myself to ignore spaces and keep case within quotes, handle backslashes, etc.
    let splitted: Vec<&str> = full_input.split(' ').collect::<Vec<&str>>();
    let mut res: Vec<String> = Vec::new();
    for arg in splitted {
        res.push(arg.trim().to_lowercase().to_string());
    }
    if res.len() == 0 {
        return None;
    }
    Some(((res.len() as i32) - 1, res.first().unwrap().clone()))
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
            // Try rem alias
            None
        }
    }
}
