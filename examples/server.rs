use std::net::{IpAddr, SocketAddr};
use tokio::time;
use tokio::time::sleep;
use zuri_net::server::Listener;

#[tokio::main]
async fn main() {
    // todo: remove temporary test
    let addr = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 19132);
    let mut l = Listener::listen(&addr).await.unwrap();
    while let Some(conn) = l.accept().await {
        match conn {
            Ok(conn) => {
                let conn2 = conn.clone();
                tokio::spawn(async move {
                    let conn = conn2;
                    while let Ok(pk) = conn.read_next_packet().await {
                        println!("Unhandled packet: {:?}", pk);
                    }
                });

                tokio::spawn(async move {
                    loop {
                        if conn.flush().await.is_err() {
                            return;
                        }
                        sleep(time::Duration::from_secs_f32(1. / 20.)).await;
                    }
                });
            }
            Err(err) => panic!("err: {:?}", err),
        };
    }
    sleep(time::Duration::from_secs(100)).await;
}
