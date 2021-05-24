use super::super::log;
use super::Opt;
use super::{Module, Modules};
use super::{Zone, Zoned};

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
}

impl Module for VolumeModule {
    fn parse_command(opt: &Opt) -> String {
        let code = VolumeModule::get_code(&opt.zone);

        match opt.cmd {
            Some(Modules::Volume(VolumeOpt::Up)) => format!("{}U", &code),
            Some(Modules::Volume(VolumeOpt::Down)) => format!("{}D", &code),
            // TODO
            // There is a way to get the volume from opt, but I don't feel like looking it up right
            // now
            // "set" => match zone {
            //     Zone::Main => {
            //         let volume =
            //             VolumeModule::translate_volume(cmd.get(2).unwrap().parse::<f32>().unwrap());
            //         Some(format!("{:0>3}{}L", volume, &code))
            //     }
            //
            //     _ => {
            //         let volume = VolumeModule::translate_volume_zoned(
            //             cmd.get(2).unwrap().parse::<f32>().unwrap(),
            //         );
            //         Some(format!("{:0>2}{}", volume, &code))
            //     }
            // },
            Some(Modules::Volume(VolumeOpt::Query)) => format!("?{}", &code),
            _ => format!("?{}", &code),
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

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum VolumeOpt {
    Up,
    Down,
    Set { volume: i8 },
    Query,
}
