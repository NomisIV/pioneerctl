use colored::*;
use die_exit::*;
use rustyline::{error::ReadlineError, Editor};
use structopt::StructOpt;

use std::io;
use std::process::exit;
use std::str::FromStr;

mod connection;
mod modules;
mod opt;
mod zone;

use connection::{connect, SocketConnection};
use modules::{parse_command, Modules, ParseCommandError};
use opt::Opt;
use zone::Zone;

fn parse_direct_command(opt: &Opt, con: &mut Box<dyn SocketConnection>, cmd: &Modules) {
    // Parse command
    let command = match parse_command(&cmd, &opt.zone) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Send command
    con.send(&command).unwrap();

    // Print response
    if opt.listen {
        con.start_listen();
    }
}

fn parse_repl_line(con: &mut Box<dyn SocketConnection>, line: String) {
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

    // The first argument is supposed to be the program
    module_vec.insert(0, "pioneerctl");

    // Generate the options with structopt
    match Modules::from_iter_safe(&module_vec) {
        // If ok, send the command
        // Ok(cmd) => send_command(stream_maybe, &parse_command(&cmd, &zone)),
        Ok(cmd) => {
            con.send(&parse_command(&cmd, &zone).die("Bad cmd"))
                .expect("Oh no");
        }

        // If bad command, print err
        Err(err) => println!("\n{}\n", err.message),
    }
}

fn repl(opt: &Opt, con: &mut Box<dyn SocketConnection>) {
    if opt.listen {
        con.start_listen();
    }

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(&format!("{} $ ", "pioneerctl".bold().purple()));
        match readline {
            Ok(line) => match line.as_str() {
                "" => continue,
                "exit" => break,
                _ => parse_repl_line(con, line),
            },

            Err(ReadlineError::Interrupted) => break,

            Err(ReadlineError::Eof) => break,

            Err(err) => {
                eprintln!("REPL ERROR: {:?}", err);
                exit(1);
            }
        }
    }
}

fn main() -> Result<(), ParseCommandError> {
    // Read command line arguments
    let opt = Opt::from_args();

    // Generate completions
    if let Some(shell) = opt.completions {
        let app = Opt::clap();
        let app_name = app.get_name();
        app.clone()
            .gen_completions_to(app_name, shell, &mut io::stdout());
        return Ok(());
    }

    // Set up connection
    let mut con: Box<dyn SocketConnection> =
        connect(&opt.ip.clone().die("No IP address configured"), opt.pretend);

    // Check if command is supplied
    match &opt.cmd {
        Some(cmd) => parse_direct_command(&opt, &mut con, &cmd),
        None => repl(&opt, &mut con),
    }

    Ok(())
}
