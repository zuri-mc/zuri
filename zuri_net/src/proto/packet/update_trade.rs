use bytes::Bytes;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::inventory::Window;

/// Sent by the server to update the trades offered by a villager to a player. It is sent at the
/// moment that a player interacts with a villager.
#[derive(Debug, Clone)]
pub struct UpdateTrade {
    /// The trading window that the client currently has opened.
    pub window: Window,
    /// An identifier specifying the type of the window opened. In vanilla, it appears this is
    /// always filled out with fifteen.
    pub window_type: u8,
    /// The amount of trading options that the villager has.
    pub size: i32,
    /// The tier of the villager that the player is trading with. The tier starts at zero with the
    /// first two offers being available, after which two additional offers are unlocked each time
    /// the tier becomes one higher.
    pub trade_tier: i32,
    /// The unique ID of the villager entity that the player is trading with. The `trade_tier` sent
    /// above applies to this villager.
    pub villager_unique_id: i64,
    /// The unique ID of the entity (usually a player) for which the trades are updated. The updated
    /// trades may apply only to this entity.
    pub entity_unique_id: i64,
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

impl PacketType for UpdateTrade {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.window.to_u8().unwrap());
        writer.u8(self.window_type);
        writer.var_i32(self.size);
        writer.var_i32(self.trade_tier);
        writer.var_i64(self.villager_unique_id);
        writer.var_i64(self.entity_unique_id);
        writer.string(self.display_name.as_str());
        writer.bool(self.new_trade_ui);
        writer.bool(self.demand_based_prices);
        writer.byte_slice(&self.serialised_offers);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u8(reader.u8()).unwrap(),
            window_type: reader.u8(),
            size: reader.var_i32(),
            trade_tier: reader.var_i32(),
            villager_unique_id: reader.var_i64(),
            entity_unique_id: reader.var_i64(),
            display_name: reader.string(),
            new_trade_ui: reader.bool(),
            demand_based_prices: reader.bool(),
            serialised_offers: reader.byte_slice(),
        }
    }
}
