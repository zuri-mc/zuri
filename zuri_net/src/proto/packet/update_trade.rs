use crate::proto::ints::{VarI32, VarI64};
use bytes::Bytes;
use zuri_net_derive::proto;

use crate::proto::types::inventory::Window;

/// Sent by the server to update the trades offered by a villager to a player. It is sent at the
/// moment that a player interacts with a villager.
#[proto]
#[derive(Debug, Clone)]
pub struct UpdateTrade {
    /// The trading window that the client currently has opened.
    pub window: Window,
    /// An identifier specifying the type of the window opened. In vanilla, it appears this is
    /// always filled out with fifteen.
    pub window_type: u8,
    /// The amount of trading options that the villager has.
    pub size: VarI32,
    /// The tier of the villager that the player is trading with. The tier starts at zero with the
    /// first two offers being available, after which two additional offers are unlocked each time
    /// the tier becomes one higher.
    pub trade_tier: VarI32,
    /// The unique ID of the villager entity that the player is trading with. The `trade_tier` sent
    /// above applies to this villager.
    pub villager_unique_id: VarI64,
    /// The unique ID of the entity (usually a player) for which the trades are updated. The updated
    /// trades may apply only to this entity.
    pub entity_unique_id: VarI64,
    /// The name displayed at the top of the trading UI. It is usually used to represent the
    /// profession of the villager in the UI.
    pub display_name: String,
    /// Specifies if the villager should be using the new trade UI (The one added in v1.11.0) rather
    /// than the old one. This should usually be set to true.
    pub new_trade_ui: bool,
    /// Specifies if the prices of the villager's offers are modified by an increase in demand for
    /// the item. (A mechanic added in v1.11.0) Buying more of the same item will increase the price
    /// of that particular item.
    pub demand_based_prices: bool,
    /// Network NBT serialised compound of offers that the villager has.
    pub serialised_offers: Bytes,
}
