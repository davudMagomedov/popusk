use crate::parse_cli::CliCommand;
use crate::pcommand::*;
use crate::wacommand::*;

#[allow(non_camel_case_types)]
pub enum P_WA_Command {
    PCommand(Box<dyn PCommand>),
    WACommand(Box<dyn WACommand>),
}

pub fn parse_cli_command(cli_command: CliCommand) -> P_WA_Command {
    match cli_command {
        CliCommand::Init => P_WA_Command::WACommand(Box::new(InitWACMD::new())),
        CliCommand::AddPath { path } => P_WA_Command::PCommand(Box::new(AddPathPCMD::new(path))),
        CliCommand::AddProgress { id, progress } => {
            P_WA_Command::PCommand(Box::new(AddProgressPCMD::new(id, progress)))
        }
        CliCommand::AddEntitybase { id } => {
            P_WA_Command::PCommand(Box::new(AddEntitybasePCMD::new(id)))
        }
        CliCommand::DelPath { path } => P_WA_Command::PCommand(Box::new(DelPathPCMD::new(path))),
        CliCommand::DelProgress { id } => {
            P_WA_Command::PCommand(Box::new(DelProgressPCMD::new(id)))
        }
        CliCommand::DelEntitybase { id } => {
            P_WA_Command::PCommand(Box::new(DelEntitybasePCMD::new(id)))
        }
        CliCommand::GetId { path } => P_WA_Command::PCommand(Box::new(GetIDPCMD::new(path))),
        CliCommand::GetProgress { id } => {
            P_WA_Command::PCommand(Box::new(GetProgressPCMD::new(id)))
        }
        CliCommand::GetEntitybase { id } => {
            P_WA_Command::PCommand(Box::new(GetEntitybasePCMD::new(id)))
        }
        CliCommand::AddLibentity {
            path,
            name,
            tags,
            prog_ceil,
        } => P_WA_Command::PCommand(Box::new(AddLibentityPCMD::new(path, name, tags, prog_ceil))),
        CliCommand::DelLibentity { path } => {
            P_WA_Command::PCommand(Box::new(DelLibentityPCMD::new(path)))
        }
        CliCommand::Look { path } => P_WA_Command::PCommand(Box::new(LookPCMD::new(path))),
        CliCommand::List { wide } => {
            P_WA_Command::PCommand(Box::new(ListPCMD::new(ListMode::wide(wide))))
        }
        CliCommand::Status {
            show_hidden,
            ignore,
        } => P_WA_Command::PCommand(Box::new(StatusPCMD::new(show_hidden, ignore))),
        CliCommand::Open { path, just_look } => {
            P_WA_Command::PCommand(Box::new(OpenPCMD::new(path, just_look)))
        }
        CliCommand::ChangeProgress {
            id,
            progress_update,
        } => P_WA_Command::PCommand(Box::new(ChangeProgressPCMD::new(id, progress_update))),
        CliCommand::AddTags { id, tags } => {
            P_WA_Command::PCommand(Box::new(AddTagsPCMD::new(id, tags)))
        }
        CliCommand::DelTags { id } => P_WA_Command::PCommand(Box::new(DelTagsPCMD::new(id))),
    }
}
