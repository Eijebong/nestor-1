use std::fs;
use std::path::{Path, PathBuf};

use failure::Error;
use irc::client::data::Config as IrcConfig;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "connection")]
    pub irc_config: IrcConfig,
    #[serde(rename = "bot")]
    pub bot_settings: BotSettings,
}

#[derive(Deserialize)]
pub struct BotSettings {
    pub admins: Vec<String>,
    pub blacklisted_users: Vec<String>,
    pub database_url: String,
    pub command_indicator: String,
    pub contact: String,
    pub alias_depth: u32,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        // Load entries via serde
        let conf = fs::read_to_string(path.as_ref())?;
        let conf: Config = toml::de::from_str(&conf)?;
        Ok(conf)
    }
}

pub fn is_admin(nick: &str, config: &Config) -> bool {
    config.bot_settings.admins.contains(&nick.into())
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(
        short = "c",
        long = "config",
        parse(from_os_str),
        default_value = "irc.config.toml"
    )]
    pub config: PathBuf,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "import")]
    Import {
        #[structopt(short = "f", long = "file", parse(from_os_str))]
        file: PathBuf,
    },
    #[structopt(name = "export")]
    Export {
        #[structopt(short = "f", long = "file", parse(from_os_str))]
        file: PathBuf,
    },
    #[structopt(name = "launch")]
    Launch {},
}
