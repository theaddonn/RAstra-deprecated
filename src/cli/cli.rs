use input_macro::input;
use tokio::sync::mpsc::Sender;

use crate::{log_info, log_warning};
use crate::cli::command::registry::register::register_commands;
use crate::cli::command::registry::registry::CliCommandRegistry;
use crate::server::Server;

pub struct Cli {
    pub commands_registry: CliCommandRegistry,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            commands_registry: CliCommandRegistry { commands: vec![] },
        }
    }

    pub async fn start_cli(&mut self, running_sender: Sender<bool>) {
        register_commands(self).await;

        for command in &self.commands_registry.commands {
            log_info!(command.name.to_uppercase());
        }

        let _ = tokio::spawn(Self::cli_task(running_sender.clone()));

        let _ = tokio::spawn(Self::ctrl_c_task(running_sender.clone()));
    }

    async fn cli_task(sender: Sender<bool>) {
        'input_cli: loop {
            // request input and get rid of extra chars like whitespaces and newline
            let input = input!();
            let input = input.trim();

            // split into command name, and it's args, if there is just one word return empty rest
            let (name, rest) = match input.split_once(" ") {
                Some(v) => v,
                None => (input, ""),
            };

            // If there was no input, skip processing it and get next input
            if name.trim() == "" {
                continue 'input_cli;
            }

            // split the args into a vec
            let args = rest.split_whitespace().collect::<Vec<_>>();
            let args: Vec<String> = args.iter().map(|&s| s.to_string()).collect();

            if name.to_uppercase() == String::from("EXIT") {
                break 'input_cli;
            }

            let server = Server::get_instance().await;
            let server = server.lock().await;

            for command in &server.cli.commands_registry.commands {
                if command.name.to_uppercase() == name.to_uppercase() {
                    command.command.execute(args.clone()).await;
                    continue 'input_cli;
                }
            }

            log_warning!(format!("COMMAND {name:?} NOT FOUND!"))
        }

        sender.send(false).await.unwrap();
    }

    async fn ctrl_c_task(sender: Sender<bool>) {
        tokio::signal::ctrl_c().await.unwrap();
        sender.send(false).await.unwrap();
    }
}
