use std::net::{IpAddr, SocketAddr};
use tokio::time;
use tokio::time::sleep;
use zuri_net::server::{Edition, Listener, Motd};

#[tokio::main]
async fn main() {
    // todo: remove temporary test
    let addr = SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 19132);
    let l = Listener::listen(
        &addr,
        &Motd {
            edition: Edition::Bedrock,
            local_motd: "Zuri".to_string(),
            motd: "Zuri Server".to_string(),
            player_count: 0,
            max_player_count: 1234,
        },
    )
    .await
    .unwrap();
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
}
