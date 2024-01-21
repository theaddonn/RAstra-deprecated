use serde::Deserialize;

#[derive(Deserialize)]
pub enum CompressionMethod {
    Flate = 0,
    Snappy = 1,
}
