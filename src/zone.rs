use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
pub enum Zone {
    Main,
    Zone2,
    Zone3,
    HDZone,
}

pub fn match_zone(args: &mut Vec<String>) -> (Zone, Vec<String>) {
    match args.get(0).unwrap().as_str() {
        "zone2" => (Zone::Zone2, args.drain(1..).collect()),
        "zone3" => (Zone::Zone3, args.drain(1..).collect()),
        "hdzone" => (Zone::HDZone, args.drain(1..).collect()),
        _ => (Zone::Main, args.clone()),
    }
}

impl FromStr for Zone {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "main" => Ok(Self::Main),
            "zone2" => Ok(Self::Zone2),
            "zone3" => Ok(Self::Zone3),
            "hdzone" => Ok(Self::HDZone),
            _ => Ok(Self::Main), // FIXME
        }
    }
}
