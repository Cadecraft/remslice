# remslice

A personal, ergonomic CLI assistant for my miscellaneous needs

This project is currently in development and not all features are implemented;
please keep in mind that this is purely intended for my own personal and productivity usecases.

## Installation

To build remslice, Cargo must be installed.

There is also the dependency of `cli-clipboard` for the copy feature:
- On Windows and macOS, this should work without any additional installation.
- On Linux, according to crates.io, `xorg-dev` and `libxcb-composite0-dev` must first be installed.

Run `cargo build` or `cargo run` for this project.

It is recommended to add the generated executable to the PATH variable or put it in your bin
for global easy access on the command line.

## Config

Check or modify `config.rs` to find or set the file path to the `remrc.txt` file.
This file is where all configuration goes.
Follow this syntax in `remrc.txt`:

- Use the hash (`#`) symbol to start a comment line
```
# This is a comment
```

- `tip` creates a new tip with the given nickname that links to the given file path
```
tip tipname C:/MyFolder/thing.txt
tip anothertip C:/Other/Path/anotherthing.txt
```

- `todopath` defines the path used by the todo features (ex. the `tda` command)
```
todopath C:/MyFolder/todos_list.md
```

- Because file paths come as the last argument, spaces in them *are* allowed naturally; do not use quotes around paths. This also means spaces *cannot* be used in tip nicknames; consider underscores or dashes. Capitalization is allowed and preserved.

Example config:
```
tip vimtoremember C:/Cade/PDFs/Utility/ToRememberDocs/VimToRemember.md
tip shortcutstoremember C:/Cade/PDFs/Utility/ToRememberDocs/ShortcutsToRemember.md
tip testtodos C:/Cade/Java/testtodos.txt
todo C:/Cade/Java/testtodos.txt
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
