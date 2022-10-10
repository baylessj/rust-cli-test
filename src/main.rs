mod command_helpers;

use clap::{Parser, Subcommand};
use command_helpers::{display_error, make, new_project, vexcom_command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enables JSON output for machine parsing
    #[clap(short, long, value_parser, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum ConductorCommands {
    New,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project
    Make,

    /// Example VEXCOM call
    VexcomTest,

    /// Super Basic Conductor rewrite
    Conductor {
        #[command(subcommand)]
        command: ConductorCommands,
    },
}

fn main() {
    let args = Cli::parse();

    let command_result = match args.command {
        Commands::Make => make(),
        Commands::VexcomTest => vexcom_command("test"),
        Commands::Conductor {
            command: ConductorCommands::New,
        } => new_project(),
    };

    match command_result {
        Ok(_) => println!("Success"),
        Err(e) => display_error(args.json, "Command Failed", e.to_string()),
    }
}
