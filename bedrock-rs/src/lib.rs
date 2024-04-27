pub mod core {
    pub use bedrock_core::*;

    pub mod ser {
        pub use serialize_derive::MCSerialize;

        pub use serialize::error::SerilizationError;
        pub use serialize::ser::*;
    }

    pub mod de {
        pub use serialize_derive::MCDeserialize;

        pub use serialize::de::*;
        pub use serialize::error::DeserilizationError;
    }
}

pub mod protocol {
    pub use protocol::*;
}
