use crate::remstate;
use crate::utils;
use crate::command;

pub fn run_score(state: &mut remstate::RemState) {
    // Get based on config
    let divide_by: f32 = state.config.score_divby;
    let formula_number: &str = &state.config.score_formula_number;
    // Obtain relevant information
    println!("Today's questions:");
    let mut daily_score_disp = format!("Daily Score (Formula {}) = (", formula_number);
    let mut total_score: f32 = 0.0;
    for cat in state.config.score_positive() {
        println!("{}", cat);
        let uin = utils::get_user_input_decimal(0.0, 1.0);
        total_score += uin;
        daily_score_disp.push_str(&format!(" + {:.2}", uin));
    }
    for cat in state.config.score_negative() {
        println!("{}", cat);
        let uin = utils::get_user_input_decimal(0.0, 1.0);
        total_score -= uin;
        daily_score_disp.push_str(&format!(" - {:.2}", uin));
    }
    // Calculate and format
    total_score /= divide_by;
    daily_score_disp.push_str(&format!(") / {} = {:.2}", divide_by, total_score));
    state.to_copy_val = daily_score_disp.clone();
    // Create the score report
    println!("Today's daily score:");
    println!("{}", daily_score_disp);
    // Options: copy, continue, restart, edit
    println!("To copy the report, enter `copy`");
}

pub fn run_tip(state: &mut remstate::RemState, key: &str, grepval: Option<&str>) {
    // Search for the given file and display it, so a tip can be found
    match state.config.get_tip_value(key) {
        Some(tip_value) => {
            // Open and load the file, if possible
            match utils::read_file(&tip_value) {
                Some(thecontents) => {
                    // Load the file
                    state.file_loaded = thecontents.clone();
                    println!("The file at {} is loaded into the buffer.", tip_value);
                    match grepval {
                        // Automatically grep
                        Some(query) => {
                            run_grep(state, query);
                        },
                        _ => {
                            println!("Consider using `grep` or `print`");
                        }
                    }
                },
                _ => {
                    println!("The file pointed to doesn't exist");
                }
            }
        },
        _ => {
            // Failed
            println!("The tip nickname doesn't exist");
        }
    }
}

// Search the file for lines including the query
pub fn run_grep(state: &remstate::RemState, query: &str) {
    let mut success: bool = false;
    println!("Searching...");
    for (i, line) in state.file_loaded.lines().enumerate() {
        // Match?
        if line.to_lowercase().find(&query.to_lowercase()).is_some() {
            // Found
            println!("   {:5} {}", i + 1, line);
            success = true;
        }
    }
    if !success {
        println!("I found no results in the file.");
    }
}

/// Get the line of the given number
pub fn run_line(state: &remstate::RemState, line_num: &str) {
    match line_num.parse::<usize>() {
        Ok(linenum) => {
            if linenum < 1 || linenum > state.file_loaded.lines().count() {
                println!("Enter a line number from 1 to {}", state.file_loaded.lines().count());
                return;
            }
            // Print the line
            println!("   {:5} {}", linenum, state.file_loaded.lines().collect::<Vec<&str>>()[linenum - 1]);
        },
        _ => {
            println!("Enter a line number from 1 to {}", state.file_loaded.lines().count());
        }
    }
}

pub fn run_tda(state: &remstate::RemState, todo_string: &str) {
    // Append to the end of todos
    if utils::append_to_file(&state.config.todo_path, &format!("- {}", todo_string)) {
        println!("Todo added successfully");
    } else {
        println!("Todo could not be added");
    }
}

/// Todo top (up until the given number of headers, default 1)
pub fn run_tdt(state: &mut remstate::RemState, count: u32) {
    const TDT_MAX_ARG: u32 = 9;
    if count > TDT_MAX_ARG {
        println!("It is unreasonable to request this many ({}) todo headers.", count);
        println!("Please simply open the todo file in a text editor (i.e. using the 'ted' command).");
        println!("You can configure the 'ted' command in your .remrc file.");
        return;
    }
    // Get the end of todos
    match utils::read_file(&state.config.todo_path) {
        Some(contents) => {
            // Print the end of the file up until the first hash symbol
            let mut res = String::new();
            let lines = contents.lines().collect::<Vec<&str>>();
            let mut headers_seen = 0;
            state.todos_ids.clear();
            let mut currid = "a".to_string();
            for i in (0..lines.len()).rev() {
                let mut final_line = false;
                if lines[i].starts_with("##") {
                    headers_seen += 1;
                    if headers_seen >= count {
                        final_line = true;
                    }
                }
                // Track this line's ID
                state.todos_ids.insert(currid.clone(), i + 1);
                // Line goes above res (because iterating in reverse)
                res = format!("{:3}{:5} {}\n{}", currid, i + 1, lines[i], res);
                if final_line {
                    break;
                }
                currid = utils::generate_next_id(currid.clone());
            }
            println!("{}", res);
        },
        _ => {
            println!("Todo file could not be accessed");
        }
    }
}

