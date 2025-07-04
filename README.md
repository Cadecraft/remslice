# remslice 🍊

A personal, minimal, ergonomic CLI assistant for my miscellaneous productivity needs

Please keep in mind that this project is purely intended for my own personal usecases and requirements; the repository is only open source for simplicity.

## Installation

To build, Cargo must be installed. This is a cross-platform application.

There is also the dependency of `cli-clipboard` for the copy feature:
- On Windows and macOS, this should work without any additional installation.
- On Linux, according to crates.io, `xorg-dev` and `libxcb-composite0-dev` must first be installed.

In the directory, run `cargo build` to build or `cargo install --path .` to build and install this project.

It is recommended to add the generated executable to the PATH variable or put it in your bin folder
for global easy access on the command line.

## Features

This is meant to be a portable, personal productivity CLI tool that makes various everyday tasks more centralized and efficient without having to leave the keyboard. For specifics, see the configuration and command sections below.
- Keep track of todos
- Create aliases to shell commands
- Generate daily scores to measure personal progress
- Deal with the clipboard
- Easily search specific reference files

Some of my everyday usecases:
- Check the date and time (I keep my taskbar hidden)
- Add a shell alias to my config file so I can launch different browser profiles without having to enter its profile manager
- Add todos to a file the moment I think of them using the `tda` command

## Configuration File

All configuration is done in a plain text file called `.remrc`.

Use the `remfetch` command to see the directory where the `.remrc` file should be placed (it should be your home directory).

### Config syntax

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

- `todo` defines the path to the file used by the todo features (e.g. the `tda` command) (see the More section below for the format)
```
todo C:/MyFolder/todos_list.md
```

- `shell_alias` defines an alias to a shell command
    - Runs in `powershell` for Windows and `sh` elsewhere
    - Like with the file paths above, you *cannot* use spaces in your alias names.
```
# Runs `firefox -P "Personal"` when you type `al ffp`
shell_alias ffp firefox -P "Personal"
```

- `shell_alias_quitting` defines an alias to a shell command that also immediately quits remslice after running, to be used a temporary launcher that makes other commands quicker
    - Same details as `shell_alias` above
```
# Launches Firefox and quits remslice when you type `al ffq`
shell_alias_quitting ffq firefox -P "Personal"
```

- `rem_alias` defines an alias to a top-level rem command
    - This can be any valid rem command, including another `rem_alias`
    - Because of the recursive nature of these aliases, recursion cannot exceed 100 levels, to prevent infinite loops (i.e. using `rem_alias crash crash` then running `crash` is not allowed)
```
# Runs tdt when the 't' command is entered
rem_alias t tdt
```

- `score_p` defines a positive input prompt for daily scoring (see the `score` command)
```
# Will ask for a number from 0.0..=1.0 and add it when scoring
score_p Percent of total day's work I got done
```

- `score_n` defines a negative input prompt for daily scoring
```
# Will ask for a number from 0.0..=1.0 and subtract it when scoring
score_n Hours wasted on YouTube (0.0 to 1.0)
```

- `score_divby` defines the number the total score sum is divided by to achieve the final result (default is `5.0`)

- `score_formula_number` defines the number or name used to describe the score formula

### Example config
```
# remrc for personal Windows laptop
# R: v0.2.0, E: 2025/06/27

# Tips: these are used to easily see the contents of frequently accessed text documents
tip help C:/Dev/remslice/README.md
tip shortcuts C:/Documents/Utility/ShortcutsToRemember.md
tip commands C:/Documents/Utility/CommandsToRemember.md
tip todo C:/Lists/Todos/todos.md

# Todo path
todo C:/Lists/Todos/todos.md

# Shell aliases
shell_alias sayhi echo "hi from the shell"
# Open a Firefox profile and close remslice
shell_alias_quitting ffp & "C:/Program Files/Mozilla Firefox/firefox.exe" -P "Personal"
shell_alias_quitting ffc & "C:/Program Files/Mozilla Firefox/firefox.exe" -P "College"
# Make the above only take one keystroke
rem_alias h al ffc
```

## System Commands
- `exit`/`quit`/`q` - exit immediately
- `version`/`ver` - display simple version information
- `remfetch` - aesthetically display more version information
- `pwd` - display the current working directory
- `help` - display a simple help screen that points to this README file (if you use this often, consider adding a tip)

## Misc. Commands
- `bye` - exit with a farewell message
- `ping` - pong!
- `wipe` - wipe the screen
- `time` - display the current time
- `copy`/`y` - copy ("yank") the last copyable message sent to the system clipboard (generally used after `score` or `time`)
- `paste`/`p` - display the contents of the system clipboard
- `pasterun!`/`pr!` - run the contents of the system clipboard as a command input to rem (dangerous--`paste` first to see contents!)

## Procedure/Action Commands
- `score` - generate a daily score based on input prompts defined in your config
- `tip {nickname}` - load a file based on its defined nickname (searches for the first one starting with the argument) (note: `b` is synonymous with `tip`)
- `tip {nickname} {grep prompt}` - load a file like above, but automatically call the `grep` command below on it
- `tip-ls` - list all available tips and their file paths
- `grep` - search through the currently loaded file for lines containing a specific term (case-insensitive; instantly displays)
- `line {line number}` - print the given line of the currently loaded file
- `tda` - "todo append": add an entry into the todo file specified in `remrc.txt` (entries are automatically markdown bulleted with a dash)
- `tdt` - "todo top": display the top (most recent) entries in the todo file (up until the most recent `##` header); display lowercase alphabetical IDs alongside each entry
- `tdt2` - "todo top x2": display more of the top todo entries (up until the 2nd most recent `##` header)
- `tdc` - "todo clear/complete": toggle the strikethrough for a todo in the todo file by its lowercase alphabetical ID (see `tdt`)
- `tde` - "todo edit": edit the topmost todo entry by replacing it, used for making a correction
- `tdae` - "todo append-edit": append text to the topmost todo entry, used for making a correction
- `tdn` - "todo new day": insert the current date as a new `##` header in the todo file
- `al {shell alias}` - run the command defined by a certain shell alias in the config file
- `al-ls` - list all available shell and rem aliases and what they refer to

## More

This was developed using the `rebelot/kanagawa.nvim` colorscheme.

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

This name is based on a) REM sleep, b) the thin fresh simplicity of orange slices, and c) an old way of describing version information that I used in some of my projects:

- Recent version
- Edit date
- Most fully tested
