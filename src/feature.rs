use crate::remstate;
use crate::remdata;
use crate::utils;
use crate::config::Config;
use crate::remfetch;
use crate::command;

pub fn run_score(state: &mut remstate::RemState) {
    // Get based on config
    let divide_by: f32 = state.config.score_divby();
    let formula_number: &str = &state.config.score_formula_number();
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

pub fn run_grep(state: &mut remstate::RemState, query: &str) {
    // Search the file for lines including it
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
