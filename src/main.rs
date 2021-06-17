use colored::*;
use die_exit::*;
use rustyline::{error::ReadlineError, Editor};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
    process::exit,
    str::FromStr,
};
use structopt::StructOpt;

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

    /// IP address (overrides config)
    #[structopt(short = "a", long, env = "PIONEERCTL_ADDRESS")]
    ip_address: Option<String>,

    /// Zone
    #[structopt(short, long, default_value = "main")]
    zone: Zone,

    #[structopt(subcommand)]
    cmd: Option<Modules>,
}

fn send_code(stream: &mut TcpStream, code: &str) {
    stream
        .write(format!("{}\r\n", code).as_bytes())
        .die("Expected to send the command successfully");
}

fn main() {
    // Read command line arguments
    let opt = Opt::from_args();

    // Set up connection
    let mut stream = TcpStream::connect(&opt.ip_address.die("No IP address configured"))
        .die("Could not connect to the reciever");

    // Command supplied, execute it
    if opt.cmd.is_some() {
        // Parse command
        let code = parse_command(&opt.cmd.unwrap(), &opt.zone);

        // Send command
        send_code(&mut stream, &code);

        // Print response
        // TODO: Parse response
        loop {
            let mut data = Vec::new();
            BufReader::new(&stream)
                .read_until(b'\r', &mut data)
                .die("Could not listen");
            println!("{}", String::from_utf8_lossy(&data));
        }

        // Do nothing more
        // return;
    }

    // Enter REPL-interface
    let mut rl = Editor::<()>::new();

    loop {
        // Read
        let readline = rl.readline(&format!("{} $ ", "pioneerctl".bold().purple()));
        match readline {
            Ok(line) => match line.as_str() {
                "" => continue,
                "exit" => break,
                _ => parse_repl(&mut stream, line),
            },

            Err(ReadlineError::Interrupted) => break,

            Err(ReadlineError::Eof) => break,

            Err(err) => {
                println!("REPL ERROR: {:?}", err);
                exit(1);
            }
        }
    }
}

fn parse_repl(stream: &mut TcpStream, line: String) {
    // Split the line at whitespace
    let mut input_vec = line.split(" ").collect::<Vec<&str>>();

    // Parse for zone
    let (mut module_vec, zone) = match Zone::from_str(input_vec.get(0).unwrap()) {
        Ok(zone) => {
            input_vec.remove(0);
            (input_vec, zone)
        }
        Err(..) => (input_vec, Zone::Main),
    };

    // The first word is supposed to be the program
    module_vec.insert(0, "pioneerctl");

    // Generate the command with structopt
    let cmd = Modules::from_iter_safe(module_vec);

    if cmd.is_err() {
        println!("\n{}\n", cmd.unwrap_err().message);
        return;
    }

    // Get the code for the current command
    let code = parse_command(&cmd.unwrap(), &zone);

    send_code(stream, &code);

    // TODO: Print response
}