/// Clear the todo of a certain ID
pub fn run_tdc(state: &remstate::RemState, id: &str) {
    let linenum: usize = match state.todos_ids.get(id) {
        Some(l) => {
            *l
        },
        _ => {
            println!("ID does not exist");
            return;
        }
    };
    match utils::read_file(&state.config.todo_path) {
        Some(contents) => {
            let mut lines = contents.lines().collect::<Vec<&str>>();
            // Check bounds
            if linenum < 1 || linenum > lines.len() {
                println!("Line number pointed to is out of bounds");
                return;
            }
            let target: String = lines[linenum - 1].to_string();
            let res: String = utils::strikethrough_text(&target);
            // Print successful result
            println!("   {:5} {}", linenum, res);
            // Update the contents lines
            lines[linenum - 1] = &res;
            let mut newcontents = String::new();
            for line in lines {
                newcontents.push_str(&format!("{}\n", line));
            }
            // Overwrite the file with the new contents
            utils::write_to_file(&state.config.todo_path, &newcontents);
        },
        _ => {
            println!("Todo file could not be accessed");
            return;
        }
    }
}

/// Edit the latest todo
pub fn run_tde(state: &remstate::RemState, new_todo: &str) {
    if utils::edit_last_line_of_file(&state.config.todo_path, &format!("- {}", new_todo), false) {
        println!("- {}", new_todo);
    } else {
        println!("Topmost todo could not be edited");
    }
}

/// Append-edit the latest todo
pub fn run_tdae(state: &remstate::RemState, new_todo: &str) {
    // If the first char is punctuation, don't include a space between the original and appended contents
    let formatted_to_append: String = match new_todo.chars().next().unwrap_or(' ') {
        ',' | ';' | '-' | '.' | ':' => new_todo.to_string(),
        _ => format!(" {}", new_todo)
    };
    if utils::edit_last_line_of_file(&state.config.todo_path, &formatted_to_append, true) {
        println!("Appended to the topmost todo");
    } else {
        println!("Topmost todo could not be edited");
    }
}

/// Start a new day as a header in the todo list
pub fn run_tdn(state: &remstate::RemState) {
    // Append the day to the end of todos
    if utils::append_to_file(&state.config.todo_path, &format!("## {}", utils::get_date_only_formatted())) {
        println!("New day added successfully");
    } else {
        println!("Todo could not be added");
    }
}

/// Open a third-party text editor with the todo file and close remslice
pub fn run_ted(state: &remstate::RemState) -> command::CommandResult {
    let editor_command_prefix = &state.config.ted_command_prefix;
    let full_command = format!("{} {}", editor_command_prefix, state.config.todo_path);
    let command_successful = utils::run_shell_command(&full_command);
    if command_successful {
        command::CommandResult::EndProgram
    } else {
        println!("The todo editor command failed! Check ted_command_prefix in your .remrc file");
        command::CommandResult::Nominal
    }
}

/// Run a shell alias
pub fn run_al(state: &remstate::RemState, alias: &str) -> command::CommandResult {
    match state.config.get_shell_alias(&alias) {
        Some(alias) => {
            let command_successful = utils::run_shell_command(&alias.command);
            // Only quit if successful AND desired
            if command_successful && alias.quit_after_running {
                command::CommandResult::EndProgram
            } else {
                command::CommandResult::Nominal
            }
        },
        _ => {
            println!("The shell alias doesn't exist");
            command::CommandResult::Nominal
        }
    }
}

/// Display all aliases
pub fn run_al_ls(state: &remstate::RemState) {
    println!("All shell aliases added:");
    println!("{}", state.config.display_shell_aliases());
    println!("All rem aliases added:");
    println!("{}", state.config.display_rem_aliases());
}

/// Print the current file
pub fn run_print(state: &remstate::RemState) {
    for (i, line) in state.file_loaded.lines().enumerate() {
        println!("   {:5} {}", i + 1, line);
    }
}
