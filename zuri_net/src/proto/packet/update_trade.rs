use bytes::Bytes;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::inventory::Window;

#[derive(Debug)]
pub struct UpdateTrade {
    pub window: Window,
    pub window_type: u8,
    pub size: i32,
    pub trade_tier: i32,
    pub villager_unique_id: i64,
    pub entity_unique_id: i64,
    pub display_name: String,
    pub new_trade_ui: bool,
    pub demand_based_prices: bool,
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
