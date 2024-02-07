use async_trait::async_trait;
use crate::cli::command::cli_command::CliCommand;
use crate::info;

pub struct CliArgsCommand {
}

#[async_trait]
impl CliCommand for CliArgsCommand {
    async fn execute(&self, args: Vec<String>) {
        info!(format!("List of args: {args:?}"));
    }
}