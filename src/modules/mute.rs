use super::super::log;
use super::Opt;
use super::{Module, Modules};
use super::{Zone, Zoned};

pub struct MuteModule;

impl Module for MuteModule {
    fn parse_command(opt: &Opt) -> String {
        let code = MuteModule::get_code(&opt.zone);

        match opt.cmd {
            Some(Modules::Mute(MuteOpt::On)) => format!("{}O", &code),
            Some(Modules::Mute(MuteOpt::Off)) => format!("{}F", &code),
            Some(Modules::Mute(MuteOpt::Toggle)) => format!("{}Z", &code),
            Some(Modules::Mute(MuteOpt::Query)) => format!("?{}", &code),
            _ => "".into(), // FIXME
        }
    }

    fn on_response(&self, code: &str) {
        match code {
            "MUT1" => log("Mute off"),
            "MUT0" => log("Mute on"),
            _ => (),
        }
    }
}

impl Zoned for MuteModule {
    fn get_code(zone: &Zone) -> String {
        match zone {
            Zone::Main => "M",
            Zone::Zone2 => "Z2M",
            Zone::Zone3 => "Z3M",
            Zone::HDZone => "HZM",
        }
        .to_string()
    }
}

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum MuteOpt {
    /// Mutes the reciever
    On,

    /// Unmutes the reciever
    Off,

    /// Toggles mute state on the reciever
    Toggle,

    /// Queries the mute state of the reciever
    Query,
}
