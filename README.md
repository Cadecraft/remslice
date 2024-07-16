# remslice

A personal CLI assistant tool for my miscellaneous needs

This project is currently in development and not all features are implemented.

## Installation

To build remslice, you need Cargo installed.

There is also the dependency of `cli-clipboard` for the copy feature:
- On Windows and macOS, this should work without any additional installation.
- On Linux, according to crates.io, `xorg-dev` and `libxcb-composite0-dev` must be installed.

Run `cargo build` or `cargo run` for this project.

It is recommended to add the generated executable to the PATH variable or put it in your bin.

## Config

Check or modify `config.rs` for the file path to the `remrc.txt` file.

This is where you put your config.

Syntax:
```
# Comment
# This creates a new tip called tipname that links to a file
tip tipname C:/MyFolder/thing.txt
tip anothertip C:/Other/Path/anotherthing.txt
```

## Misc. Commands
- `exit`/`quit`/`q` - exit immediately
- `version`/`ver` - display the version information
- `bye` - exit with a farewell message
- `ping` - pong!
- `copy` - copy the last copyable message sent

<!-- TODO: list all -->

## Procedure/Action Commands
- `score` - generate a daily score based on input prompts
- `wipe` - wipe the screen

## Etymology?

Rem is easily forgotten;
this program takes utilitarian slices of everyday tools and files to make remembering things easier,
creating a clean and lightweight feel, like an orange slice
