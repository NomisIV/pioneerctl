use structopt::StructOpt;

#[derive(Debug, Clone, Copy, StructOpt, PartialEq)]
pub enum Sources {
    BD,
    /// zoned
    DVD,
    /// zoned
    SatCbl,
    /// zoned
    DvrBdr,
    HDMI1,
    HDMI2,
    HDMI3,
    HDMI4,
    HDMI5,
    HDMI6,
    HDMI7,
    /// zoned
    Network,
    /// zoned
    InternetRadio,
    /// zoned
    Spotify,
    /// zoned
    Pandora,
    /// zoned
    MediaServer,
    /// zoned
    Favorites,
    /// zoned
    IpodUsb,
    /// zoned
    TV,
    /// zoned
    CD,
    /// zoned
    UsbDac,
    /// zoned
    Tuner,
    Phono,
    MultiChannel,
    /// zoned
    Bluetooth,
    HDMI,
    /// zoned
    Airplay,
    /// zoned
    DMR,
}

impl Sources {
    // This is kinda dumb, but I need a two-way HashMap
    fn code_map() -> Vec<(Self, u8)> {
        vec![
            (Self::BD, 25),
            (Self::DVD, 4),
            (Self::SatCbl, 6),
            (Self::DvrBdr, 15),
            (Self::HDMI1, 19),
            (Self::HDMI2, 20),
            (Self::HDMI3, 21),
            (Self::HDMI4, 22),
            (Self::HDMI5, 23),
            (Self::HDMI6, 24),
            (Self::HDMI7, 25),
            (Self::Network, 26),
            (Self::InternetRadio, 38),
            (Self::Spotify, 53),
            (Self::Pandora, 41),
            (Self::MediaServer, 44),
            (Self::Favorites, 45),
            (Self::IpodUsb, 17),
            (Self::TV, 5),
            (Self::CD, 1),
            (Self::UsbDac, 13),
            (Self::Tuner, 2),
            (Self::Phono, 0),
            (Self::MultiChannel, 12),
            (Self::Bluetooth, 33),
            (Self::HDMI, 31),
            (Self::Airplay, 46),
            (Self::DMR, 47),
        ]
    }

    pub fn to_code(&self) -> u8 {
        Sources::code_map()
            .iter()
            .find(|map| self.eq(&map.0))
            .unwrap()
            .1
    }

    pub fn from_code(code: u8) -> Option<Self> {
        Sources::code_map()
            .iter()
            .find(|map| code == map.1)
            .map(|s| s.0)
    }

    pub fn is_zoned(&self) -> bool {
        [
            Self::DVD,
            Self::SatCbl,
            Self::DvrBdr,
            Self::Network,
            Self::InternetRadio,
            Self::Spotify,
            Self::Pandora,
            Self::MediaServer,
            Self::Favorites,
            Self::IpodUsb,
            Self::TV,
            Self::CD,
            Self::UsbDac,
            Self::Tuner,
            Self::Bluetooth,
            Self::Airplay,
            Self::DMR,
        ]
        .contains(self)
    }
}
