use structopt::StructOpt;

use super::Module;
use super::{Zone, Zoned};

#[derive(Debug, StructOpt)]
pub enum PowerOpt {
    /// Turns the reciever on
    On,

    /// Turns the reciever off
    Off,

    /// Toggles the power state of the reciever
    Toggle,

    /// Queries the power state of the reciever
    Query,
}

pub struct PowerModule;

impl PowerModule {
    pub fn parse_command(cmd: &PowerOpt, zone: &Zone) -> String {
        let code = PowerModule::get_code(zone);

        match cmd {
            PowerOpt::On => format!("{}O", &code),
            PowerOpt::Off => format!("{}F", &code),
            PowerOpt::Toggle => format!("{}Z", &code),
            PowerOpt::Query => format!("?{}", &code),
        }
    }
}

impl Module for PowerModule {
    fn parse_response(&self, code: &str) -> Option<String> {
        match code {
            "PWR1" => Some("Power off".into()),
            "PWR0" => Some("Power on".into()),
            _ => None,
        }
    }
}

impl Zoned for PowerModule {
    fn get_code(zone: &Zone) -> String {
        match zone {
            Zone::Main => "P",
            Zone::Zone2 => "AP",
            Zone::Zone3 => "BP",
            Zone::HDZone => "ZE",
        }
        .to_string()
    }
}
