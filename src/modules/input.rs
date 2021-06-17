use structopt::StructOpt;

use super::Module;
use super::{Zone, Zoned};

mod sources;
use sources::{Sources, ToCode, ToZonedSource};

#[derive(Debug, StructOpt)]
pub enum InputOpt {
    Next,
    Prev,
    Set { source: Sources }, // This only supports non-zoned sources for now
    Query,
}

pub struct InputModule;

impl InputModule {
    pub fn parse_command(cmd: &InputOpt, zone: &Zone) -> String {
        let code = InputModule::get_code(zone);

        match cmd {
            InputOpt::Next => format!("{}U", code),
            InputOpt::Prev => format!("{}D", code),
            InputOpt::Set { source } => match zone {
                Zone::Main => format!("{}{}N", source.to_code(), code),
                Zone::HDZone => format!(
                    "{}{}A",
                    source
                        .to_zoned()
                        .expect("invalid zoned input source") // TODO: Make better
                        .to_code(),
                    code
                ),
                _ => format!(
                    "{}{}",
                    source
                        .to_zoned()
                        .expect("invalid zoned input source") // TODO: Make better
                        .to_code(),
                    code
                ),
            },
            InputOpt::Query => format!("?{}", &code),
        }
    }
}

impl Module for InputModule {
    fn parse_response(&self, code: &str) -> Option<String> {
        match code {
            // TODO: Implement this
            _ => None,
        }
    }
}

impl Zoned for InputModule {
    fn get_code(zone: &Zone) -> String {
        match zone {
            Zone::Main => "F",
            Zone::Zone2 => "ZS",
            Zone::Zone3 => "ZT",
            Zone::HDZone => "ZE", // This one is a bitch
        }
        .to_string()
    }
}
