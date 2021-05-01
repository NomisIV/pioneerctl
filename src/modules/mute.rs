use crate::{log, modules::Module, modules::Zoned, Zone};

pub struct MuteModule;

impl MuteModule {
    pub fn new() -> Self {
        MuteModule
    }
}

impl Module for MuteModule {
    fn match_command(&self, cmd: &Vec<String>, zone: &Zone) -> Option<String> {
        if cmd.get(0).unwrap().as_str() != "mute" {
            return None;
        }

        let code = MuteModule::get_code(&zone);

        match cmd.get(1).unwrap().as_str() {
            "on" => Some(format!("{}O", &code)),
            "off" => Some(format!("{}F", &code)),
            "toggle" => Some(format!("{}Z", &code)),
            _ => Some(format!("?{}", &code)),
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
