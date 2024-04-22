use async_trait::async_trait;

use crate::cli::command::cli_command::CliCommand;
use crate::log_info;

pub struct CliHelpCommand {}

#[async_trait]
impl CliCommand for CliHelpCommand {
    async fn execute(&self, args: Vec<String>) {
        log_info!("help is under the water");
    }
}
