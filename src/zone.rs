use std::str::FromStr;

use structopt::clap::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Zone {
    Main,
    Zone2,
    Zone3,
    HDZone,
}

impl FromStr for Zone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "main" => Ok(Self::Main),
            "zone2" => Ok(Self::Zone2),
            "zone3" => Ok(Self::Zone3),
            "hdzone" => Ok(Self::HDZone),
            _ => Err(Error {
                message: format!("{} is not a zone", s),
                kind: structopt::clap::ErrorKind::InvalidValue,
                info: Some(vec!["See pioneerctl --help for available zones".into()]),
            }),
        }
    }
}
