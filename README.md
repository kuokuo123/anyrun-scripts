# Anyrun-Scripts

Run cli programs or arbitrary shell scripts in preferred terminal emulators with [anyrun](https://github.com/Kirottu/anyrun). 

## Usage

Set the preferred terminal emulator (default to foot) in the config file. Type the configured prefix to call for shell scripts.

With default setting, type in ":sh ranger" will open ranger in foot terminal, with the input line being translated to "foot -e ranger" and launched. You can also configure customized prefix to call for arbitrary scripts. See the example config file.

Note that this plugin allows for two kinds of prefixes. The main prefix call for anyrun-terminal plugin itself, the secondary prefix call for the specific command or script you configured. See the example config file.

## Configuration

With this example, typing ":sh ranger" will open alacritty and run ranger. Typing ":ddg tux the penguin" will open the browser and search "tux the penguin" with duckduckgo. Typing ":fo" will open fzf in foot, which allows you to open the selected file with xdg-open. Typing ":the" will open a script named theme-switcher.sh.

You can also depend wholly on secondary prefixes by setting the main prefix to be "".

```ron
// <Anyrun config dir>/scripts.ron

Config(

  // the main prefix to call this plugin from anyrun.
  prefix: ":",

  // Override the shell used to launch the command. For example, "shell: Some("/bin/bash"),"
  // "shell: None," will use env $SHELL as the default shell to launch program.
  shell: None,

  // Options: Custom, Foot, Alacritty
  // NOTE:
  // 	1. `{}` is replaced by the anyrun search query.
  // 	2. When setting custom engines, all four options (name, cmd, secondary_prefix, icon) should be set, or the config won't be read.
  engines: [

    Custom(
      name: "Fzf-open",
      cmd: "foot bash -c 'fd -tf -H . ~ | fzf | xargs xdg-open'",
      secondary_prefix: "fo",
      icon: "gnome-search-tool",
    ),

    Custom(
      name: "DuckDuckgo",
      cmd: "xdg-open https://duckduckgo.com/?q={}",
      secondary_prefix: "ddg ",
      icon: "gnome-search-tool",
    ),

    Custom(
      name: "Theme Switcher",
      cmd: "$HOME/.config/hypr/scripts/theme-switcher.sh",
      secondary_prefix: "the",
      icon: "gnome-settings-theme",
    ),

    Alacritty,

  ],

)

```

