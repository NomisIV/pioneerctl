use structopt::StructOpt;

use super::Module;
use super::{Zone, Zoned};

#[derive(Debug, StructOpt)]
pub enum InputOpt {
    Next,
    Prev,
    Set { source: String },
    Query,
}

pub struct InputModule;

impl InputModule {
    // TODO: Convert these to FromStr to a Source enum?
    pub fn match_source(source: &str) -> String {
        format!(
            "{:0>2}",
            match source {
                "bd" => 25,
                "dvd" => 4,
                "sat/cbl" => 6,
                "dvr/bdr" => 15,
                "hdmi1" => 19,
                "hdmi2" => 20,
                "hdmi3" => 21,
                "hdmi4" => 22,
                "hdmi5" => 23,
                "hdmi6" => 24,
                "hdmi7" => 34,
                "network" => 26,
                "internet-radio" => 38,
                "spotify" => 53, // Is this even supported anymore?
                "pandora" => 41,
                "media-server" => 44,
                "favorites" => 45,
                "ipod-usb" => 17,
                "tv" => 5,
                "cd" => 1,
                "usb-dac" => 13,
                "tuner" => 2,
                "phono" => 0,
                "multi-channel" => 12,
                "bluetooth" => 33, // BT AUDIO
                "hdmi" => 31,      // HDMI (cyclic)
                "airplay" => 46,   // Information only
                "dmr" => 47,       // Information only
                _ => 99, // ERROR
            }
        )
    }

    pub fn match_source_zoned(source: &str) -> String {
        format!(
            "{:0>2}",
            match source {
                "dvd" => 4,
                "sat/cbl" => 6,
                "dvr/bdr" => 15,
                "network" => 26,
                "internet-radio" => 38,
                "spotify" => 53, // Is this even supported anymore?
                "pandora" => 41,
                "media-server" => 44,
                "favorites" => 45,
                "ipod-usb" => 17,
                "usb-dac" => 13,
                "tv" => 5,
                "cd" => 1,
                "tuner" => 2,
                "bluetooth" => 33, // BT AUDIO
                "airplay" => 46,   // Information only
                "dmr" => 47,       // Information only
                _ => 99, // ERROR
            }
        )
    }

    pub fn parse_command(cmd: &InputOpt) -> String {
        let code = InputModule::get_code(&Zone::Main);

        match cmd {
            InputOpt::Next => format!("{}U", code),
            InputOpt::Prev => format!("{}D", code),
            InputOpt::Set { source } => match Zone::Main {
                Zone::Main => {
                    let source = InputModule::match_source(source);
                    format!("{}{}N", source, code)
                }
                Zone::HDZone => {
                    let source = InputModule::match_source_zoned(source);
                    format!("{}{}A", source, code)
                }
                _ => {
                    let source = InputModule::match_source_zoned(source);
                    format!("{}{}", source, code)
                }
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
