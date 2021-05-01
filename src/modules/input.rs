use super::{Module, Zone, Zoned};

pub struct InputModule;

impl InputModule {
    pub fn new() -> Self {
        InputModule
    }

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
                _ => 99,
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
                _ => 99,
            }
        )
    }
}

impl Module for InputModule {
    fn match_command(&self, cmd: &Vec<String>, zone: &Zone) -> Option<String> {
        if cmd.get(0).unwrap().as_str() != "input" {
            return None;
        }

        let code = InputModule::get_code(zone);

        let cmd = match cmd.get(1).unwrap().as_str() {
            "next" => format!("{}U", code),
            "prev" => format!("{}D", code),
            "set" => match zone {
                Zone::Main => {
                    let source = InputModule::match_source(cmd.get(2).unwrap().as_str());

                    format!("{}{}N", source, code)
                },
                Zone::HDZone => {
                    let source = InputModule::match_source_zoned(cmd.get(2).unwrap().as_str());

                    format!("{}{}A", source, code)
                }
                _ => {
                    let source = InputModule::match_source_zoned(cmd.get(2).unwrap().as_str());

                    format!("{}{}", source, code)
                }
            },
            _ => format!("?{}", &code),
        };
        Some(cmd)
    }

    fn on_response(&self, code: &str) {
        match code {
            _ => (),
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
