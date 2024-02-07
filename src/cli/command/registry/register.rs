use crate::cli::cli::Cli;
use crate::cli::command::instance::args::CliArgsCommand;
use crate::cli::command::instance::help::CliHelpCommand;

pub async fn register_commands(cli: &mut Cli){
    cli.commands_registry.add_command(
        Box::new(CliHelpCommand{}),
        String::from("help"),
        String::from("Helps you.")
    ).await.unwrap();

    cli.commands_registry.add_command(
        Box::new(CliArgsCommand{}),
        String::from("args"),
        String::from("Lists all given args.")
    ).await.unwrap();
}