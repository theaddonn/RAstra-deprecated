pub mod core {
    pub use bedrock_core::*;

    pub mod ser {
        pub use serialize::error::SerilizationError;
        pub use serialize::ser::*;
        pub use serialize_derive::MCSerialize;
    }

    pub mod de {
        pub use serialize::de::*;
        pub use serialize::error::DeserilizationError;
        pub use serialize_derive::MCDeserialize;
    }
}

pub mod protocol {
    pub use protocol::*;
}
