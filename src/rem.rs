use crate::remdata;
use crate::command;
use crate::remstate;
use crate::command_lists;

/// Stores state and runs commands from user input
pub struct Rem {
    state: remstate::RemState
}

impl Rem {
    pub fn new(rem_data: remdata::RemData) -> Rem {
        Rem {
            state: remstate::RemState::new(rem_data)
        }
    }

    /// Respond to a raw user-inputted string and return whether the program should quit
    pub fn respond_to_input(&mut self, input: String, recursion_level: i32) -> Option<command::CommandResult> {
        // Ensure we aren't in an infinite loop
        const MAX_RECURSION_LEVEL: i32 = 100;
        if recursion_level > MAX_RECURSION_LEVEL {
            println!("Infinitely recursive command encountered (recursed over {MAX_RECURSION_LEVEL} times)");
            return None
        }
        let res = command::run_command(&input, &mut self.state, command_lists::get_rem_commands());
        match res {
            Some(command::CommandResult::Error(descr)) => {
                println!("Error: {}", descr);
                Some(command::CommandResult::Error(descr.clone()))
            },
            Some(_) => {
                res
            },
            None => {
                // Couldn't run the command verbatim, so check rem aliases
                // TODO: refactor this?
                let first_arg = Self::first_arg(&input);
                match self.state.config.get_rem_alias_value(first_arg) {
                    Some(val) => {
                        self.run_rem_alias(&val, recursion_level + 1)
                    }
                    _ => {
                        println!("?");
                        None
                    }
                }
            }
        }
    }

    fn first_arg(input: &str) -> &str {
        input.splitn(2, ' ').collect::<Vec<&str>>()[0]
    }

    /// Run a rem alias recursively
    fn run_rem_alias(&mut self, alias: &str, recursion_level: i32) -> Option<command::CommandResult> {
        self.respond_to_input(alias.to_string(), recursion_level + 1)
    }
}
