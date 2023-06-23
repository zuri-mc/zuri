use crate::block::component::geometry::Geometry;
use crate::block::component::ComponentStorageType;
use crate::block::{BlockMapBuilder, BlockType, PropertyValues};
use bytes::Bytes;
use std::collections::HashMap;
use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_nbt::{tag, NBTTag};

/// Returns a base [BlockMapBuilder] containing all vanilla block states.
pub fn vanilla_block_map() -> BlockMapBuilder {
    const BLOCK_STATES: &[u8] = include_bytes!("block_states.nbt");

    let mut nbt_stream = Bytes::from(BLOCK_STATES);
    let mut vanilla_block_states: HashMap<Box<str>, HashMap<Box<str>, PropertyValues>> =
        Default::default();
    while !nbt_stream.is_empty() {
        let nbt: tag::Compound = NBTTag::read(&mut nbt_stream, &mut NetworkLittleEndian)
            .expect("could not decode nbt")
            .try_into()
            .unwrap();

        let name: &str = if let NBTTag::String(s) = &nbt.0["name"] {
            s.as_str()
        } else {
            panic!("Disallowed tag type for `name` field");
        };

        if !vanilla_block_states.contains_key(name) {
            vanilla_block_states.insert(Box::from(name), HashMap::new());
        }
        let property_map = vanilla_block_states.get_mut(name).unwrap();

        let states_list: tag::Compound = nbt.0["states"].clone().try_into().unwrap();
        for (name, val) in states_list.0.iter().map(|(k, v)| (k.as_str(), v)) {
            if !property_map.contains_key(name) {
                property_map.insert(
                    Box::from(name),
                    match val {
                        NBTTag::Byte(_) => PropertyValues::Bool,
                        NBTTag::Int(_) => PropertyValues::Ints(Default::default()),
                        NBTTag::String(_) => PropertyValues::Strings(Default::default()),
                        default => panic!(
                            "Disallowed tag type for property value: `{}`",
                            default.tag_type()
                        ),
                    },
                );
            }

            match property_map.get_mut(name).unwrap() {
                PropertyValues::Strings(set) => {
                    if let NBTTag::String(val) = val {
                        set.push(Box::from(val.as_str()));
                    } else {
                        panic!(
                            "Disallowed tag type for property value: `{}`",
                            val.tag_type()
                        );
                    }
                }
                PropertyValues::Ints(set) => {
                    if let NBTTag::Int(val) = val {
                        set.push(val.0);
                    } else {
                        panic!(
                            "Disallowed tag type for property value: `{}`",
                            val.tag_type()
                        );
                    }
                }
                PropertyValues::Bool => {}
            }
        }
    }

    let mut block_map =
        BlockMapBuilder::empty().with_component_type::<Geometry>(ComponentStorageType::Vector);

    for (name, properties) in vanilla_block_states {
        let mut block_type = BlockType::new(name);
        for (name, values) in properties {
            block_type.insert_property(name, values);
        }
        block_map.insert_block(block_type);
    }

    block_map
    /*    let mut block_map = block_map.build();

    block_map.set_component(
        BlockBuilder::new("minecraft:air"),
        Geometry {
            mesh: Mesh::new(PrimitiveTopology::TriangleList),
        },
    );
    {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let vertices: Vec<[f32; 3]> = vec![[0., 0., 0.], [0., 1., 0.], [1., 0., 1.], [1., 1., 1.]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        let normals: Vec<[f32; 3]> = vec![[1., 0., 0.], [1., 0., 0.], [1., 0., 0.], [1., 0., 0.]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        let uvs: Vec<[f32; 2]> = vec![[0., 0.], [0., 1.], [1., 0.], [1., 1.]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 1, 2, 3, 2, 1, 0, 3, 2, 1])));

        let mut rids = Vec::new();
        for block in BlockBuilder::new("minecraft:tallgrass").matches(&block_map) {
            rids.push(block.to_runtime_id(&block_map));
        }
        for rid in rids {
            block_map.set_component(rid, Geometry { mesh: mesh.clone() });
        }
    }

    block_map*/
}
