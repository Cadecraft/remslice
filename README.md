# remslice

A personal, ergonomic CLI assistant for my miscellaneous needs

This project is currently in development and not all features are implemented;
please keep in mind that this is purely intended for my own personal and productivity usecases.

## Installation

To build remslice, Cargo must be installed.

There is also the dependency of `cli-clipboard` for the copy feature:
- On Windows and macOS, this should work without any additional installation.
- On Linux, according to crates.io, `xorg-dev` and `libxcb-composite0-dev` must be installed.

Run `cargo build` or `cargo run` for this project.

It is recommended to add the generated executable to the PATH variable or put it in your bin
for global easy access on the command line.

## Config

Check or modify `config.rs` to find or set the file path to the `remrc.txt` file.

This file is where all configuration goes.

Syntax for `remrc.txt`:
```
# Comment
# This creates a new tip nicknamed `tipname` that links to a file
tip tipname C:/MyFolder/thing.txt
tip anothertip C:/Other/Path/anotherthing.txt
```

## System Commands
- `exit`/`quit`/`q` - exit immediately
- `version`/`ver` - display the version information
- `cd` - display the current working directory

## Misc. Commands
- `bye` - exit with a farewell message
- `ping` - pong!
- `wipe` - wipe the screen
- `time` - display the current time
- `copy` - copy the last copyable message sent (generally used after `score`)

<!-- TODO: list all -->

## Procedure/Action Commands
- `score` - generate a daily score based on input prompts
- `tip {nickname}` - load a file based on its defined nickname (searches for the first one starting with the argument) (note: `b` is synonymous with `tip`)
- `tip {nickname} {grep prompt}` - load a file like above, but automatically call the `grep` command below on it
- `grep` - search through the currently loaded file for lines containing a specific term (case-insensitive; instantly displays)
- `tda` - "todo append": add an entry into the todo file specified in `remrc.txt` (entries are automatically markdown bulleted with a dash)

## Etymology?

*Rem is easily forgotten;*

*this program takes utilitarian slices of everyday tools and systems to make remembering things easier,*

*creating a clean and lightweight feel, like an orange slice*
