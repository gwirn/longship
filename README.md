# Longship

Makes your command line prompt more informative without being intrusive.

## Installation
[Install rust and cargo](https://www.rust-lang.org/tools/install)

Run `cargo build --release` to create a binary of the program.

## Configuration

Simple replace your current `PS1` with the path to the longship executable.
**No need for any elevated privileges.**

*zsh*
```zsh
PS1=$'$(/path/to/longship)'
```
*bash*
```bash
PS1='$(/path/to/longship)'
```
## Example
If in a directory which is in `/home/USER/..` a `~` will be displayed instead.
```
~/Code/rust_projects/longship
»
```

If rust, go or python files are present in the current directory they will be indicated
with an indicating emoji and the version of the compiler/ interpreter.

If a python virtual environment is active its name will be displayed even when there are
no python files.

If it is a remote session it is indicated with the user name and the last part of the IP
address in front of the path.

If in a directory where in its or the parent path a `.git/HEAD` file is present the name
of the branch will be displayed.
```
🛰 [USER.11]~/Code/rust_projects/longship/src 🌿master 🐊0.13.0  🦀1.82.0  🐿️go1.23.4  🐍3.13.0 python_venv
»
```

You can still use all your desired command completions and all other features you
modified your shell with without any changes.

## Inspiration
This is heavily inspired by [starship](https://github.com/starship/starship/tree/master) which is a much more powerful and versatile
prompt.
