use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_nbt::Value;
use zuri_net::proto::io::Reader;

use crate::pos::SubChunkIndex;

#[derive(Clone, Debug)]
pub struct Palette {
    mapping: Vec<u32>,
}

impl Palette {
    pub fn new(mapping: Vec<u32>) -> Self {
        if mapping.len() == 0 {
            panic!("Palette must contain at least 1 entry");
        }
        Self {
            mapping,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PalettedStorage {
    bits_per_index: u16,
    index_mask: u32,
    filled_bits_per_index: u16,

    palette: Palette,
    indices: Vec<u32>,
}

impl PalettedStorage {
    pub fn new(indices: Vec<u32>, palette: Palette) -> Self {
        let bits_per_index = (indices.len() / 32 / 4) as u16;
        let index_mask = (1u32 << bits_per_index) - 1;
        let mut filled_bits_per_index = 0u16;
        if bits_per_index != 0 {
            filled_bits_per_index = 32 / bits_per_index * bits_per_index;
        }

        Self {
            bits_per_index,
            index_mask,
            filled_bits_per_index,

            palette,
            indices,
        }
    }

    pub fn at(&self, pos: SubChunkIndex) -> u32 {
        let palette_index = if self.bits_per_index == 0 {
            0
        } else {
            let offset = (((pos.x() as u16) << 8)
                | ((pos.z() as u16) << 4)
                | (pos.y() as u16)) * self.bits_per_index;

            let u32_offset = offset / self.filled_bits_per_index;
            let bit_offset = offset % self.filled_bits_per_index;
            ((self.indices[u32_offset as usize] >> bit_offset) as u32) & self.index_mask
        };
        self.palette.mapping[palette_index as usize]
    }


    pub fn read(reader: &mut Reader) -> PalettedStorage {
        // The first byte encodes two values: the first 7 bits denote the amount of bits each index
        // takes in the index vector. The last gives info about how the palette is structured,
        let (bits_per_index, nbt_palette) = {
            let temp = reader.u8();
            (temp >> 1, temp & 1 != 1)
        };

        // We calculate the amount of `u32`s needed to store all the indices of the paletted
        // storage. If the bits_per_index is zero, no data is used to store the indices.
        let mut index_u32_count: i32 = 0;
        if bits_per_index != 0 {
            // Dividing 32 (the amount of bits in a u32) by the bits per index gives us the maximum
            // amount of indices we can store per u32. The remainder of this division can be
            // ignored: it will be unused padding.
            let indices_per_u32 = 32 / bits_per_index as i32;
            // THe total amount of u32s needed can simply be calculated by dividing 4096 (the amount
            // of indices we need to store) by the indices per u32.
            index_u32_count = 4096 / indices_per_u32;
        }
        // If the amount of bytes per index is either 3, 5 or 6 we need an extra u32 to accommodate
        // for the last index.
        if bits_per_index == 3 || bits_per_index == 5 || bits_per_index == 6 {
            index_u32_count += 1;
        }

        // Read the actual data: n u32s, where n is the amount we previously calculated. The u32s
        // are encoded as little endian.
        let mut u32s = Vec::<u32>::with_capacity(index_u32_count as usize);
        for _ in 0..index_u32_count {
            u32s.push(reader.u32());
        }

        // Read the total amount of unique entries that are stored in the palette. If bits per index
        // is zero (= the length of the indices is also zero), the whole paletted storage consists
        // of only the single block type found in the palette.
        let palette_size = if bits_per_index != 0 {
            reader.var_i32() as usize
        } else {
            1
        };
        // For some reason, there are two different ways to encode a palette.
        let mut palette = Vec::with_capacity(palette_size);
        if !nbt_palette {
            // In most cases, the palette is just encoded as a vector of `var_i32`s.
            for _ in 0..palette_size {
                palette.push(reader.var_i32() as u32);
            }
        } else {
            // The palette can be encoded with nbt. In this case, each entry is a compound tag with
            // the namespaced block id and the block state.
            for _ in 0..palette_size {
                let nbt = reader.nbt(NetworkLittleEndian);
                if let Value::Compound(map) = nbt {
                    if let Value::String(name) = map.get("name").unwrap() {
                        if name == "air" {
                            palette.push(10462);
                            continue
                        }
                        palette.push(0);
                    }
                } else {
                    panic!("unexpected value type for root in nbt palette");
                }
            }
        }

        Self::new(u32s, Palette::new(palette))
    }
}
