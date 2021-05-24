use super::super::log;
use super::Opt;
use super::{Module, Modules};
use super::{Zone, Zoned};

pub struct PowerModule;

impl Module for PowerModule {
    fn parse_command(opt: &Opt) -> String {
        let code = PowerModule::get_code(&opt.zone);

        match opt.cmd {
            Some(Modules::Power(PowerOpt::On)) => format!("{}O", &code),
            Some(Modules::Power(PowerOpt::Off)) => format!("{}F", &code),
            Some(Modules::Power(PowerOpt::Toggle)) => format!("{}Z", &code),
            Some(Modules::Power(PowerOpt::Query)) => format!("?{}", &code),
            _ => "".into(), // FIXME
        }
    }

    fn on_response(&self, code: &str) {
        match code {
            "PWR1" => log("Power off"),
            "PWR0" => log("Power on"),
            _ => (),
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

use structopt::StructOpt;

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
