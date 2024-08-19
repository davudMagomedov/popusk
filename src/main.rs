#![allow(dead_code)]
#![cfg(unix)]

use anyhow::Result;
use clap::Parser;

mod app;
mod comps_appearance;
mod comps_interaction;
mod config;
mod core_commands;
mod entity_base;
mod global_conf_directory;
mod id;
mod parse_cli;
mod parse_cli_command;
mod pcommand;
mod progress;
mod progress_update;
mod scripts;
mod storage;
mod wacommand; // w(ithout)a(pplication)command

use app::App;
use parse_cli::CLI;
use parse_cli_command::{parse_cli_command, P_WA_Command};

fn main() -> Result<()> {
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
