use crate::{log, modules::Module, modules::Zoned, Zone};

pub struct PowerModule;

impl PowerModule {
    pub fn new() -> Self {
        PowerModule
    }
}

impl Module for PowerModule {
    fn match_command(&self, cmd: &Vec<String>, zone: &Zone) -> Option<String> {
        if cmd.get(0).unwrap().as_str() != "power" {
            return None;
        }

        let code = PowerModule::get_code(&zone);

        match cmd.get(1).unwrap().as_str() {
            "on" => Some(format!("{}O", &code)),
            "off" => Some(format!("{}F", &code)),
            "toggle" => Some(format!("{}Z", &code)),
            _ => Some(format!("?{}", &code)),
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
