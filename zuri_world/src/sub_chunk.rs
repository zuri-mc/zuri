use zuri_net::proto::io::Reader;
use crate::paletted_storage::{Palette, PalettedStorage};
use crate::pos::SubChunkIndex;

#[derive(Clone)]
pub struct SubChunk<const L: usize> {
    air_id: u32,
    layers: [PalettedStorage; L],
    // todo: biomes
}

impl<const L: usize> SubChunk<L> {
    fn empty_layers(air_id: u32) -> [PalettedStorage; L] {
        let mut layers = Vec::with_capacity(L);
        for _ in 0..L {
            layers.push(PalettedStorage::new(vec![], Palette::new(vec![air_id])));
        }
        layers.iter().cloned().collect::<Vec<PalettedStorage>>().try_into().unwrap()
    }

    pub fn empty(air_id: u32) -> Self {
        Self {
            air_id,
            layers: Self::empty_layers(air_id),
        }
    }

    pub fn at(&self, pos: SubChunkIndex, layer: u8) -> u32 {
        if layer as usize >= L {
            panic!("layer {layer} is out of bounds");
        }
        self.layers[layer as usize].at(pos)
    }

    pub fn set(&mut self, _pos: SubChunkIndex, layer: u8, _val: u32) {
        if layer as usize >= L {
            panic!("layer {layer} is out of bounds")
        }
        todo!()
    }

    pub fn read(reader: &mut Reader, y_index: &mut u32, air_id: u32) -> Self {
        // The first byte contains the chunk version. We support version 8 and 9.
        let ver = reader.u8();
        assert!(ver == 1 || ver == 8 || ver == 9);

        // Next up is the amount of layers in the sub chunk.
        let mut layer_count: u8 = 1;
        if ver > 1 {
            layer_count = reader.u8();
            if layer_count as usize >= L {
                panic!("SubChunk layer count overflows may supported layers");
            }

            // If the version is 9, there is an extra byte which tells us where the sub chunk is
            // positioned vertically in the chunk.
            if ver == 9 {
                reader.u8();
                // todo: this doesnt work
                //*y_index = reader.u8() as u32;
            }
        }

        // Now, reach each layer of the sub chunk.
        let mut layers = Self::empty_layers(air_id);
        for current_layer in 0..layer_count {
            layers[current_layer as usize] = PalettedStorage::read(reader);
        }

        // todo: biomes
        Self {
            air_id,
            layers,
        }
    }
}
