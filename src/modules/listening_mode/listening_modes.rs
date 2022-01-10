use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt, PartialEq)]
pub enum ListeningModes {
    // Sometimes many options are too manu options. And these aren't even all of them
    // StereoCyclic,
    Standard,
    // StereoDirect,
    // TwoChannelSource,
    // ProLogic2Movie,
    // ProLogic2Music,
    // ProLogic2Game,
    // ProLogic,
    // Neo6Cinema,
    // Neo6Music,
    // NeoXCinema,
    // NeoXMusic,
    // NeoXGame,
    DolbySurround,
    ExtendedStereo,
    AdvancedSurround,
    Action,
    Drama,
    AdvancedGame,
    Sports,
    Classical,
    RockPop,
    // FrontStageSurroundAdvance,
    // EcoMode,
    // EcoMode1,
    // EcoMode2,
    // RetrieverAir,
    // PhonesSurround,
    // AutoSurrStreamDirect,
    AutoSurround,
    // AutoLevelControl,
    Direct,
    PureDirect,
    OptimumSurround,
}

impl ListeningModes {
    pub fn to_code(&self) -> u8 {
        match self {
            Self::Standard => 10,
            Self::DolbySurround => 40,
            Self::ExtendedStereo => 41,
            Self::AdvancedSurround => 100,
            Self::Action => 101,
            Self::Drama => 103,
            Self::AdvancedGame => 118,
            Self::Sports => 117,
            Self::Classical => 107,
            Self::RockPop => 110,
            Self::AutoSurround => 6,
            Self::Direct => 7,
            Self::PureDirect => 8,
            Self::OptimumSurround => 152,
        }
    }
}
