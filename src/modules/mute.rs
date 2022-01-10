use structopt::StructOpt;

use super::Module;
use super::ParseCommandError;
use super::Zone;
use super::Zoned;

#[derive(Debug, StructOpt, Clone)]
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

pub struct MuteModule {
    cmd: Option<MuteOpt>,
    zone: Zone,
}

impl MuteModule {
    pub fn new(zone: &Zone) -> MuteModule {
        MuteModule {
            cmd: None,
            zone: zone.to_owned(),
        }
    }

    pub fn add_cmd(mut self, cmd: &MuteOpt) -> Self {
        self.cmd = Some(cmd.to_owned());
        self
    }
}

impl Module for MuteModule {
    fn parse_command(&self) -> Result<String, ParseCommandError> {
        let code = self.get_code();

        self.cmd
            .clone()
            .ok_or(ParseCommandError::new("No command supplied"))
            .map(|cmd| match cmd {
                MuteOpt::On => format!("{}O", &code),
                MuteOpt::Off => format!("{}F", &code),
                MuteOpt::Toggle => format!("{}Z", &code),
                MuteOpt::Query => format!("?{}", &code),
            })
    }

    fn parse_response(&self, code: &str) -> Option<String> {
        match code {
            "MUT1" => Some("Mute off".into()),
            "MUT0" => Some("Mute on".into()),
            _ => None,
        }
    }
}

impl Zoned for MuteModule {
    fn get_code(&self) -> String {
        match self.zone {
            Zone::Main => "M",
            Zone::Zone2 => "Z2M",
            Zone::Zone3 => "Z3M",
            Zone::HDZone => "HZM",
        }
        .to_string()
    }
}
