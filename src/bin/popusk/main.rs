#![allow(dead_code)]

mod pcommand;
mod io_ext;
mod parse_cli;
mod parse_cli_command;
mod wacommand;

use clap::Parser;

use parse_cli::CLI;
use parse_cli_command::{parse_cli_command, P_WA_Command};

use popusk::app::App;

use std::process::ExitCode;
use std::path::PathBuf;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();
    let command = parse_cli_command(cli.command);

    match command {
        P_WA_Command::WACommand(wacommand) => wacommand.execute()?,
        P_WA_Command::PCommand(pcommand) => {
            let mut app = App::new(&PathBuf::from("."))?;
            pcommand.execute(&mut app)?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
