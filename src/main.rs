mod command_helpers;

use clap::{Parser, Subcommand};
use command_helpers::{make, vexcom_command};

use crate::command_helpers::display_error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enables JSON output for machine parsing
    #[clap(short, long, value_parser, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project
    Make,

    /// Example VEXCOM call
    VexcomTest,
}

fn main() {
    let args = Cli::parse();

    let command_result = match args.command {
        Commands::Make => make(),
        Commands::VexcomTest => vexcom_command("test"),
    };

    match command_result {
        Ok(_) => println!("Success"),
        Err(e) => display_error(args.json, "Command Failed", e.to_string()),
    }
}
