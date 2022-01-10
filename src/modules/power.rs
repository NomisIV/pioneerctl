use structopt::StructOpt;

use super::Module;
use super::ParseCommandError;
use super::Zone;
use super::Zoned;

#[derive(Debug, StructOpt, Clone)]
pub enum PowerOpt {
    /// Turn the reciever on
    On,

    /// Turn the reciever off
    Off,

    /// Toggle the power state of the reciever
    Toggle,

    /// Get the power state of the reciever
    Get,
}

pub struct PowerModule {
    cmd: Option<PowerOpt>,
    zone: Zone,
}

impl PowerModule {
    pub fn new(zone: &Zone) -> PowerModule {
        PowerModule {
            cmd: None,
            zone: zone.to_owned(),
        }
    }

    pub fn add_cmd(mut self, cmd: &PowerOpt) -> Self {
        self.cmd = Some(cmd.to_owned());
        self
    }
}

impl Module for PowerModule {
    fn parse_command(&self) -> Result<String, ParseCommandError> {
        let code = self.get_code();

        self.cmd
            .clone()
            .ok_or(ParseCommandError::new("No command supplied"))
            .map(|cmd| match cmd {
                PowerOpt::On => format!("{}O", &code),
                PowerOpt::Off => format!("{}F", &code),
                PowerOpt::Toggle => format!("{}Z", &code),
                PowerOpt::Get => format!("?{}", &code),
            })
    }

    fn parse_response(&self, code: &str) -> Option<String> {
        match code {
            "PWR1" => Some("Power off".into()),
            "PWR0" => Some("Power on".into()),
            _ => None,
        }
    }
}

impl Zoned for PowerModule {
    fn get_code(&self) -> String {
        match self.zone {
            Zone::Main => "P",
            Zone::Zone2 => "AP",
            Zone::Zone3 => "BP",
            Zone::HDZone => "ZE",
        }
        .to_string()
    }
}
