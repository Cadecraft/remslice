// remslice

// Modules
mod remdata;
mod utils;
mod rem;
mod config;
mod remfetch;

/* TODO:
    feat: rem_alias, which points to a different rem command (essentially allowing you to bypass starting each alias with `al`)
    feat: allow intaking a file as an argument, or taking flags?
    feat: colors with `crossterm`? `colored`?
    feat: opt-in log file of commands/responses and dates?
    feat: (if log is implemented) display size of log, uptime, etc. in remfetch
    refactor: move file operations, grep, etc. into a separate struct or file?
    feat: copy file path from tip
    feat: reminders (have timing, store in file, maybe even use notifications)
    feat: command to display a range of lines, if multiple args, in a file
    test installing and copying on linux
*/

fn main() {
    // Initialize
    let rem_data = remdata::RemData::new("0.5.3", "2025/02/04", true);
    let mut rem = rem::Rem::new(rem_data.clone());

    // Begin the input loop immediately
    loop {
        let user_input = utils::get_user_input_line();
        let should_quit = rem.respond_to_input(user_input, 0);
        if should_quit {
            break;
        }
    }
    // End
}
