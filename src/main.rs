mod command_helpers;
mod new_project;

use clap::{Parser, Subcommand};
use command_helpers::{display_error, make, vexcom_command};
use new_project::new_project;

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
    /// Create a New Project. Alias: "n"
    #[command(alias("n"))]
    New { directory: String },
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project
    #[command(alias("m"))]
    Make,

    /// Example VEXCOM call
    VexcomTest,

    /// Super Basic Conductor rewrite
    #[command(alias("c"))]
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
            command: ConductorCommands::New { directory },
        } => new_project(&directory),
    };

    match command_result {
        Ok(_) => println!("Success"),
        Err(e) => display_error(args.json, "Command Failed", e.to_string()),
    }
}
