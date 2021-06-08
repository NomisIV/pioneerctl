use structopt::StructOpt;

use super::Module;
use super::{Zone, Zoned};

#[derive(Debug, StructOpt)]
pub enum VolumeOpt {
    Up,
    Down,
    Set { volume: i8 },
    Query,
}

pub struct VolumeModule;

impl VolumeModule {
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

    pub fn parse_command(cmd: &VolumeOpt) -> String {
        let code = VolumeModule::get_code(&Zone::Main); // TODO: make zone independent

        match cmd {
            VolumeOpt::Up => format!("{}U", &code),
            VolumeOpt::Down => format!("{}D", &code),
            VolumeOpt::Set { volume } => match Zone::Main {
                Zone::Main => {
                    // let volume =
                    //     VolumeModule::translate_volume(cmd.get(2).unwrap().parse::<f32>().unwrap());
                    format!(
                        "{:0>3}{}L",
                        VolumeModule::translate_volume(volume.clone().into()),
                        &code
                    )
                }

                _ => {
                    // let volume = VolumeModule::translate_volume_zoned(
                    //     cmd.get(2).unwrap().parse::<f32>().unwrap(),
                    // );
                    format!(
                        "{:0>2}{}",
                        VolumeModule::translate_volume_zoned(volume.clone().into()),
                        &code
                    )
                }
            },
            VolumeOpt::Query => format!("?{}", &code),
        }
    }
}

impl Module for VolumeModule {
    fn parse_response(&self, code: &str) -> Option<String> {
        match code {
            "VU" => Some("Volume up".into()),
            "VD" => Some("Volume down".into()),
            _ => None,
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
