// i8
// use normal i8

// u8
// use normal u8

// i16
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i16le {
    pub data: i16,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i16be {
    pub data: i16,
}

// u16
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u16le {
    pub data: u16,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u16be {
    pub data: u16,
}

// i32
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i32le {
    pub data: i32,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i32be {
    pub data: i32,
}

// u32
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u32le {
    pub data: u32,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u32be {
    pub data: u32,
}

// i64
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i64le {
    pub data: i64,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i64be {
    pub data: i64,
}

// u64
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u64le {
    pub data: u64,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u64be {
    pub data: u64,
}

// i128
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i128le {
    pub data: i128,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct i128be {
    pub data: i128,
}

// u128
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u128le {
    pub data: u128,
}

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct u128be {
    pub data: u128,
}

// f32
#[derive(Copy, Clone, Default)]
pub struct f32le {
    pub data: f32,
}

#[derive(Copy, Clone, Default)]
pub struct f32be {
    pub data: f32,
}

// f64
#[derive(Copy, Clone, Default)]
pub struct f64le {
    pub data: f64,
}

#[derive(Copy, Clone, Default)]
pub struct f64be {
    pub data: f64,
}

// uvarint 32
#[derive(Copy, Clone, Default)]
pub struct uvar32le {
    pub data: u32,
}

#[derive(Copy, Clone, Default)]
pub struct uvar32be {
    pub data: u32,
}

// ivarint 32
#[derive(Copy, Clone, Default)]
pub struct ivar32le {
    pub data: i32,
}

#[derive(Copy, Clone, Default)]
pub struct ivar32be {
    pub data: i32,
}

// uvarint 64
#[derive(Copy, Clone, Default)]
pub struct uvar64le {
    pub data: u64,
}

#[derive(Copy, Clone, Default)]
pub struct uvar64be {
    pub data: u64,
}

// ivarint 32
#[derive(Copy, Clone, Default)]
pub struct ivar64le {
    pub data: i64,
}

#[derive(Copy, Clone, Default)]
pub struct ivar64be {
    pub data: i64,
}

// bool
// use normal bool