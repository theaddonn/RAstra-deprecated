use crate::log_info;
use crate::network::connection::connection::{ConnectionReadHalf, ConnectionWrapper};
use crate::network::login::handle::handle_login_process;

pub async fn handle_connection(mut conn_wrapper: ConnectionWrapper) {
    log_info!("HANDLE CONNECTION!");

    let (mut conn_read_half, mut conn_write_half) = conn_wrapper.split();

    match handle_login_process(&mut conn_read_half, conn_write_half).await {
        Ok(_) => {}
        Err(_) => {
            return;
        }
    };

    return;

    // TODO GET EVERYTHING WORKING AGAIN!!! tokio::spawn(handle_recv_loop(conn_read_half, peer_id));
}

async fn handle_recv_loop(mut conn_read_half: ConnectionReadHalf, peer_id: u64) {
    'handle: loop {
        let data = conn_read_half.recv().await;

        match data {
            Err(_) => {
                drop(conn_read_half);

                break 'handle;
            }
            Ok(buf) => {
                log_info!("NEW PACKET!");

                tokio::spawn(handle_recv_data(buf, peer_id));
            }
        }
    }
}

async fn handle_recv_data(mut data: Vec<u8>, peer_id: u64) {
    log_info!("HANDLE RECV DATA!");

    //let packets = match decode(data.clone(), peer_id, false, false).await {
    //    Ok(v) => { v }
    //    Err(_) => { return; }
    //};
    //
    //
    //for packet in packets {
    //    tokio::spawn(handle_packet(packet, peer_id));
    //}
}
