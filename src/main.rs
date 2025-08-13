// remslice

// Modules
mod remdata;
mod utils;
mod rem;
mod config;
mod remfetch;
mod command;
mod remstate;
mod feature;
mod command_lists;

/* TODO:
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
    let rem_data = remdata::RemData::new("0.6.2", "2025/08/08", true);
    let mut rem = rem::Rem::new(rem_data.clone());

    // Begin the input loop immediately
    loop {
        let user_input = utils::get_user_input_line();
        let res = rem.respond_to_input(user_input, 0);
        match res {
            Some(command::CommandResult::EndProgram) => {
                break;
            },
            _ => ()
        }
    }
    // End
}
