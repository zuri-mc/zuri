use crate::block;
use crate::block::{BlockBuilder, BlockMap, RuntimeId, ToRuntimeId};
use crate::paletted_storage::{Palette, PalettedStorage};
use crate::pos::SubChunkIndex;
use zuri_net::proto::io::Reader;

pub const SUBCHUNK_SIZE: u16 = 16;

/// A 16x16x16 area that makes up part of a world chunk.
/// It consists of `L` layers which are used for things like waterlogged blocks.
#[derive(Clone)]
pub struct SubChunk<const L: usize> {
    _air_id: RuntimeId,
    layers: [PalettedStorage; L],
    // todo: biomes
}

impl<const L: usize> SubChunk<L> {
    /// Creates `L` paletted storages filled with `air_rid`.
    fn empty_layers(air_id: RuntimeId) -> [PalettedStorage; L] {
        let mut layers = Vec::with_capacity(L);
        for _ in 0..L {
            layers.push(PalettedStorage::new(vec![], Palette::new(vec![air_id])));
        }
        layers
            .iter()
            .cloned()
            .collect::<Vec<PalettedStorage>>()
            .try_into()
            .unwrap()
    }

    /// Creates a subchunk filled with `air_rid`.
    pub fn empty(air_id: RuntimeId) -> Self {
        Self {
            _air_id: air_id,
            layers: Self::empty_layers(air_id),
        }
    }

    pub fn at(&self, pos: SubChunkIndex, layer: u8) -> RuntimeId {
        if layer as usize >= L {
            panic!("layer {layer} is out of bounds");
        }
        self.layers[layer as usize].at(pos)
    }

    pub fn set(&mut self, pos: SubChunkIndex, layer: u8, val: RuntimeId) {
        if layer as usize >= L {
            panic!("layer {layer} is out of bounds")
        }
        self.layers[layer as usize].set(pos, val);
    }

    pub fn read(
        reader: &mut Reader,
        y_index: &mut u32,
        min_y_pos: i32,
        block_map: &BlockMap,
    ) -> Self {
        let air_rid = BlockBuilder::new(block::AIR_ID)
            .to_runtime_id(block_map)
            .expect("Missing air runtime id");

        // The first byte contains the chunk version. We support version 8 and 9.
        let ver = reader.u8();
        assert!(ver == 1 || ver == 8 || ver == 9);

        // Next up is the amount of layers in the sub chunk.
        let mut layer_count: u8 = 1;
        if ver > 1 {
            layer_count = reader.u8();
            if layer_count as usize >= L {
                panic!("Subchunk layer count overflows max supported layers");
            }

            // If the version is 9, there is an extra byte which tells us where the sub chunk is
            // positioned vertically in the chunk.
            if ver == 9 {
                let new_index = reader.u8();
                *y_index = (new_index as i32 - (min_y_pos >> 4)) as u32;
            }
        }

        // Now, reach each layer of the sub chunk.
        let mut layers = Self::empty_layers(air_rid);
        for current_layer in 0..layer_count {
            layers[current_layer as usize] = PalettedStorage::read(reader, block_map);
        }

        // todo: biomes
        Self {
            _air_id: air_rid,
            layers,
        }
    }
}
