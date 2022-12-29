use tokio::sync::mpsc;
use crate::proto::packet::Packet;

pub fn pk_chan() -> (PkSender, PkReceiver) {
    let (s0, r0) = mpsc::channel(1);
    let (s1, r1) = mpsc::channel(1);
    (
        PkSender {
            r: r1,
            s: s0,
        },
        PkReceiver {
            r: r0,
            s: s1,
        },
    )
}

#[derive(Debug)]
pub struct PkReceiver {
    r: mpsc::Receiver<Packet>,
    s: mpsc::Sender<()>,
}

impl PkReceiver {
    #[must_use]
    pub async fn recv(&mut self) -> Packet {
        self.s.send(()).await.unwrap();
        let pk = self.r.recv().await.unwrap();
        pk
    }
}

#[derive(Debug)]
pub struct PkSender {
    r: mpsc::Receiver<()>,
    s: mpsc::Sender<Packet>,
}

impl PkSender {
    #[must_use]
    pub async fn send(&mut self, pk: Packet) -> bool {
        if self.s.send(pk).await.is_err() {
            return false;
        }
        self.r.recv().await.unwrap();
        true
    }
}
