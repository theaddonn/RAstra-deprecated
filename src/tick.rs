use std::time::Duration;

use tokio::sync::mpsc::Sender;
use tokio::time;

use crate::log_info;

pub async fn tick_loop(update_motd_sender: Sender<bool>) {
    // 20 ticks per sec, 1 tick every 50 mil-secs
    let mut interval = time::interval(Duration::from_millis(50));

    log_info!("SPAWNED TICK LOOP");

    //loop {
    //    interval.tick().await;

    //    let server = Server::get_instance().await;
    //    let mut server = server.lock().await;

    //    server.tick += 1;
    //    tokio::spawn(tick(server.tick.clone()));

    //    let update_motd_sender = update_motd_sender.clone();

    //    tokio::spawn(async move {
    //        let _ = update_motd_sender.send(true);
    //    });
    //}
}

async fn tick(tick: u128) {}
