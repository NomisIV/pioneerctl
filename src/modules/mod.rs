mod input;
mod mute;
mod power;
mod volume;

use super::zone::Zone;

pub trait Module {
    fn parse_command(opt: &Opt) -> String;
    fn on_response(&self, code: &str);
}

trait Zoned {
    fn get_code(zone: &Zone) -> String;
}

use super::Opt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Modules {
    Power(power::PowerOpt),
    Volume(volume::VolumeOpt),
    Mute(mute::MuteOpt),
    Input(input::InputOpt),
}

pub fn parse_command(opt: &Opt) -> Option<String> {
    match opt.cmd {
        Some(Modules::Power(..)) => Some(power::PowerModule::parse_command(opt)),
        Some(Modules::Input(..)) => Some(input::InputModule::parse_command(opt)),
        Some(Modules::Mute(..)) => Some(mute::MuteModule::parse_command(opt)),
        Some(Modules::Volume(..)) => Some(volume::VolumeModule::parse_command(opt)),
        _ => None,
    }
}
