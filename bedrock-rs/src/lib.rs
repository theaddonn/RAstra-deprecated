pub mod core {
    pub use bedrock_core::*;
}

pub mod protocol {
    pub use protocol::*;

    pub mod ser {
        pub use serialize_derive::MCProtoSerialize;

        pub use serialize::proto::error::SerilizationError;
        pub use serialize::proto::ser::*;
    }

    pub mod de {
        pub use serialize_derive::MCProtoDeserialize;

        pub use serialize::proto::de::*;
        pub use serialize::proto::error::DeserilizationError;
    }
}
