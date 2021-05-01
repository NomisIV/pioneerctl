use std::env;

mod connection;
use connection::Connection;

mod zone;
use modules::init_modules;
use zone::{match_zone, Zone};

mod modules;

fn main() {
    let con = Connection::new("192.168.1.3:8102");

    let modules = init_modules();
    let mut args: Vec<String> = env::args().skip(1).collect();
    // println!("{:#?}", args);
    let (zone, new_args) = match_zone(&mut args);
    // println!("{:#?}, {:#?}", zone, new_args);

    for module in modules {
        let cmd = module.match_command(&new_args, &zone);

        if let Some(i) = cmd {
            con.send_command(i.as_str());
            break;
        }
    }
}

pub fn log(message: &str) {
    println!("{}", message)
}
