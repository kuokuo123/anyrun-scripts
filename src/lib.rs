use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use serde::{Deserialize, Serialize};
use std::{env, fs, process::Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Engine {
    Shell,
    Alacritty,
    Foot,
    Custom { name: String, cmd: String, secondary_prefix: String, icon: String },
}

impl Engine {
    fn value(&self) -> &str {
        match self {
            Self::Shell => "$TERMINAL -e {}",
            Self::Alacritty => "alacritty -e {}",
            Self::Foot => "foot {}",
            Self::Custom { cmd, .. } => cmd,
        }
    }
    fn secondary_prefix(&self) -> &str {
        match self {
            Self::Shell => "sh ",
            Self::Alacritty => "sh ",
            Self::Foot => "sh ",
            Self::Custom { secondary_prefix, .. } => secondary_prefix,
        }
    }
    fn name(&self) -> &str {
        match self {
            Self::Shell => "Shell",
            Self::Alacritty => "Alacritty",
            Self::Foot => "Foot",
            Self::Custom { name, .. } => name,
        }
    }
    fn icon(&self) -> &str {
        match self {
            Self::Shell => "utilities-terminal",
            Self::Alacritty => "Alacritty",
            Self::Foot => "foot",
            Self::Custom { icon, .. } => icon,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    prefix: String,
    shell: Option<String>,
    engines: Vec<Engine>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: ":".to_string(),
            shell: None,
            engines: vec![Engine::Shell],
        }
    }
}

#[init]
fn init(config_dir: RString) -> Config {
    match fs::read_to_string(format!("{}/terminal.ron", config_dir)) {
        Ok(content) => ron::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Terminal".into(),
        icon: "terminal".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, config: &Config) -> RVec<Match> {
    if !input.starts_with(&config.prefix) {
        RVec::new()
    } else {
        config
            .engines
            .iter()
            .filter(|engine| input.strip_prefix(&config.prefix)
                    .expect("Unable to strip prefix from input lines")
                    .starts_with(&engine.secondary_prefix())
            )
            .enumerate()
            .map(|(_, engine)| Match {
                title: engine.name().into(),
                description: if input.trim_start_matches(&config.prefix).trim_start_matches(&engine.secondary_prefix()).is_empty() {
                    ROption::RNone.into()
                } else {
                    ROption::RSome(format!("{}", input.trim_start_matches(&config.prefix).trim_start_matches(&engine.secondary_prefix())).into())
                },
                use_pango: false,
                icon: ROption::RSome(format!("{}", engine.icon()).into()),
                id: ROption::RNone,
            })
            .collect()
    }
}

#[handler]
fn handler(selection: Match, config: &Config) -> HandleResult {

    let engine = config
        .engines
        .iter()
        .find(|engine| engine.name() == selection.title)
        .unwrap();

    if let Err(why) = if selection.description == None.into() {
        Command::new(
            config
                .shell
                .clone()
                .unwrap_or_else(|| {
                    env::var("SHELL").unwrap_or_else(|_| {
                        "The shell could not be determined!".to_string()
                    })
                }))
        .arg("-c")
        .arg(format!("{}", engine.value().replace("{}", "")))
        .spawn()
    } else {
        Command::new(
            config
                .shell
                .clone()
                .unwrap_or_else(|| {
                    env::var("SHELL").unwrap_or_else(|_| {
                        "The shell could not be determined!".to_string()
                    })
                }))
        .arg("-c")
        .arg(format!("{}", engine.value().replace("{}", &&selection.description.unwrap())))
        .spawn()
    }

    {
        println!("Failed to perform anyrun-terminal: {}", why);
    }

    HandleResult::Close
}
