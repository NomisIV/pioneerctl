use super::{Module, Zone, Zoned};
use crate::log;

pub struct VolumeModule;

impl VolumeModule {
    pub fn new() -> Self {
        VolumeModule
    }

    fn translate_volume(vol: f32) -> u8 {
        let min_vol: f32 = -80.0;
        let max_vol: f32 = 12.0;
        let step: f32 = 0.5;
        if vol >= min_vol && vol <= max_vol {
            ((vol - min_vol) / step) as u8 + 1
        } else {
            0
        }
    }

    // These should probably be merged somehow
    fn translate_volume_zoned(vol: f32) -> u8 {
        let min_vol: f32 = -80.0;
        let max_vol: f32 = 0.0;
        let step: f32 = 1.0;
        if vol >= min_vol && vol <= max_vol {
            ((vol - min_vol) / step) as u8 + 1
        } else {
            0
        }
    }
}

impl Module for VolumeModule {
    fn match_command(&self, cmd: &Vec<String>, zone: &Zone) -> Option<String> {
        if cmd.get(0).unwrap().as_str() != "volume" {
            return None;
        }

        let code = VolumeModule::get_code(&zone);

        match cmd.get(1).unwrap().as_str() {
            "up" => Some(format!("{}U", &code)),

            "down" => Some(format!("{}D", &code)),

            "set" => match zone {
                Zone::Main => {
                    let volume =
                        VolumeModule::translate_volume(cmd.get(2).unwrap().parse::<f32>().unwrap());
                    Some(format!("{:0>3}{}L", volume, &code))
                }

                _ => {
                    let volume = VolumeModule::translate_volume_zoned(
                        cmd.get(2).unwrap().parse::<f32>().unwrap(),
                    );
                    Some(format!("{:0>2}{}", volume, &code))
                }
            },

            _ => Some(format!("?{}", &code)),
        }
    }

    fn on_response(&self, code: &str) {
        match code {
            "VU" => log("Volume up"),
            "VD" => log("Volume down"),
            _ => (),
        }
    }
}

impl Zoned for VolumeModule {
    fn get_code(zone: &Zone) -> String {
        match zone {
            Zone::Main => "V",
            Zone::Zone2 => "Z",
            Zone::Zone3 => "Y",
            Zone::HDZone => "HZ",
        }
        .to_string()
    }
}
