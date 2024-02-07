use crate::config::config::Config;
use crate::player::Player;
use crate::{error, info, warning};
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use rand::random;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::process::exit;
use std::sync::{Arc};
use std::time::Duration;
use rak_rs::Listener;
use rak_rs::connection::Connection;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, MutexGuard};
use tokio::{select, time};
use crate::cli::cli::Cli;
use crate::motd::update::update_motd;
use std::clone::Clone;

// Load server instance as a sort of singleton
// Make it a static ref to be accessible on the entire file
// Load it just on use through lazy-static and init it asynchronous
lazy_static! {
    static ref SERVER_INSTANCE: AsyncOnce<Arc<Mutex<Server>>> = AsyncOnce::new( async {
        Arc::new(Mutex::new(Server::init().await))
    });
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
    pub players: Vec<Player>,
    // The guid of the server used for the identity of the server, is rnd every time
    pub guid: u64,
    // The current tick of the server
    tick: usize,

    // The cli
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
        }

        let server_instance = Server {
            config: server_config,
            players: vec![],
            guid: random(),
            tick: 0,
            cli: Cli::new(),
        };

        return server_instance;
    }

    pub async fn start() {
        info!("HELLO", "START SERVER");

        rak_rs::rakrs_debug!("DD");

        let server: Arc<Mutex<Server>> = Server::get_instance().await;
        let mut server: MutexGuard<Server> = server.lock().await;

        let (running_sender, mut running_receiver) = tokio::sync::mpsc::channel(1);
        let (update_motd_sender, update_motd_receiver) = tokio::sync::mpsc::channel::<bool>(1);

        server.cli.start_cli(running_sender).await;

        tokio::spawn(Server::handle_listener(update_motd_receiver));
        tokio::spawn(Server::tick_loop(update_motd_sender));

        drop(server);

        info!("SPAWNED EVERYTHING!");

        'running: loop {
            match running_receiver.recv().await.unwrap() {
                _ => break 'running,
            }
        }
    }

    pub async fn stop() {
        info!("STOP SERVER");
    }

    pub async fn tick_loop(update_motd_sender: Sender<bool>) {
        // 20 ticks per sec, 1 tick every 50 mil-secs
        let mut interval = time::interval(Duration::from_millis(50));

        info!("SPAWNED TICK LOOP");

        loop {
            interval.tick().await;

            let server = Server::get_instance().await;
            let mut server = server.lock().await;

            server.tick += 1;
            tokio::spawn(Server::tick(server.tick.clone()));

            let update_motd_sender = update_motd_sender.clone();

            tokio::spawn(async move {
                let _ = update_motd_sender.send(true);
            });
        }
    }

    pub async fn tick(tick: usize) {
    }

    pub async fn build_raknet_listener() -> Listener {
        let server = &Server::get_instance().await;
        let server = &server.lock().await;

        let server_config = &server.config;

        return Listener::bind(SocketAddr::new(
            IpAddr::from(server_config.ip),
            server_config.port_v4,
        )).await.unwrap_or_else(|_| {
            error!("ERROR WHILE TRYING TO START SERVER")
        });
    }

    async fn handle_listener(mut update_motd_receiver: Receiver<bool>) {
        let mut listener = Server::build_raknet_listener().await;

        update_motd(&mut listener).await;

        listener.start().await.unwrap();

        info!("LISTENED!");
        //exlog_info!("INFO:\nMOTD: {:#?}\nID: {:#?}\nVERSIONS: {:#?}", listener.motd, listener.id, listener.versions);

        loop {
            let conn = listener.accept().await.unwrap();

            //log_info!("INFO:\nMOTD: {:#?}\nID: {:#?}\nVERSIONS: {:#?}", listener.motd, listener.id, listener.versions);

            tokio::spawn(Server::handle_connection(conn));
            info!("SPAWNED CONNECTION!!");

            //log_info!("SELECTING!");
            //select! {
            //    socket = listener.accept() => {
            //        log_info!("SPAWNING CONNECTION!");
            //        tokio::spawn(Server::handle_connection(socket.unwrap()));
            //        log_info!("SPAWNED CONNECTION!!");
            //    }
            //    _ = update_motd_receiver.recv() => {
//
            //        log_info!("UPDATE MOTD!!");
            //        //tokio::spawn(update_motd(&mut listener));
            //        log_info!("UPDATED MOTD!!!");
            //    }
            //}
            //log_info!("SELECTED!");
        }
    }

    async fn handle_connection(mut connection: Connection) {
        info!("HANDLE CONNECTION!");
        'handle: loop {
            let data = connection.recv().await;
            match data {
                Err(error) => {
                    warning!("A Error occurred while handling new connections");
                    connection.close().await;
                    break 'handle;
                }
                Ok(buf) => {
                    info!(format!(
                        "NEW PACKET!\nDAT: {:?}\nSTR: {:?}",
                        buf,
                        String::from_utf8_lossy(&*buf)
                    ));
                }
            }
        }
    }




}
