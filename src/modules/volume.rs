use structopt::clap::AppSettings;
use structopt::StructOpt;

use super::Module;
use super::ParseCommandError;
use super::Zone;
use super::Zoned;

#[derive(Debug, StructOpt, Clone)]
pub enum VolumeOpt {
    /// Raise the volume one step
    Up,

    /// Lower the volume one step
    Down,

    /// Set the specific volume
    #[structopt(alias = "edit", setting = AppSettings::AllowNegativeNumbers)]
    Set { volume: f32 },

    /// Get the current volume
    Get,
}

pub struct VolumeModule {
    cmd: Option<VolumeOpt>,
    zone: Zone,
    min_vol: f32,
    max_vol: f32,
    step: f32,
}

impl VolumeModule {
    pub fn new(zone: &Zone) -> VolumeModule {
        let (min_vol, max_vol, step) = match zone {
            Zone::Main => (-80.0, 12.0, 0.5),
            _ => (-80.0, 0.0, 1.0),
        };

        VolumeModule {
            cmd: None,
            zone: zone.to_owned(),
            min_vol,
            max_vol,
            step,
        }
    }

    pub fn add_cmd(mut self, cmd: &VolumeOpt) -> Self {
        self.cmd = Some(cmd.to_owned());
        self
    }

    fn encode_volume(&self, vol: &f32, zone_code: &str) -> String {
        let mut vol = vol.clamp(self.min_vol, self.max_vol);
        vol = (vol * self.step).round() / self.step;
        let vol_code = ((vol - self.min_vol) / self.step) as u8 + 1;
        match self.zone {
            Zone::Main => format!("{:0>3}{}L", vol_code, zone_code),
            _ => format!("{:0>2}{}", vol_code, zone_code),
        }
    }

    fn decode_volume(&self, vol: &str) -> f32 {
        let code = u8::from_str_radix(vol.to_string().get(3..).unwrap(), 10).unwrap();
        (code - 1) as f32 * self.step + self.min_vol
    }
}

impl Module for VolumeModule {
    fn parse_command(&self) -> Result<String, ParseCommandError> {
        let code = self.get_code();

        self.cmd
            .clone()
            .ok_or(ParseCommandError::new("No command supplied"))
            .map(|cmd| match cmd {
                VolumeOpt::Up => format!("{}U", &code),
                VolumeOpt::Down => format!("{}D", &code),
                VolumeOpt::Set { volume } => self.encode_volume(&volume, &code),
                VolumeOpt::Get => format!("?{}", &code),
            })
    }

    fn parse_response(&self, code: &str) -> Option<String> {
        if code.starts_with("VOL") {
            return Some(format!("Volume set to {}", self.decode_volume(code)));
        }

        match code {
            "VU" => Some("Volume up".into()),
            "VD" => Some("Volume down".into()),
            _ => None,
        }
    }
}

impl Zoned for VolumeModule {
    fn get_code(&self) -> String {
        match self.zone {
            Zone::Main => "V",
            Zone::Zone2 => "Z",
            Zone::Zone3 => "Y",
            Zone::HDZone => "HZ",
        }
        .to_string()
    }
}
