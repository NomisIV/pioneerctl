mod input;
mod mute;
mod power;
mod volume;

use super::zone::Zone;

pub trait Module {
    // I want to use this, but I want to be able to specify the module's own
    // opt instead of a generic opt as parameter
    // fn parse_command(cmd: &Modules) -> String;
    fn parse_response(&self, code: &str) -> Option<String>;
}

trait Zoned {
    fn get_code(zone: &Zone) -> String;
}

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Modules {
    Power(power::PowerOpt),
    Volume(volume::VolumeOpt),
    Mute(mute::MuteOpt),
    Input(input::InputOpt),
}

pub fn parse_command(cmd: &Modules) -> String {
    match cmd {
        Modules::Power(power_command) => power::PowerModule::parse_command(power_command),
        Modules::Volume(volume_command) => volume::VolumeModule::parse_command(volume_command),
        Modules::Input(input_command) => input::InputModule::parse_command(input_command),
        Modules::Mute(mute_command) => mute::MuteModule::parse_command(mute_command),
    }
}
