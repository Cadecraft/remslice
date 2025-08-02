use crate::remstate;
use std::sync::LazyLock;

pub enum CommandResult {
    Nominal,
    EndProgram
}

pub struct Command {
    name: String,
    min_args: i32,
    /// The maximum number of arguments. If None, the min_args-th argument and beyond is one infinite string
    max_args: Option<i32>,
    pub run: fn(args: &Vec<String>, state: &mut remstate::RemState) -> CommandResult
}

impl Command {
    pub fn matches(&self, name: &str, num_args: i32) -> bool {
        return name == self.name
            && num_args >= self.min_args
            && (self.max_args.is_none() || num_args <= self.max_args.unwrap())
    }

    // Parse the input properly: return [Arg1, Arg2, "Infinite argument as one string"]
    pub fn parse_input(&self, full_input: &str) -> Vec<String> {
        let mut args_completed: i32 = -1;
        let mut curr_str: String = String::new();
        let mut res: Vec<String> = Vec::new();
        for c in full_input.chars() {
            let at_infinite_arg = args_completed >= self.min_args - 1;
            if !at_infinite_arg && c == ' ' {
                // Treat as a delimiting space before a new argument
                if args_completed >= 0 {
                    res.push(curr_str.clone());
                }
                curr_str.clear();
                args_completed += 1;
            } else {
                // Treat as a verbatim input character
                if at_infinite_arg {
                    curr_str.push(c);
                } else {
                    let c_lower = c.to_lowercase().collect::<String>();
                    curr_str.push_str(&c_lower);
                }
            }
        }
        if args_completed >= 0 {
            res.push(curr_str);
        }
        for s in &res {
            s.trim();
        }
        return res;
    }
}

// Store these commands lazily so they are only accessed on the first call
static COMMAND_LIST: LazyLock<Vec<Command>> = LazyLock::new(|| {vec![
    Command {
        name: String::from("ping"),
        min_args: 0,
        max_args: Some(0),
        run: |args, state| {
            // Do stuff
            state.ping_count += 1;
            println!("pong (x{})", state.ping_count);
            println!("DBG: you entered {}", args.concat());
            CommandResult::Nominal
        }
    },
    Command {
        name: String::from("q"),
        min_args: 0,
        max_args: Some(0),
        run: |_args, _state| {
            // Do stuff
            println!("DBG: QUITTING");
            CommandResult::EndProgram
        }
    },
    Command {
        name: String::from("tda"),
        min_args: 1,
        max_args: None,
        run: |args, _state| {
            // Do stuff
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
