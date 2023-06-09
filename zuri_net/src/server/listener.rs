use std::net::SocketAddr;
use std::sync::Arc;

use rust_raknet::error::RaknetError;
use rust_raknet::RaknetListener;
use tokio::sync::Mutex;

use crate::connection::Connection;
use crate::server::login::CompressionSettings;
use crate::server::{LoginSequence, Motd};

/// Server listener that listens to incoming client connections on a certain [SocketAddr]. Incoming
/// connections can be handled using the [Listener::accept] method. Before connections are passed
/// along, minecraft's login sequence is performed.
pub struct Listener {
    /// The underlying RakNet listener.
    listener: Arc<Mutex<RaknetListener>>,
}

impl Listener {
    /// Starts listening and accepting players on the given [SocketAddr]. A handler is passed to
    /// handle incoming connections.
    pub async fn listen(addr: &SocketAddr, motd: &Motd) -> Result<Listener, RaknetError> {
        let mut listener = RaknetListener::bind(addr).await?;
        listener
            .set_full_motd(motd.serialize(234872937684, addr.port()))
            .unwrap(); // todo: get the actual server id
        listener.listen().await;
        let listener = Arc::new(Mutex::new(listener));

        Ok(Listener { listener })
    }

    /// Accepts a new incoming minecraft connection.
    ///
    /// Returns [None] if the listener is no longer listening.
    pub async fn accept(&self) -> Option<Result<Arc<Connection>, RaknetError>> {
        let mut listener = self.listener.lock().await;

        let conn = listener.accept().await;
        if let Err(err) = conn {
            return match err {
                RaknetError::NotListen => None,
                _ => Some(Err(err)),
            };
        }
        let conn = Arc::new(Connection::new(conn.unwrap()));

        let seq_conn = conn.clone();
        tokio::spawn(async move {
            let conn = seq_conn;
            let res = conn
                .exec_sequence(LoginSequence {
                    xbox_auth: false,
                    compression: CompressionSettings::default(),
                })
                .await;
            if let Err(err) = res {
                log::debug!("login sequence for `{}` failed: {}", conn.peer_addr(), err);
            }
        });

        Some(Ok(conn))
    }

    /// Stops listening for and accepting new connections.
    ///
    /// Can be called even when the listener is possibly already closed.
    pub async fn close(&self) {
        self.listener
            .lock()
            .await
            .close()
            .await
            .expect("unreachable");
    }

    /// Changes the MOTD displayed in minecraft clients.
    pub async fn set_motd(&self, motd: &Motd) {
        // todo: make this function not wait for mutex lock when possible (will require new raknet)
        let mut listener = self.listener.lock().await;
        let port = listener.local_addr().expect("unreachable").port();
        listener
            .set_full_motd(motd.serialize(234872937684, port))
            .expect("unreachable");
    }
}
