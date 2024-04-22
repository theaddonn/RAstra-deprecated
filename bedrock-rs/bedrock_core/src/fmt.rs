use std::fmt;
use std::fmt::Formatter;
use crate::types::*;

// i16
impl fmt::Display for i16le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for i16be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// u16
impl fmt::Display for u16le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for u16be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// i32
impl fmt::Display for i32le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for i32be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// u32
impl fmt::Display for u32le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for u32be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// i64
impl fmt::Display for i64le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for i64be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// u64
impl fmt::Display for u64le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for u64be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// i128
impl fmt::Display for i128le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for i128be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// u128
impl fmt::Display for u128le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for u128be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// f32
impl fmt::Display for f32le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for f32be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

// f64
impl fmt::Display for f64le {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}

impl fmt::Display for f64be {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data.to_string().as_str())
    }
}