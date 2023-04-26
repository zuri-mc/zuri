use crate::chan::PkReceiver;
use crate::connection::{ConnError, Connection, ExpectedPackets, Sequence};
use crate::proto::packet::disconnect::Disconnect;
use crate::proto::packet::request_network_settings::RequestNetworkSettings;
use crate::proto::packet::Packet;
use crate::proto::CURRENT_PROTOCOL;
use async_trait::async_trait;

pub struct LoginSequence {}

#[async_trait]
impl Sequence<Result<(), ConnError>> for LoginSequence {
    async fn execute<'b>(
        self,
        mut reader: PkReceiver,
        conn: &'b Connection,
        expectancies: &'b ExpectedPackets,
    ) -> Result<(), ConnError> {
        // Phase 1: Network settings.
        {
            expectancies.queue::<RequestNetworkSettings>().await;
            let req_net_set = RequestNetworkSettings::try_from(reader.recv().await).unwrap();
            // Disconnect the player if the protocol does not match.
            if req_net_set.client_protocol.0 != 123 {
                conn.write_packet(&Packet::from(Disconnect {
                    message: Some(format!(
                        "Incompatible client version: expected {}, got {}",
                        CURRENT_PROTOCOL, req_net_set.client_protocol.0
                    )),
                }))
                .await;
                conn.flush().await?;
                conn.close().await?;
                return Ok(()); // todo: return something other than this
            }


        }

        Ok(())
    }
}
