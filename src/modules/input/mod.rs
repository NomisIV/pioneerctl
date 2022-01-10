use structopt::StructOpt;

use super::Module;
use super::ParseCommandError;
use super::Zone;
use super::Zoned;

mod sources;
use sources::Sources;

#[derive(Debug, StructOpt, Clone)]
pub enum InputOpt {
    /// Switch to the next input source
    Next,

    /// Switch to the previous input source
    Prev,

    /// Set the specific input source
    Set(Sources), // This only supports non-zoned sources for now

    /// Get the current input source
    Get,
}

pub struct InputModule {
    cmd: Option<InputOpt>,
    zone: Zone,
}

impl InputModule {
    pub fn new(zone: &Zone) -> InputModule {
        InputModule {
            cmd: None,
            zone: zone.to_owned(),
        }
    }

    pub fn add_cmd(mut self, cmd: &InputOpt) -> Self {
        self.cmd = Some(cmd.to_owned());
        self
    }
}

impl Module for InputModule {
    fn parse_command(&self) -> Result<String, ParseCommandError> {
        let code = self.get_code();

        if self.cmd.is_none() {
            return Err(ParseCommandError::new("No command supplied"));
        }

        let cmd = match self.cmd.clone().unwrap() {
            InputOpt::Next => format!("{}U", code),
            InputOpt::Prev => format!("{}D", code),
            InputOpt::Set(source) => match self.zone {
                Zone::Main => format!("{}{}N", source.to_code(), code),
                Zone::HDZone => {
                    if !source.is_zoned() {
                        return Err(ParseCommandError::new(&format!(
                            "{:#?} is not a zoned source",
                            source
                        )));
                    }
                    format!("{}{}A", source.to_code(), code)
                }
                _ => {
                    if !source.is_zoned() {
                        return Err(ParseCommandError::new(&format!(
                            "{:#?} is not a zoned source",
                            source
                        )));
                    }
                    format!("{}{}", source.to_code(), code)
                }
            },
            InputOpt::Get => format!("?{}", &code),
        };

        Ok(cmd)
    }

    fn parse_response(&self, code: &str) -> Option<String> {
        if code.starts_with("FN") {
            let input_code = u8::from_str_radix(code.to_string().get(2..).unwrap(), 10).unwrap();
            Sources::from_code(input_code).map(|s| String::from(format!("{:?}", s)))
        } else {
            None
        }
    }
}

impl Zoned for InputModule {
    fn get_code(&self) -> String {
        match self.zone {
            Zone::Main => "F",
            Zone::Zone2 => "ZS",
            Zone::Zone3 => "ZT",
            Zone::HDZone => "ZE", // This one is a bitch
        }
        .to_string()
    }
}
