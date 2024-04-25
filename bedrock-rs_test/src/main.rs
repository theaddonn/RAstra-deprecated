use std::io::{Bytes, Read};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::main;
use tokio::net::UdpSocket;
use raknet_rs::server::{ConfigBuilder, MakeIncoming};

use futures_util::StreamExt;
use futures_util::SinkExt;

#[main]
async fn main() {
    let socket = UdpSocket::bind(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        19132
    )).await.unwrap();


    let config = ConfigBuilder::default()
        .min_mtu(128)
        .max_mtu(1024)
        .send_buf_cap(1024*2*2*2*2)
        .sever_guid(123456789)
        .advertisement(bytes::Bytes::copy_from_slice(b"HELLO"))
        .support_version(vec![9, 10, 11])
        .max_pending(128)

        .build()
        .unwrap();

    let mut incoming = socket.make_incoming(config);

    let mut io = incoming.next().await.unwrap();
    let data: bytes::Bytes = io.next().await.unwrap();
    io.send(data).await.unwrap();
}
