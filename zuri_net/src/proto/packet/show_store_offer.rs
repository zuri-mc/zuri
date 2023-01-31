use zuri_net_derive::packet;

/// Sent by the server to show a Marketplace store offer to a player. It opens a window client-side
/// that displays the item. The ShowStoreOffer packet only works on the partnered servers: Servers
/// that are not partnered will not have a store button show up in the in-game pause menu and will,
/// as a result, not be able to open store offers on the client side. Sending the packet does
/// therefore not work when using a proxy that is not connected to with the domain of one of the
/// partnered servers.
#[packet]
#[derive(Debug, Clone)]
pub struct ShowStoreOffer {
    /// A string that identifies the offer for which a window should be opened. While typically a
    /// UUID, the ID could be anything.
    pub offer_id: String,
    /// Specifies if all other offers of the same 'author' as the one of the offer associated with
    /// the `offer_id` should also be displayed, alongside the target offer.
    pub show_all: bool,
}
