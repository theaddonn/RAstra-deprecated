use std::io::Cursor;
use bedrock_rs::core::types::{i64be, i64le};
use bedrock_rs::protocol::error::ProtocolError;
use bedrock_rs::protocol::GamepacketSerialize;
use bedrock_rs::protocol::gamepacket::{Deserialize, GamepacketSerialize, Serialize};

#[derive(Debug, Default)]
struct NetworkSettings {
    id: i64,
    compression_method: i16
}

#[derive(Debug, Default, GamepacketSerialize)]
struct Help {
    i: i64,
    settings: NetworkSettings,
}

impl Serialize for Help {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> where Self: Sized {
        self.i.serialize()
    }
}

fn main() {
    let help = Help::default();
    
    println!("{:?}", help.to_packet());

    use bedrock_rs::protocol::gamepacket::GamepacketSerialize;
    
    println!("Hello, world!");

    let data: i64be = 42.into();

    let bin = data.serialize().unwrap();

    let data2: i64be =  i64be::deserialize(&mut Cursor::new(bin)).unwrap();

    println!("{:?} == {:?}", data2, i64be { data: 42 });
}
