use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::main;
use tokio::net::UdpSocket;

#[main]
async fn main() {
    let socket = UdpSocket::bind(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        19132
    )).await.unwrap();

    println!("{:?}", socket.local_addr().unwrap());

    let mut buf = vec![0; 1024*2*2*2*2*2*2];
    socket.recv(&mut buf).await.unwrap();
    println!("{:?}", buf);

}
