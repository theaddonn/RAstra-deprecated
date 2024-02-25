use rak_rs::connection::Connection;

use crate::network::batching::debatch_packet_stream;
use crate::network::packet_handle::handle_packet;
use crate::{info, warning};

pub async fn handle_connection(mut connection: Connection) {
    info!("HANDLE CONNECTION!");
    'handle: loop {
        let data = connection.recv().await;
        match data {
            Err(_) => {
                connection.close().await;
                break 'handle;
            }
            Ok(buf) => {
                info!(format!(
                    "NEW PACKET!\nDAT: {:?}\nSTR: {:?}",
                    buf,
                    String::from_utf8_lossy(&*buf)
                ));

                tokio::spawn(handle_recv_data(buf));
            }
        }
    }
}

async fn handle_recv_data(data: Vec<u8>) {
    match data[0] {
        0xfe => {
            let packets = debatch_packet_stream(data.clone());

            for packet in packets {
                match packet {
                    Ok(packet) => {
                        tokio::spawn(handle_packet(packet));
                    }
                    Err(_) => {
                        warning!("PACKET WAS INVALID!")
                    }
                }
            }
        }
        _ => {
            warning!("GOT UNKNOWN RAKNET PACKET!")
        }
    }
}
