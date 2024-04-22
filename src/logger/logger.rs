#[macro_export]
macro_rules! log_info {
    ($arg:expr) => {{
        use chrono::Timelike;

        let now = chrono::Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        println!("{}", format!("[ \x1b[38;5;47mINFO\x1b[0m \x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m ] {}", hour, minute, second, $arg));
    }};
    ($sender:expr, $arg:expr) => {{
        use chrono::Timelike;

        let now = chrono::Local::now();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        println!("{}", format!("[ \x1b[38;5;47mINFO\x1b[0m \x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m ] [ \x1b[38;5;51m{}\x1b[0m ] {}", hour, minute, second, $sender, $arg));
    }};
}

#[macro_export]
macro_rules! log_warning {
    ($arg:expr) => {{
        use chrono::Timelike;

        let now = chrono::Local::now();
        let hour = chrono::DateTime::hour(&now);
        let minute = chrono::DateTime::minute(&now);
        let second = chrono::DateTime::second(&now);

        println!("{}", format!("[ \x1b[38;5;227mWARNING\x1b[0m \x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m ] {}", hour, minute, second, $arg));
    }};
    ($sender:expr, $arg:expr) => {{
        use chrono::Timelike;

        let now = chrono::Local::now();
        let hour = chrono::DateTime::hour(&now);
        let minute = chrono::DateTime::minute(&now);
        let second = chrono::DateTime::second(&now);

        println!("{}", format!("[ \x1b[38;5;227mWARNING\x1b[0m \x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m ] [ {} ] {}", hour, minute, second, $sender, $arg));
    }};
}

#[macro_export]
macro_rules! log_error {
    ($arg:expr) => {{
        use chrono::Timelike;

        let now = chrono::Local::now();
        let hour = chrono::DateTime::hour(&now);
        let minute = chrono::DateTime::minute(&now);
        let second = chrono::DateTime::second(&now);

        println!("{}", format!("[ \x1b[38;5;196mERROR\x1b[0m \x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m ] {}", hour, minute, second, $arg));
        std::process::exit(1);
    }};
    ($sender:expr, $arg:expr) => {{
        use chrono::Timelike;

        let now = chrono::Local::now();
        let hour = chrono::DateTime::hour(&now);
        let minute = chrono::DateTime::minute(&now);
        let second = chrono::DateTime::second(&now);

        println!("{}", format!("[ \x1b[38;5;196mERROR\x1b[0m \x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m\x1b[38;5;242m:\x1b[0m\x1b[38;5;243m{:02}\x1b[0m ] [ \x1b[38;5;126m{}\x1b[0m  ] {}", hour, minute, second, $sender, $arg));
        std::process::exit(1);
    }};
}
