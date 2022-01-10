use structopt::clap::Shell;
use structopt::StructOpt;

use crate::Modules;
use crate::Zone;

#[derive(Debug, StructOpt)]
#[structopt(name = "pioneerctl", author = "NomisIV")]
/// Control (some) pioneer recievers directly from the terminal
pub struct Opt {
    /// Generate shell completions
    #[structopt(short, long, value_name = "SHELL")]
    pub completions: Option<Shell>,

    /// Print which commands would be sent without sending them (useful for debugging)
    #[structopt(short, long)]
    pub pretend: bool,

    /// Print reciever response messages (does nothing with pretend enabled)
    #[structopt(short, long)]
    pub listen: bool,

    /// IP address (overrides config)
    #[structopt(
        short = "a",
        long,
        value_name = "IP_ADDRESS",
        env = "PIONEERCTL_ADDRESS"
    )]
    pub ip: Option<String>,

    /// Zone (one of main, zone2, zone3 and hdzone)
    #[structopt(short, long, default_value = "main")]
    pub zone: Zone,

    #[structopt(subcommand)]
    pub cmd: Option<Modules>,
}
