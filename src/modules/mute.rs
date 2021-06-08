use structopt::StructOpt;

use super::Module;
use super::{Zone, Zoned};

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

pub struct MuteModule;

impl MuteModule {
    pub fn parse_command(cmd: &MuteOpt) -> String {
        let code = MuteModule::get_code(&Zone::Main);

        match cmd {
            MuteOpt::On => format!("{}O", &code),
            MuteOpt::Off => format!("{}F", &code),
            MuteOpt::Toggle => format!("{}Z", &code),
            MuteOpt::Query => format!("?{}", &code),
        }
    }
}

impl Module for MuteModule {
    fn parse_response(&self, code: &str) -> Option<String> {
        match code {
            "MUT1" => Some("Mute off".into()),
            "MUT0" => Some("Mute on".into()),
            _ => None,
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
