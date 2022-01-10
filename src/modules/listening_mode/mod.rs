use structopt::StructOpt;

use super::Module;
use super::ParseCommandError;

mod listening_modes;
use listening_modes::ListeningModes;

#[derive(Debug, StructOpt, Clone)]
pub enum ListeningModeOpt {
    /// Set the listening mode
    Set(ListeningModes),

    /// Get the current listening mode
    Get,

    /// Get the current listening mode
    GetDisplay,
}

pub struct ListeningModeModule {
    cmd: Option<ListeningModeOpt>,
}

impl ListeningModeModule {
    pub fn new() -> ListeningModeModule {
        ListeningModeModule { cmd: None }
    }

    pub fn add_cmd(mut self, cmd: &ListeningModeOpt) -> Self {
        self.cmd = Some(cmd.to_owned());
        self
    }
}

impl Module for ListeningModeModule {
    fn parse_command(&self) -> Result<String, ParseCommandError> {
        self.cmd
            .clone()
            .ok_or(ParseCommandError::new("No command supplied"))
            .map(|cmd| match cmd {
                ListeningModeOpt::Set(mode) => format!("{:0>4}SR", mode.to_code()),
                ListeningModeOpt::Get => "?S".into(),
                ListeningModeOpt::GetDisplay => "?L".into(),
            })
    }

    fn parse_response(&self, code: &str) -> Option<String> {
        if code.starts_with("SR") {
            todo!()
        } else if code.starts_with("LM") {
            todo!()
        } else {
            None
        }
    }
}
