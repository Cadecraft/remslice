# remslice 🍊

A personal, ergonomic CLI assistant for my miscellaneous needs

This project is currently in development and not all features are implemented;
please keep in mind that this is purely intended for my own personal and productivity usecases.

## Installation

To build remslice, Cargo must be installed. This is a cross-platform application.

There is also the dependency of `cli-clipboard` for the copy feature:
- On Windows and macOS, this should work without any additional installation.
- On Linux, according to crates.io, `xorg-dev` and `libxcb-composite0-dev` must first be installed.

In the directory, run `cargo build` to build or `cargo install --path .` to build and install this project.

It is recommended to add the generated executable to the PATH variable or put it in your bin folder
for global easy access on the command line.

## Config

Use the `remfetch` command to see the directory where your `.remrc` file should be placed (it should be your home directory).
This file is where all configuration goes.
Follow this syntax in the `.remrc` file:

- Use the hash (`#`) symbol to start a comment line
```
# This is a comment
```

- `tip` creates a new tip with the given nickname that links to the given file path
    - Because file paths come as the last argument, spaces in them *are* allowed naturally; do not use quotes around paths. This also means spaces *cannot* be used in tip nicknames; consider underscores or dashes. Capitalization is allowed and preserved.
```
tip tipname C:/MyFolder/thing.txt
tip anothertip C:/Other/Path/anotherthing.txt
```

- `todo` defines the path used by the todo features (ex. the `tda` command)
```
todo C:/MyFolder/todos_list.md
```

- `shell_alias` defines an alias to a shell command (to be run with the `al` command)
    - Runs in `powershell` for Windows and `sh` elsewhere
    - Like with the file paths above, you *cannot* use spaces in your alias names.
```
# Runs `firefox -P "Personal` when you type `al ffp`
shell_alias ffp firefox -P "Personal"
```

<!-- TODO: if ever published publicly, change the example config to be less personal -->
<!-- TODO: when publishing publicly, check ALL todos -->
<!-- TODO: when publishing publicly, change/remove bye and hi messages -->

Example config:
```
# remrc for personal PC
# R: v0.2.0, E: 2024/08/09

# Tips: these are used to easily see the contents of frequently accessed text documents
tip vimtoremember C:/Cade/PDFs/Utility/ToRememberDocs/VimToRemember.md
tip shortcutstoremember C:/Cade/PDFs/Utility/ToRememberDocs/ShortcutsToRemember.md
tip testtodos C:/Cade/Java/testtodos.txt
tip commandstoremember C:/Cade/PDFs/Utility/ToRememberDocs/CommandsToRemember.md
tip help C:/Cade/Rust/Misc/remslice/README.md
tip todos C:/Cade/Favorites/Todos/todos_ShortTermSync2.md
# TODO: add class schedules and other useful files

# Todo path
todo C:/Cade/Favorites/Todos/todos_ShortTermSync2.md

# Aliases
shell_alias ffp & "C:/Program Files/Mozilla Firefox/firefox.exe" -P "Personal"
shell_alias ffc & "C:/Program Files/Mozilla Firefox/firefox.exe" -P "College"
```

## System Commands
- `exit`/`quit`/`q` - exit immediately
- `version`/`ver` - display simple version information
- `remfetch` - aesthetically display more version information (think neofetch)
- `pwd` - display the current working directory
- `help` - display a simple help screen that points to this README file (if you use this often, consider adding a tip)

## Misc. Commands
- `bye` - exit with a farewell message
- `ping` - pong!
- `wipe` - wipe the screen
- `time` - display the current time
- `copy`/`y` - copy ("yank") the last copyable message sent to the system clipboard (generally used after `score`)
- `paste`/`p` - display the contents of the system clipboard
- `pasterun!`/`pr!` - run the contents of the system clipboard as a command input to rem (dangerous--`paste` first to see contents!)

<!-- TODO: list all if still unfinished -->

## Procedure/Action Commands
- `score` - generate a daily score based on input prompts
- `tip {nickname}` - load a file based on its defined nickname (searches for the first one starting with the argument) (note: `b` is synonymous with `tip`)
- `tip {nickname} {grep prompt}` - load a file like above, but automatically call the `grep` command below on it
- `tip-ls` - list all available tips and their file paths
- `grep` - search through the currently loaded file for lines containing a specific term (case-insensitive; instantly displays)
- `line {line number}` - print the given line of the currently loaded file
- `tda` - "todo append": add an entry into the todo file specified in `remrc.txt` (entries are automatically markdown bulleted with a dash)
- `tdt` - "todo top": display the top (most recent) entries in the todo file (up until the most recent `#` header); display lowercase alphabetical IDs alongside each entry
- `tdt2` - "todo top x2": display more of the top todo entries (up until the 2nd most recent `##` header)
- `tdc` - "todo clear/complete": toggle the strikethrough for a todo in the todo file by its lowercase alphabetical ID (see `tdt`)
- `tdn` - "todo new day": insert the current date as a new `##` header in the todo file
- `al {shell alias}` - run the command defined by a certain shell alias in the config file
- `al-ls` - list all available aliases and their file paths

## More

This was developed using the `rebelot/kanagawa.nvim` theme.

Todo files are expected to be in markdown, formatted like this:
```md
# Todo List
## 2024/08/08
- build a project
- ~~this one has been completed~~
## 2024/08/09
- keep building
- take notes
```

## Etymology?

*Rem is easily forgotten;*

*this program takes utilitarian slices of everyday tools and systems to make remembering things easier,*

*creating a clean and lightweight feel, like an orange slice*
