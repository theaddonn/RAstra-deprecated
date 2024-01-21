use crate::config::config::Config;
use crate::player::Player;
use crate::{log_error, log_info};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use rand::random;
use rust_raknet::{RaknetListener, RaknetSocket};
use std::env;
use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::{select, time};

// Load server instance as a sort of singleton
// Make it a static ref to be accessible on the entire file
// Load it just on use through lazy-static and init it asynchronous
lazy_static! {
    static ref SERVER_INSTANCE: AsyncOnce<Arc<Mutex<Server>>> = AsyncOnce::new( async {
        Arc::new(Mutex::new(Server::init().await))
    });

    //static ref SERVER_UPDATE_MOTD_SIGNAL: AsyncOnce<(Sender<bool>, Receiver<bool>)> = AsyncOnce::new( async {
    //    tokio::sync::mpsc::channel(1);
    //});
}

pub struct Server {
    config: Config,
    // The player list
    players: Vec<Player>,
    // The guid of the server used for the identity of the server, is rnd every time
    guid: u64,
    // The current tick of the server
    tick: usize,
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
        }

        let server_instance = Server {
            config: server_config,
            players: vec![],
            guid: random(),
            tick: 0,
        };

        return server_instance;
    }

    pub async fn start() {
        log_info!("START SERVER");

        let (running_sender, mut running_receiver) = tokio::sync::mpsc::channel(1);
        let (update_motd_sender, mut update_motd_receiver) = tokio::sync::mpsc::channel(1);

        let _ = vec![
            tokio::spawn(Server::handle_listener(update_motd_receiver)),
            tokio::spawn(Server::start_ctrl_c_task(running_sender.clone())),
            tokio::spawn(Server::start_cli_task(running_sender.clone())),
            tokio::spawn(Server::start_tick_loop(update_motd_sender)),
        ];

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

    pub async fn update_motd(listener: &mut RaknetListener, server: &Server) {
        log_info!("CURRENT MOTD: {}", listener.get_motd().await);

        listener
            .set_full_motd(format!(
                "{};{};{};{};{};{};{};{};Survival;1;{};{};",
                server.config.protocol_edition,
                server.config.name,
                server.config.protocol_version,
                server.config.game_version,
                random::<u8>(),
                server.config.max_players,
                server.guid,
                server.config.motd,
                server.config.port_v4,
                server.config.port_v6,
            ))
            .unwrap();

        log_info!(
            "UPDATED MOTD: {}",
            format!(
                "{};{};{};{};{};{};{};{};Survival;1;{};{};",
                server.config.protocol_edition,
                server.config.name,
                server.config.protocol_version,
                server.config.game_version,
                random::<u8>(),
                server.config.max_players,
                server.guid,
                server.config.motd,
                server.config.port_v4,
                server.config.port_v6,
            )
        );
    }

    pub async fn start_tick_loop(update_motd_sender: Sender<bool>) {
        let mut interval = time::interval(Duration::from_millis(5000));

        log_info!("SPAWNED TICK LOOP");

        loop {
            interval.tick().await;

            let mut server = Server::get_instance().await;
            let mut server = server.lock().await;

            server.tick += 1;
            tokio::spawn(Server::tick(server.tick.clone()));
            update_motd_sender.send(true).await.unwrap();
        }
    }

    pub async fn tick(tick: usize) {
        log_info!("TICK: {tick}")
    }

    pub async fn build_raknet_listener() -> RaknetListener {
        let server = &Server::get_instance().await;
        let server = &server.lock().await;

        let server_config = &server.config;

        return match RaknetListener::bind(&SocketAddr::new(
            IpAddr::from(server_config.ip),
            server_config.port_v4,
        ))
        .await
        {
            Ok(v) => v,
            Err(err) => {
                log_error!("ERROR WHILE TRYING TO START SERVER: \n{err}")
            }
        };
    }

    async fn handle_listener(mut update_motd_receiver: Receiver<bool>) {
        let mut listener = Server::build_raknet_listener().await;

        let server = Server::get_instance().await;
        let server = server.lock().await;

        drop(server);

        listener.listen().await;

        loop {
            select! {
                socket = RaknetListener::accept(&mut listener) => {
                    tokio::spawn(Server::handle_connection(socket.unwrap()));
                }
                _ = update_motd_receiver.recv() => {
                    let server = Server::get_instance().await;
                    let server = server.lock().await;

                    Server::update_motd(&mut listener, &server).await;
                }
            }
        }
    }

    async fn handle_connection(raknet_socket: RaknetSocket) {
        loop {
            let data = raknet_socket.recv().await;
            match data {
                Err(error) => log_error!(
                    "A Error occurred while handling new connections: {:?}",
                    error
                ),
                Ok(buf) => {
                    log_info!(
                        "NEW PACKET!\nDAT: {:?}\nSTR: {:?}",
                        buf,
                        String::from_utf8_lossy(&*buf)
                    );
                }
            }
        }
    }

    async fn start_ctrl_c_task(sender: Sender<bool>) {
        tokio::signal::ctrl_c().await.unwrap();
        sender.send(false).await.unwrap();
    }

    async fn start_cli_task(sender: Sender<bool>) {
        'input_cli: loop {
            let mut buffer = String::new();
            let data = std::io::stdin();
            data.read_line(&mut buffer).unwrap();

            match buffer
                .strip_suffix(String::from_utf8_lossy(&[13, 10]).as_ref())
                .unwrap()
                .to_uppercase()
                .as_str()
            {
                "EXIT" | "STOP" | "RETURN" | "END" | "KILL" => {
                    break 'input_cli;
                }
                val => {
                    log_info!("COMMAND NOT FOUND: {val}")
                }
            }
        }

        sender.send(false).await.unwrap();
    }
}
