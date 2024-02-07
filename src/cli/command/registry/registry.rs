use crate::cli::command::cli_command::CliCommand;
use crate::error::RastraError;

pub struct RegistryCliCommand {
    pub command: Box<dyn CliCommand + Send + Sync + 'static>,
    // will always be treated as uppercase
    pub name: String,
    pub description: String,
}

pub struct CliCommandRegistry {
    pub commands: Vec<RegistryCliCommand>
}

impl CliCommandRegistry {
    pub async fn add_command(&mut self, command: Box<(dyn CliCommand + Send + Sync + 'static)>, name: String, description: String) -> Result<(), RastraError> {

        // Check to not have duplicate cli commands.md
        for cli_command in &self.commands {
            if cli_command.name == name {
                return Err(RastraError::CliCommandNameAlreadyTaken);
            }
        }

        self.commands.push(
            RegistryCliCommand{
                command,
                name,
                description
            }
        );

        Ok(())
    }
}