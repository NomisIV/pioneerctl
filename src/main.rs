use std::{fs, path::PathBuf};
use structopt::StructOpt;
use toml_edit::Document;

mod connection;
use connection::Connection;

mod zone;
use zone::Zone;

mod modules;
use modules::{parse_command, Modules};

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Don't print reciever on_response
    #[structopt(short, long)]
    silent: bool,

    /// Config file
    #[structopt(
        short,
        long,
        parse(from_os_str),
        // default_value = "$XDG_CONFIG_HOME/pioneerctl/config.toml"
        default_value = "/home/simon/.config/pioneerctl/config.toml"
    )]
    config: PathBuf,

    /// IP address (overrides config)
    #[structopt(short = "a", long)]
    ip_address: Option<String>,

    /// Zone
    #[structopt(short, long, default_value = "main")]
    zone: Zone,

    #[structopt(subcommand)]
    cmd: Option<Modules>,
}

fn main() {
    // Read command line arguments
    let opt = Opt::from_args();

    // Read configuration file
    if opt.debug {
        println!("Path to config file: {}", opt.config.to_str().unwrap());
    }
    let cfg_raw = fs::read_to_string(&opt.config).expect("Could not read config file");
    let cfg = cfg_raw.parse::<Document>().expect("Bad config file");

    // Read ip address from config file
    let ip_address = match opt.ip_address {
        Some(..) => opt.ip_address.clone().unwrap(),
        None => format!(
            "{}:{}",
            cfg["reciever"]["ip_address"]
                .as_str()
                .expect("No address to reciever")
                .to_string(),
            cfg["reciever"]["port"]
                .as_integer()
                .expect("No port to reciever")
                .to_string(),
        ),
    };

    // Parse command
    let code = parse_command(&opt);

    if code.is_none() {
        println!("No command, exiting");
        return;
    }

    // Set up connection
    if opt.debug {
        println!("Connecting to address: {}", ip_address);
    }
    let con = Connection::new(&ip_address);

    // Send command
    con.send_command(code.unwrap().as_str());

    // Print response
}

pub fn log(message: &str) {
    println!("{}", message)
}
