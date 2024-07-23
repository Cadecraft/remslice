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
    store past commands and allow arrow up/down to move between them
    colors with `crossterm`? `colored`?
    rem logger
    randomized responses to certain commands?
    initial access password (encrypt somehow?) (use a command called auth {password} to access stuff?)
    figure out how to store the remrc path so users can adjust it (needed?); otherwise, src must be edited
    refactor: move file operations, grep, etc. into a separate struct
    feat: copy file path from tip
    feat: tip
    feat: search through todos (make api?)
    feat: strikethrough todos?
    feat: print only the end of a file
    feat: reminders (have timing, store in file, maybe even use notifications)
    feat: display size of log, uptime, etc. in remfetch
    feat: cmd to display a line (or lines, if multiple args, or range of lines) in a file
    add to path
    test installing and copying on linux
*/

fn main() {
    // Initialize
    let rem_data = remdata::RemData::new("0.3.3", "2024/07/21", true);
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
