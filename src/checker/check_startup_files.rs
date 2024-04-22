use std::env::current_exe;

use tokio::fs;

use crate::log_error;

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
            }
        },
    };
}

async fn check_or_create_file(path: &str, data: &[u8]) {
    return match file_exits(path).await {
        true => {}
        false => match fs::write(path, data).await {
            Ok(_) => {}
            Err(err) => {
                log_error!(
                    "An Error occurred while trying to write a missing file: {:?}",
                    err
                );
            }
        },
    };
}

pub async fn check_all() {
    let mut current_exe_dir = current_exe().unwrap();
    current_exe_dir.pop();

    check_or_create_file("rastra.toml", include_bytes!("default_rastra.toml")).await;

    check_or_create_dir("plugins").await;
    check_or_create_dir("worlds").await;
    check_or_create_dir("resource_packs").await;
}
