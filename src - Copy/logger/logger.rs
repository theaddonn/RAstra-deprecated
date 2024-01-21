#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        println!("{}", format!("[ \x1b[38;5;47mINFO\x1b[0m ] {}", format!($($arg)*)))
    }}
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {{
        println!("{}", format!("[ \x1b[38;5;227mWARNING\x1b[0m ] {}", format!($($arg)*)))
    }}
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        println!("{}", format!("[ \x1b[38;5;196mERROR\x1b[0m ] {}", format!($($arg)*)))
    }}
}
