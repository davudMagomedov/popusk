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
        CliCommand::SetProgress { path, progress } => {
            P_WA_Command::PCommand(Box::new(SetProgressPCMD::new(path, progress)))
        }
        CliCommand::SetEntitybase { path } => {
            P_WA_Command::PCommand(Box::new(SetEntitybasePCMD::new(path)))
        }
        CliCommand::SetName { path, name } => {
            P_WA_Command::PCommand(Box::new(SetNamePCMD::new(path, name)))
        }
        CliCommand::SetEtype { path, etype } => {
            P_WA_Command::PCommand(Box::new(SetEtypePCMD::new(path, etype)))
        }
        CliCommand::SetFreeData { path, file } => {
            P_WA_Command::PCommand(Box::new(SetFreeDataPCMD::new(path, file)))
        }
        CliCommand::AddTags { path, tags } => {
            P_WA_Command::PCommand(Box::new(AddTagsPCMD::new(path, tags)))
        }
        CliCommand::SetDescription { path, description } => {
            P_WA_Command::PCommand(Box::new(SetDescriptionPCMD::new(path, description)))
        }
        CliCommand::DelProgress { path } => {
            P_WA_Command::PCommand(Box::new(DelProgressPCMD::new(path)))
        }
        CliCommand::DelTags { path } => P_WA_Command::PCommand(Box::new(DelTagsPCMD::new(path))),
        CliCommand::DelDescription { path } => {
            P_WA_Command::PCommand(Box::new(DelDescriptionPCMD::new(path)))
        }
        CliCommand::GetProgress { path } => {
            P_WA_Command::PCommand(Box::new(GetProgressPCMD::new(path)))
        }
        CliCommand::GetDescription { path } => {
            P_WA_Command::PCommand(Box::new(GetDescriptionPCMD::new(path)))
        }
        CliCommand::GetTags { path } => {
            P_WA_Command::PCommand(Box::new(GetTagsPCMD::new(path)))
        }
        CliCommand::GetEntitybase { path } => {
            P_WA_Command::PCommand(Box::new(GetEntitybasePCMD::new(path)))
        }
        CliCommand::AddLibentity { path } => {
            P_WA_Command::PCommand(Box::new(AddLibentityPCMD::new(path)))
        }
        CliCommand::DelLibentity { path } => {
            P_WA_Command::PCommand(Box::new(DelLibentityPCMD::new(path)))
        }
        CliCommand::Look { path } => P_WA_Command::PCommand(Box::new(LookPCMD::new(path))),
        CliCommand::List { wide } => {
            P_WA_Command::PCommand(Box::new(ListPCMD::new(ListMode::wide(wide))))
        }
        CliCommand::Status {
            show_hidden,
            show_directories,
            ignore,
        } => P_WA_Command::PCommand(Box::new(StatusPCMD::new(
            show_hidden,
            show_directories,
            ignore,
        ))),
        CliCommand::Open { path, just_look } => {
            P_WA_Command::PCommand(Box::new(OpenPCMD::new(path, just_look)))
        }
        CliCommand::ChangeProgress {
            path,
            progress_update,
        } => P_WA_Command::PCommand(Box::new(ChangeProgressPCMD::new(path, progress_update))),
        CliCommand::RenameLibentity { path, new_path } => {
            P_WA_Command::PCommand(Box::new(RenameLibEntityPCMD::new(path, new_path)))
        }
    }
}
