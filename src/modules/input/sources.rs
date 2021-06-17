use std::str::FromStr;
use structopt::clap::Error;

pub trait ToCode {
    fn to_code(&self) -> u8;
}

pub trait ToZonedSource {
    fn to_zoned(&self) -> Result<SourcesZoned, Error>; // TODO: Make it work with into()
}

#[derive(Debug)]
pub enum Sources {
    BD,
    DVD,
    SatCbl,
    DvrBdr,
    HDMI1,
    HDMI2,
    HDMI3,
    HDMI4,
    HDMI5,
    HDMI6,
    HDMI7,
    Network,
    InternetRadio,
    Spotify,
    Pandora,
    MediaServer,
    Favorites,
    IpodUsb,
    TV,
    CD,
    UsbDac,
    Tuner,
    Phono,
    MultiChannel,
    Bluetooth,
    HDMI,
    Airplay,
    DMR,
}

impl FromStr for Sources {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bd" => Ok(Self::BD),
            "dvd" => Ok(Self::DVD),
            "sat-cbl" => Ok(Self::SatCbl),
            "dvr-bdr" => Ok(Self::DvrBdr),
            "hdmi-1" => Ok(Self::HDMI1),
            "hdmi-2" => Ok(Self::HDMI2),
            "hdmi-3" => Ok(Self::HDMI3),
            "hdmi-4" => Ok(Self::HDMI4),
            "hdmi-5" => Ok(Self::HDMI5),
            "hdmi-6" => Ok(Self::HDMI6),
            "hdmi-7" => Ok(Self::HDMI7),
            "network" => Ok(Self::Network),
            "internet-radio" => Ok(Self::InternetRadio),
            "spotify" => Ok(Self::Spotify),
            "pandora" => Ok(Self::Pandora),
            "media-server" => Ok(Self::MediaServer),
            "favorites" => Ok(Self::Favorites),
            "ipod-usb" => Ok(Self::IpodUsb),
            "tv" => Ok(Self::TV),
            "cd" => Ok(Self::CD),
            "usb-dac" => Ok(Self::UsbDac),
            "tuner" => Ok(Self::Tuner),
            "phono" => Ok(Self::Phono),
            "multi-channel" => Ok(Self::MultiChannel),
            "bluetooth" => Ok(Self::Bluetooth),
            "hdmi" => Ok(Self::HDMI),
            "airplay" => Ok(Self::Airplay),
            "dmr" => Ok(Self::DMR),
            _ => Err(Error {
                message: format!("{} is not an input source", s),
                kind: structopt::clap::ErrorKind::InvalidValue,
                info: None, // TODO
            }),
        }
    }
}

impl ToCode for Sources {
    fn to_code(&self) -> u8 {
        match self {
            Self::BD => 25,
            Self::DVD => 4,
            Self::SatCbl => 6,
            Self::DvrBdr => 15,
            Self::HDMI1 => 19,
            Self::HDMI2 => 20,
            Self::HDMI3 => 21,
            Self::HDMI4 => 22,
            Self::HDMI5 => 23,
            Self::HDMI6 => 24,
            Self::HDMI7 => 25,
            Self::Network => 26,
            Self::InternetRadio => 38,
            Self::Spotify => 53,
            Self::Pandora => 41,
            Self::MediaServer => 44,
            Self::Favorites => 45,
            Self::IpodUsb => 17,
            Self::TV => 5,
            Self::CD => 1,
            Self::UsbDac => 13,
            Self::Tuner => 2,
            Self::Phono => 0,
            Self::MultiChannel => 12,
            Self::Bluetooth => 33,
            Self::HDMI => 31,
            Self::Airplay => 46,
            Self::DMR => 47,
        }
    }
}

impl ToZonedSource for Sources {
    fn to_zoned(&self) -> Result<SourcesZoned, Error> {
        match self {
            // TODO: Macro this
            Self::DVD => Ok(SourcesZoned::DVD),
            Self::SatCbl => Ok(SourcesZoned::SatCbl),
            Self::DvrBdr => Ok(SourcesZoned::DvrBdr),
            Self::Network => Ok(SourcesZoned::Network),
            Self::InternetRadio => Ok(SourcesZoned::InternetRadio),
            Self::Spotify => Ok(SourcesZoned::Spotify),
            Self::Pandora => Ok(SourcesZoned::Pandora),
            Self::MediaServer => Ok(SourcesZoned::MediaServer),
            Self::Favorites => Ok(SourcesZoned::Favorites),
            Self::IpodUsb => Ok(SourcesZoned::IpodUsb),
            Self::TV => Ok(SourcesZoned::TV),
            Self::CD => Ok(SourcesZoned::CD),
            Self::UsbDac => Ok(SourcesZoned::UsbDac),
            Self::Tuner => Ok(SourcesZoned::Tuner),
            Self::Bluetooth => Ok(SourcesZoned::Bluetooth),
            Self::Airplay => Ok(SourcesZoned::Airplay),
            Self::DMR => Ok(SourcesZoned::DMR),
            _ => Err(Error {
                message: format!("{:#?} is not a zoned input source", self),
                kind: structopt::clap::ErrorKind::InvalidValue,
                info: None,
            }),
        }
    }
}

// SourcesZoned enum is a subset of Sources enum
#[derive(Debug)]
pub enum SourcesZoned {
    DVD,
    SatCbl,
    DvrBdr,
    Network,
    InternetRadio,
    Spotify,
    Pandora,
    MediaServer,
    Favorites,
    IpodUsb,
    UsbDac,
    TV,
    CD,
    Tuner,
    Bluetooth,
    Airplay,
    DMR,
}

impl FromStr for SourcesZoned {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dvd" => Ok(Self::DVD),
            "sat-cbl" => Ok(Self::SatCbl),
            "dvr-bdr" => Ok(Self::DvrBdr),
            "network" => Ok(Self::Network),
            "internet-radio" => Ok(Self::InternetRadio),
            "spotify" => Ok(Self::Spotify),
            "pandora" => Ok(Self::Pandora),
            "media-server" => Ok(Self::MediaServer),
            "favorites" => Ok(Self::Favorites),
            "ipod-usb" => Ok(Self::IpodUsb),
            "tv" => Ok(Self::TV),
            "cd" => Ok(Self::CD),
            "usb-dac" => Ok(Self::UsbDac),
            "tuner" => Ok(Self::Tuner),
            "bluetooth" => Ok(Self::Bluetooth),
            "airplay" => Ok(Self::Airplay),
            "dmr" => Ok(Self::DMR),
            _ => Err(Error {
                message: format!("{} is not a zoned input source", s),
                kind: structopt::clap::ErrorKind::InvalidValue,
                info: None,
            }),
        }
    }
}

impl ToCode for SourcesZoned {
    fn to_code(&self) -> u8 {
        match self {
            Self::DVD => 4,
            Self::SatCbl => 6,
            Self::DvrBdr => 15,
            Self::Network => 26,
            Self::InternetRadio => 38,
            Self::Spotify => 53,
            Self::Pandora => 41,
            Self::MediaServer => 44,
            Self::Favorites => 45,
            Self::IpodUsb => 17,
            Self::TV => 5,
            Self::CD => 1,
            Self::UsbDac => 13,
            Self::Tuner => 2,
            Self::Bluetooth => 33,
            Self::Airplay => 46,
            Self::DMR => 47,
        }
    }
}
