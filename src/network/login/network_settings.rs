use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::{ConnectionReadHalf, ConnectionWriteHalf};
use crate::network::gamepacket::{GamepacketClient, GamepacketServer};
use crate::network::packet::packet_network_settings::PacketNetworkSettings;
use crate::network::packet::packet_request_network_settings::PacketRequestNetworkSettings;
use crate::network::utils::handler::{decode, encode};

pub async fn handle_network_settings(
    mut connection_read_half: &mut ConnectionReadHalf,
    connection_write_half: &ConnectionWriteHalf, conn_info: &mut ConnInfo
) -> Result<u32, RastraError> {

    let data = match connection_read_half.recv().await {
        Ok(v) => v,
        Err(_) => return Err(RastraError::RecvError),
    };

    let buf = match decode(data, &conn_info).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let packet = match PacketRequestNetworkSettings::deserialize(buf[0].to_owned()) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    let answer_packet = PacketNetworkSettings::new_from_server().await;

    let answer_packet_data = match answer_packet.serialize() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let answer_packetbatch = vec![answer_packet_data];

    let answer_packetbatch_data = match encode(answer_packetbatch, conn_info).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    match connection_write_half
        .send(&*answer_packetbatch_data, true)
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(RastraError::SendError),
    };

    Ok(packet.protocol_version)
}
