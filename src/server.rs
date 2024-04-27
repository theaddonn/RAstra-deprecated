use cell::Cell;
use std::clone::Clone;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::{cell, env};

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use rak_rs::error::server::ServerError;
use rak_rs::{rakrs_debug, Listener};
use rand::random;
use tokio::sync::{Mutex, MutexGuard};

use crate::cli::cli::Cli;
use crate::config::config::Config;
use crate::error::RastraError;
use crate::network::listener;
use crate::player::Player;
use crate::{log_error, log_info, tick};

lazy_static! {
    static ref SERVER_INSTANCE: AsyncOnce<Arc<Mutex<Server>>> =
        AsyncOnce::new(async { Arc::new(Mutex::new(Server::init().await)) });
}

/// The main Server struct of RAstra
///
/// ### Example:
/// ```no_run
/// #[tokio::main]
/// async fn main() {
///     Server::start().await;
///     Server::stop().await;
/// }
/// ```
pub struct Server {
    pub config: Config,
    // The player list
    pub players: HashMap<u64, Player>,
    // The guid of the server used for the identity of the server, is rnd every time
    pub guid: u64,
    // The current tick of the server
    pub tick: u128,

    pub cli: Cli,
}

impl Server {
    // Get the Server instance with also write access (read access included)
    pub async fn get_instance() -> Arc<Mutex<Server>> {
        return Arc::clone(SERVER_INSTANCE.get().await);
    }

    async fn init() -> Server {
        // Load Server Config from config file loader
        let server_config = Config::load().await;

        // If the user activated debug messages, they should have a full backtrace
        if server_config.debug_messages == true {
            // This sets the env var so Rust gives a full backtrace when it crashes
            env::set_var("RUST_BACKTRACE", "full");
            rakrs_debug!("DD");
        }

        let server_instance = Server {
            config: server_config,
            players: HashMap::new(),
            guid: random(),
            tick: 0,
            cli: Cli::new(),
        };

        return server_instance;
    }

    pub async fn start() {
        log_info!("START SERVER");

        let server: Arc<Mutex<Server>> = Server::get_instance().await;
        let mut server: MutexGuard<Server> = server.lock().await;

        let (running_sender, mut running_receiver) = tokio::sync::mpsc::channel(1);
        let (update_motd_sender, update_motd_receiver) = tokio::sync::mpsc::channel::<bool>(1);

        server.cli.start_cli(running_sender).await;

        tokio::spawn(listener::handle_listener(update_motd_receiver));
        tokio::spawn(tick::tick_loop(update_motd_sender));

        drop(server);

        log_info!("SPAWNED EVERYTHING!");

        'running: loop {
            match running_receiver.recv().await.unwrap() {
                _ => break 'running,
            }
        }
    }

    pub async fn stop() {
        log_info!("STOP SERVER");
    }

    pub async fn build_raknet_listener() -> Listener {
        let server = &Server::get_instance().await;
        let server = &server.lock().await;

        let server_config = &server.config;

        match Listener::bind(SocketAddr::new(
            IpAddr::from(server_config.ip),
            server_config.port_v4,
        ))
        .await
        {
            Ok(v) => return v,
            Err(e) => match e {
                ServerError::AddrBindErr => {
                    log_error!(format!(
                        "Could not bind to Address: {}:{}",
                        server_config.ip, server_config.port_v4
                    ));
                }
                ServerError::AlreadyOnline => {
                    log_error!("TRIED TO START SERVER: AlreadyOnline");
                }
                ServerError::NotListening => {
                    log_error!("TRIED TO START SERVER: NotListening");
                }
                ServerError::Killed => {
                    log_error!("TRIED TO START SERVER: Killed");
                }
                ServerError::Reset => {
                    log_error!("TRIED TO START SERVER: Reset");
                }
            },
        }
    }

    pub fn add_player(&mut self, player: Player) -> u64 {
        let mut players = &mut self.players;

        let id = random();

        players.insert(id, player);

        log_info!("ADDED PLAYER!", id);

        return id;
    }

    pub fn remove_player(&mut self, id: &u64) -> Result<(), RastraError> {
        Ok(())
    }

    pub fn find_player(&self, id: &u64) -> Option<&Player> {
        let mut players = &self.players;

        players.get(&id)
    }
}
