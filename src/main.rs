// remslice

// Modules
mod remdata;
mod utils;
mod rem;

/* TODO:
    allow intaking a file as an argument, or taking flags
    store past commands and allow arrow up/down to move between them
    colors with crossterm?
    rem config
    rem logger
    randomized responses
    feat: search through todos (make api)
    feat: search through utility/to remember lists
    feat: help command
*/

fn main() {
    // Initialize
    let rem_data = remdata::RemData::new("0.1.0", "2024/07/16", true);
    let mut rem = rem::Rem::new(rem_data.clone());

    // Welcome to program (brief)
    println!("REMSLICE ({})", rem_data.to_string());

    // Begin the input loop
    loop {
        // TODO: get user input
        // TODO: have rem respond to user input
        // TODO: quit if necessary
        let user_input = utils::get_user_input_line();
        let should_quit = rem.respond_to_input(user_input);
        if should_quit {
            break;
        }
    }
    // End
}
