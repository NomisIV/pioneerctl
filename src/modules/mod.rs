use structopt::StructOpt;

use std::error::Error;
use std::fmt;

mod input;
mod listening_mode;
mod mute;
mod power;
mod volume;

use crate::zone::Zone;

pub trait Module {
    fn parse_command(&self) -> Result<String, ParseCommandError>;
    fn parse_response(&self, code: &str) -> Option<String>;
}

trait Zoned {
    fn get_code(&self) -> String;
}

#[derive(Debug)]
pub struct ParseCommandError {
    msg: String,
}

impl ParseCommandError {
    pub fn new(msg: &str) -> ParseCommandError {
        ParseCommandError {
            msg: msg.to_string(),
        }
    }
}

impl Error for ParseCommandError {}

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing command: {}", &self.msg)
    }
}

#[derive(Debug, StructOpt)]
pub enum Modules {
    Power(power::PowerOpt),
    Volume(volume::VolumeOpt),
    Mute(mute::MuteOpt),
    Input(input::InputOpt),
    ListeningMode(listening_mode::ListeningModeOpt),
}

pub fn parse_command(cmd: &Modules, zone: &Zone) -> Result<String, ParseCommandError> {
    match cmd {
        Modules::Power(power_command) => power::PowerModule::new(zone)
            .add_cmd(power_command)
            .parse_command(),
        Modules::Volume(volume_command) => volume::VolumeModule::new(zone)
            .add_cmd(volume_command)
            .parse_command(),
        Modules::Input(input_command) => input::InputModule::new(zone)
            .add_cmd(input_command)
            .parse_command(),
        Modules::Mute(mute_command) => mute::MuteModule::new(zone)
            .add_cmd(mute_command)
            .parse_command(),
        Modules::ListeningMode(listening_mode_command) => {
            listening_mode::ListeningModeModule::new()
                .add_cmd(listening_mode_command)
                .parse_command()
        }
    }
}

pub fn parse_response(response: &str) -> Option<String> {
    let zone = Zone::Main;
    let modules: Vec<Box<dyn Module>> = vec![
        Box::new(power::PowerModule::new(&zone)),
        Box::new(volume::VolumeModule::new(&zone)),
        Box::new(input::InputModule::new(&zone)),
        Box::new(mute::MuteModule::new(&zone)),
        Box::new(listening_mode::ListeningModeModule::new()),
    ];
    for module in modules {
        if let Some(msg) = module.parse_response(response) {
            return Some(msg);
        }
    }
    None
}
