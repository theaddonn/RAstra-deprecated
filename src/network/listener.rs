use tokio::sync::mpsc::Receiver;

use crate::log_info;
use crate::motd::update::update_motd;
use crate::network::conn_handle;
use crate::network::connection::connection::ConnectionWrapper;
use crate::server::Server;

pub async fn handle_listener(mut update_motd_receiver: Receiver<bool>) {
    let mut listener = Server::build_raknet_listener().await;

    update_motd(&mut listener).await;

    listener.start().await.unwrap();

    log_info!("LISTENED!");

    loop {
        let connection = listener.accept().await.unwrap();
        log_info!("SPAWNING CONNECTION!");

        let conn_wrapper = ConnectionWrapper::new(connection).await;

        tokio::spawn(conn_handle::handle_connection(conn_wrapper));
        log_info!("SPAWNED CONNECTION!");

        //log_info!("SELECTING!");
        //select! {
        //    socket = listener.accept() => {
        //        log_info!("SPAWNING CONNECTION!");
        //        tokio::spawn(Server::handle_connection(socket.unwrap()));
        //        log_info!("SPAWNED CONNECTION!!");
        //    }
        //    _ = update_motd_receiver.recv() => {
        //        log_info!("UPDATE MOTD!!");
        //        //tokio::spawn(update_motd(&mut listener));
        //        log_info!("UPDATED MOTD!!!");
        //    }
        //}
        //log_info!("SELECTED!");
    }
}
