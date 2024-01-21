# Anyrun-Terminal

Run shell commands in a designated terminal emulator through [anyrun](https://github.com/Kirottu/anyrun). This is a modified version of the original [shell](https://github.com/Kirottu/anyrun/tree/master/plugins/shell) plugin in order to save some key strokes when launching cli programs. For example, to launch htop with the shell plugin you should type in ":sh alacritty -e htop", but with this modified plugin ":sh htop" does the same task.

## Usage

Specify your prefer terminal emulator in the config file. In anyrun, type in `<prefix><command>`, where `<prefix>` is the configured prefix and `<command>` is the command you want to run. This plugin can be configured to use your preferred terminal emulator, default to foot. Make sure to set the correct option that executes the command at terminal startup (for example, "-e" for alacritty).

![Demo Video](https://github.com/kuokuo123/anyrun-terminal/raw/main/asset/demo.mkv?raw=true)

## Configuration

```ron
// <Anyrun config dir>/terminal.ron

Config(

  // the prefix to launch this plugin from anyrun
  prefix: "sh",

  // Override the shell used to launch the command
  shell: None,

  // set the emulator, default to foot
  emulator: "foot",

  // the emulator's arguement to execute command when launched. For example, "-e" for alacritty
  exec_opt: "-e",

)
```
