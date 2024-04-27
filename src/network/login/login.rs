use bedrock_rs::protocol::packet_io::packet_reader::PacketReader;

use crate::error::RastraError;
use crate::network::conn_info::ConnInfo;
use crate::network::connection::connection::ConnectionReadHalf;
use crate::network::utils::handler::decode;

pub async fn handle_login(
    mut connection_read_half: &mut ConnectionReadHalf,
    conn_info: &ConnInfo,
) -> Result<(), RastraError> {
    let data = match connection_read_half.recv().await {
        Ok(v) => v,
        Err(_) => return Err(RastraError::RecvError),
    };

    let buf = match decode(data, &conn_info).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let mut reader = PacketReader::new_game_packet_reader(buf[0].clone());

    let string = reader.read_string_lossy().unwrap();

    Ok(())
}
