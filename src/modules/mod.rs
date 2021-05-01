mod input;
mod mute;
mod power;
mod volume;

use crate::Zone;

pub trait Module {
    fn match_command(&self, cmd: &Vec<String>, zone: &Zone) -> Option<String>;
    fn on_response(&self, code: &str);
}

trait Zoned {
    fn get_code(zone: &Zone) -> String;
}

pub fn init_modules() -> Vec<Box<dyn Module>> {
    let mut vec: Vec<Box<dyn Module>> = Vec::new();

    vec.push(Box::new(power::PowerModule::new()));
    vec.push(Box::new(volume::VolumeModule::new()));
    vec.push(Box::new(mute::MuteModule::new()));
    vec.push(Box::new(input::InputModule::new()));

    return vec;
}
