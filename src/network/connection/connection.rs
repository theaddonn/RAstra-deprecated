use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

use rak_rs::connection::queue::SendQueueError;
use rak_rs::connection::{Connection, RecvError};

pub(crate) type ConnNetChan = Arc<Mutex<Receiver<Vec<u8>>>>;

pub struct ConnectionWrapper {
    read_half: ConnectionReadHalf,
    write_half: ConnectionWriteHalf,
}

impl ConnectionWrapper {
    pub async fn new(connection: Connection) -> Self {
        let read_half = ConnectionReadHalf {
            internal_net_recv: connection.internal_net_recv.clone(),
        };

        let write_half = ConnectionWriteHalf { connection };

        Self {
            read_half,
            write_half,
        }
    }

    pub fn split(self) -> (ConnectionReadHalf, ConnectionWriteHalf) {
        (self.read_half, self.write_half)
    }
}

pub struct ConnectionReadHalf {
    internal_net_recv: ConnNetChan,
}

impl ConnectionReadHalf {
    pub async fn recv(&mut self) -> Result<Vec<u8>, RecvError> {
        let mut receiver = self.internal_net_recv.lock().await;

        receiver.recv().await.ok_or(RecvError::Closed)
    }
}

pub struct ConnectionWriteHalf {
    connection: Connection,
}

impl ConnectionWriteHalf {
    pub async fn send(&self, buffer: &[u8], immediate: bool) -> Result<(), SendQueueError> {
        self.connection.send(buffer, immediate).await
    }

    pub async fn close(&mut self) {
        self.connection.close().await;
    }
}
