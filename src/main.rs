mod command_helpers;

use clap::{Parser, Subcommand};
use command_helpers::vexcom_command;

use crate::command_helpers::display_error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enables JSON output for machine parsing
    #[clap(short, long, value_parser, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project
    Make,
}

fn main() {
    let args = Cli::parse();

    // println!("Hello {}!", args.name);
    println!("{}", args.json);

    match vexcom_command("test") {
        Ok(_) => println!("Success"),
        Err(e) => display_error(args.json, "Command Failed", e.to_string()),
    }
}
