// remslice

// Modules
mod remdata;
mod utils;
mod rem;
mod config;
mod remfetch;

/* TODO:
    somewhere to download the binaries? auth pw for score or other sensitive features?
    allow intaking a file as an argument, or taking flags
    colors with `crossterm`? `colored`?
    rem log of commands/responses and dates?
    randomized responses to certain commands?
    initial access password (encrypt somehow?) (use a command called auth {password} to access stuff?)
    refactor: move file operations, grep, etc. into a separate struct
    feat: copy file path from tip
    feat: reminders (have timing, store in file, maybe even use notifications)
    feat: display size of log, uptime, etc. in remfetch
    feat: cmd to display a range of lines, if multiple args, in a file
    test installing and copying on linux
*/

fn main() {
    // Initialize
    let rem_data = remdata::RemData::new("0.5.0", "2024/11/30", true);
    let mut rem = rem::Rem::new(rem_data.clone());

    // Begin the input loop immediately
    loop {
        let user_input = utils::get_user_input_line();
        let should_quit = rem.respond_to_input(user_input);
        if should_quit {
            break;
        }
    }
    // End
}
