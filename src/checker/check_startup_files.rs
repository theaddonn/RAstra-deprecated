use crate::log_error;
use std::env::current_exe;
use std::process::exit;
use tokio::fs;

async fn folder_exits(path: &str) -> bool {
    return match fs::read_dir(path).await {
        Ok(_) => true,
        Err(_) => false,
    };
}

async fn file_exits(path: &str) -> bool {
    return match fs::read(path).await {
        Ok(_) => true,
        Err(_) => false,
    };
}

async fn check_or_create_dir(path: &str) {
    return match folder_exits(path).await {
        true => {}
        false => match fs::create_dir(path).await {
            Ok(_) => {}
            Err(err) => {
                log_error!(
                    "An Error occurred while trying to write a missing file: {:?}",
                    err
                );
                exit(1);
            }
        },
    };
}

async fn check_or_create_file(path: &str, data: &[u8]) {
    return match folder_exits(path).await {
        true => {}
        false => match fs::write(path, data).await {
            Ok(_) => {}
            Err(err) => {
                log_error!(
                    "An Error occurred while trying to write a missing file: {:?}",
                    err
                );
                exit(1);
            }
        },
    };
}

pub async fn check_all() {
    let mut current_exe_dir = current_exe().unwrap();
    current_exe_dir.pop();

    check_or_create_file("config.toml", include_bytes!("default_config.toml")).await;

    check_or_create_dir("\\plugins").await;
    check_or_create_dir("\\worlds").await;
    check_or_create_dir("\\resource_packs").await;
}
