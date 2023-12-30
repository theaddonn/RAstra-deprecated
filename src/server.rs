use crate::network::packet::packet_info::GamePacket;
use crate::raknet::{RaknetListener, RaknetSocket, Reliability};
use crate::{log_info, log_warning};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use crate::network::packet_io::PacketWriter;
use crate::utils::endian::Endian;

pub struct Server {
    listener: RaknetListener,
}

impl Server {
    pub async fn new() -> Arc<Mutex<Server>> {
        log_info!("START SERVER");
        return Arc::new(Mutex::new(Server {
            listener: RaknetListener::bind(&SocketAddr::new(
                IpAddr::from_str("127.0.0.1").expect("Couldn't get Ip Address from String"),
                19132,
            ))
            .await
            .unwrap(),
        }));
    }

    pub async fn start(public_server: &Arc<Mutex<Self>>) {
        let mut server = public_server.lock().await;

        server
            .listener
            .set_pong_data(Vec::from(
                "MCPE;Dedicated Server;390;1.14.60;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;"
                    .as_bytes(),
            ))
            .unwrap();

        server.listener.listen().await;

        drop(server);

        let (mut running_sender, mut running_receiver) = tokio::sync::mpsc::channel(1);

        let task_accept_new_connections =
            tokio::spawn(Server::accept_new_connections(Arc::clone(public_server)));
        let task_start_ctrl_c_thread = tokio::spawn(Server::start_ctrl_c_thread(
            Arc::clone(public_server),
            running_sender.clone(),
        ));
        let task_start_cli_thread = tokio::spawn(Server::start_cli_thread(
            Arc::clone(public_server),
            running_sender.clone(),
        ));

        'running: loop {
            match running_receiver.recv().await.unwrap() {
                _ => break 'running,
            }
        }

        task_accept_new_connections.abort();
        task_start_ctrl_c_thread.abort();
        task_start_cli_thread.abort();
    }

    pub async fn stop(public_server: &Arc<Mutex<Self>>) {
        log_info!("STOP SERVER");
    }

    async fn accept_new_connections(public_server: Arc<Mutex<Self>>) {
        let mut server = public_server.lock().await;
        let listener = &mut server.listener;

        loop {
            let socket = listener.accept().await.unwrap();
            tokio::spawn(Server::handle_connection(
                Arc::clone(&public_server),
                socket,
            ));
        }
    }

    async fn handle_connection(server: Arc<Mutex<Self>>, raknet_socket: RaknetSocket) {
        loop {
            let data = raknet_socket.recv().await;
            match data {
                Err(error) => break,
                Ok(buf) => {
                    log_info!(
                        "NEW PACKET!\nDAT: {:?}\nSTR: {:?}",
                        buf,
                        String::from_utf8_lossy(&*buf)
                    );

                    let mut packet = PacketWriter::new();
                    packet.write_u8(254).unwrap();
                    packet.write_u32_varint(12).unwrap();
                    packet.write_u8(143).unwrap();
                    packet.write_u16(0, Endian::Big).unwrap();
                    packet.write_u16(0, Endian::Big).unwrap();
                    packet.write_u8(0).unwrap();
                    packet.write_u8(0).unwrap();
                    packet.write_f32(0.0).unwrap();
                    packet.write_u8(0).unwrap();

                    log_info!("OWN: {:?}", &packet.clone().get_raw_payload());

                    let _ = raknet_socket
                        .send(
                            &packet.clone().get_raw_payload(),
                            Reliability::Reliable,
                        )
                        .await;

                    tokio::fs::write(
                        &Path::new(&format!(
                            "D:\\user\\adria\\Downloads\\buf_data{:?}.bin",
                            buf[2]
                        )),
                        &buf,
                    )
                    .await
                    .unwrap()
                }
            }
        }
    }

    async fn start_ctrl_c_thread(server: Arc<Mutex<Self>>, sender: Sender<bool>) {
        tokio::signal::ctrl_c().await.unwrap();
        sender.send(false).await.unwrap();
    }

    async fn start_cli_thread(server: Arc<Mutex<Self>>, sender: Sender<bool>) {
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
                _ => {
                    // TODO: ADD CLI INPUT COMMANDS AND NOT FOUND
                }
            }
        }

        sender.send(false).await.expect("TODO: panic message");
    }
}
