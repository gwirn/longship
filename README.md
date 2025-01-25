# Longship

***Makes your command line prompt more informative without being intrusive.***


Simple replace your current `PS1` with the path to the longship executable.
**No need for any elevated privileges.**
Aims to improve your prompt without interfering with any other plugin or tool you have installed for your terminal by just altering the `PS1`.
So you can still use all your desired command completions and all other features you modified your shell with without any changes.

## Installation
[Install rust and cargo](https://www.rust-lang.org/tools/install)

Run `cargo build --release` to create a binary of the program.


## Example
```
🛰 [USER.11]~/Code/rust_projects/longship/src 🌿master ⚡ 0.13.0  🦀1.82.0  🐿️go1.23.4  🐍3.13.0 python_venv ✔/❌3s
»
```

If in a directory which is in `/home/USER/..` a `~` will be displayed instead.

If rust, go, zig or python files are present in the current directory they will be indicated
with an indicating emoji and the version of the compiler/ interpreter.

If a python virtual environment is active its name will be displayed even when there are
no python files.

If it is a remote session it is indicated with the user name, the last part of the IP
address in front of the path and a satellite emoji.

If in a directory where in its or the parent path a `.git/HEAD` file is present the name
of the branch will be displayed.

If your previous command ran longer than 2 seconds the command execution time will be shown at the end of the prompt with second precission. If the command exited successful, it will be indicated in green including a leading check mark, otherwise in red with a leading cross.

## Configuration
*zsh*
```zsh
PS1=$'$(/path/to/longship)'
```
*bash*
```bash
PS1='$(/path/to/longship)'
```

In order to be able to see the command execution time and whether it succseeded or not you need to expose the environment variables `LONGSHIP_TIME_STAMP` and `LONGSHIP_RET_CODE`. The first one needs to be created just before a command is run and the second one just after a command is run. This can be done via adding the following to your `.bashrc` or `.zshrc`.

*zsh*
```zsh
# This will run before any command is executed.
preexec() {
    LONGSHIP_TIME_STAMP=$(date +%s)
    export LONGSHIP_TIME_STAMP
}
# This will run after the execution of the previous full command line.
precmd() {
    LONGSHIP_RET_CODE=$?
    export LONGSHIP_RET_CODE
}
```
*bash* [source](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)
```bash
# This will run before any command is executed.
function PreCommand() {
  if [ -z "$AT_PROMPT" ]; then
    return
  fi
  unset AT_PROMPT
  LONGSHIP_TIME_STAMP=$(date +%s)
  export LONGSHIP_TIME_STAMP

}
trap "PreCommand" DEBUG

# This will run after the execution of the previous full command line.
FIRST_PROMPT=1
function PostCommand() {
  export LONGSHIP_RET_CODE=$?
  AT_PROMPT=1
  if [ -n "$FIRST_PROMPT" ]; then
    unset FIRST_PROMPT
    return
  fi
}
PROMPT_COMMAND="PostCommand"

```


## Inspiration
This is heavily inspired by [starship](https://github.com/starship/starship/tree/master) which is a much more powerful and versatile prompt but needs to be used with `eval`.
