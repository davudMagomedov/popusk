#![allow(dead_code)]

use clap::Parser;

mod app;
mod comps_appearance;
mod comps_interaction;
mod config;
mod core_commands;
mod error_ext;
mod global_conf_directory;
mod library;
mod localconf;
mod parse_cli;
mod parse_cli_command;
mod pcommand;
mod scripts;
mod storage;
mod types;
mod wacommand; // w(ithout)a(pplication)command

use app::App;
use parse_cli::CLI;
use parse_cli_command::{parse_cli_command, P_WA_Command};

use std::process::ExitCode;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();
    let command = parse_cli_command(cli.command);

    match command {
        P_WA_Command::WACommand(wacommand) => wacommand.execute()?,
        P_WA_Command::PCommand(pcommand) => {
            let mut app = App::open()?;
            pcommand.execute(&mut app)?;
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        println!("Error: {}", err);

        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}
