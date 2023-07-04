use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use glam::Affine3A;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;

/// Represents a file containing one or more different geometries.
#[derive(Deserialize, Debug, Clone)]
pub struct Geometries {
    pub format_version: String,
    #[serde(rename = "minecraft:geometry")]
    #[serde(default)]
    pub geometry: Vec<Geometry>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Geometry {
    pub description: GeometryDescription,
    /// The set of bones that make up the geometry. Each bone has a unique name, which is the key
    /// it is stored under in the map.
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_bone_map")]
    pub bones: BTreeMap<String, Bone>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeometryDescription {
    pub identifier: String,
    pub texture_width: u32,
    pub texture_height: u32,
    pub visible_bounds_width: u32,
    pub visible_bounds_height: u32,
    pub visible_bounds_offset: Vec3,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bone {
    pub pivot: Option<Vec3>,
    pub parent: Option<String>,
    pub rotation: Option<Vec3>,
    #[serde(default)]
    pub cubes: Vec<Cube>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cube {
    pub origin: Vec3,
    pub size: Vec3,
    pub inflate: Option<f32>,
    pub pivot: Option<Vec3>,
    pub rotation: Option<Vec3>,
    pub uv: UV,
}

/// A cube can have two different types of UV.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum UV {
    /// Contains the X and Y offset for the UV.
    BoxUV(Vec2),
    FaceUV(FaveUVList),
}

#[derive(Deserialize, Debug, Clone)]
pub struct FaveUVList {
    pub north: Option<FaceUV>,
    pub east: Option<FaceUV>,
    pub south: Option<FaceUV>,
    pub west: Option<FaceUV>,
    pub up: Option<FaceUV>,
    pub down: Option<FaceUV>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FaceUV {
    pub uv: Vec2,
    pub uv_size: Vec2,
}

fn deserialize_bone_map<'de, D>(deserializer: D) -> Result<BTreeMap<String, Bone>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug, Clone)]
    struct BoneWithName {
        pub name: String,
        #[serde(flatten)]
        pub bone: Bone,
    }

    let mut map = BTreeMap::new();

    let s: Vec<BoneWithName> = Deserialize::deserialize(deserializer)?;
    for elem in s {
        map.insert(elem.name, elem.bone);
    }

    Ok(map)
}

impl Geometry {
    /// Converts the geometry into a bevy [Mesh].
    pub fn as_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut uv = Vec::<[f32; 2]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut vertices = Vec::<[f32; 3]>::new();
        let mut triangles = Vec::<u32>::new();

        for (_, bone) in &self.bones {
            for cube in &bone.cubes {
                let mut start_index = vertices.len() as u32;

                let mut min = cube.origin.min(cube.origin + cube.size);
                let mut max = cube.origin.max(cube.origin + cube.size);
                if let Some(inflate) = cube.inflate {
                    min -= inflate;
                    max += inflate;
                }

                let mut corners = [
                    Vec3::new(min.x, min.y, min.z),
                    Vec3::new(max.x, min.y, min.z),
                    Vec3::new(max.x, max.y, min.z),
                    Vec3::new(min.x, max.y, min.z),
                    Vec3::new(min.x, min.y, max.z),
                    Vec3::new(max.x, min.y, max.z),
                    Vec3::new(max.x, max.y, max.z),
                    Vec3::new(min.x, max.y, max.z),
                ];
                if let Some(rotation) = cube.rotation {
                    fn rotate_axis(pos: Vec3, angle: f32, axis: u8) -> Vec3 {
                        let transform = match axis {
                            1 => Affine3A::from_rotation_y(angle.to_radians()),
                            2 => Affine3A::from_rotation_z(angle.to_radians()),
                            _ => Affine3A::from_rotation_x(angle.to_radians()),
                        };
                        transform.transform_vector3(pos)
                    }

                    let pivot = cube.pivot.unwrap_or(Vec3::ZERO);
                    for corner in &mut corners {
                        *corner -= pivot;
                        *corner = rotate_axis(*corner, rotation.x, 0);
                        *corner = rotate_axis(*corner, rotation.y, 1);
                        *corner = rotate_axis(*corner, -rotation.z, 2);
                        *corner += pivot;
                    }
                }

                macro_rules! vertices {
                    ($l:expr $(, $e:expr)* $(,)?) => {
                        vertices.push(corners[$l].into());
                        $(vertices.push(corners[$e].into());)+
                    };
                }
                macro_rules! triangle {
                    ($a:literal, $b:literal, $c:literal) => {
                        triangles.push(start_index + $a);
                        triangles.push(start_index + $b);
                        triangles.push(start_index + $c);
                    };
                }

                // todo: project UV
                if let UV::FaceUV(face_uv) = &cube.uv {
                    if let Some(_uv) = &face_uv.down {
                        vertices!(0, 1, 4, 5);

                        for _ in 0..4 {
                            normals.push([0., -1., 0.]);
                        }

                        triangle!(0, 1, 2);
                        triangle!(2, 1, 3);
                        start_index += 4;
                    }
                    if let Some(_uv) = &face_uv.up {
                        vertices!(3, 2, 7, 6);

                        for _ in 0..4 {
                            normals.push([0., 1., 0.]);
                        }

                        triangle!(2, 1, 0);
                        triangle!(3, 1, 2);
                        start_index += 4;
                    }
                    if let Some(_uv) = &face_uv.south {
                        vertices!(0, 1, 3, 2);
                        for _ in 0..4 {
                            normals.push([0., 0., -1.]);
                        }

                        triangle!(2, 1, 0);
                        triangle!(2, 1, 3);
                        start_index += 4;
                    }
                    if let Some(_uv) = &face_uv.east {
                        vertices!(1, 5, 2, 6);

                        for _ in 0..4 {
                            normals.push([1., 0., 0.]);
                        }

                        triangle!(2, 1, 0);
                        triangle!(3, 1, 2);
                        start_index += 4;
                    }
                    if let Some(_uv) = &face_uv.north {
                        // todo: figure out the correct values here (blame seb)
                        vertices!(0, 1, 3, 2);

                        for _ in 0..4 {
                            normals.push([0., 0., 1.]);
                        }

                        triangle!(2, 1, 0);
                        triangle!(3, 1, 2);
                        start_index += 4;
                    }
                    if let Some(_uv) = &face_uv.west {
                        vertices!(0, 4, 3, 7);

                        for _ in 0..4 {
                            normals.push([-1., 0., 0.]);
                        }

                        triangle!(0, 1, 2);
                        triangle!(2, 1, 3);
                    }
                }
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U32(triangles)));
        mesh
    }
}

#[cfg(test)]
mod tests {
    use crate::Geometries;

    #[test]
    fn test_deserialize() {
        let test_cases = [
            r#"{"format_version":"1.12.0","minecraft:geometry":[{"description":{"identifier":"geometry.test","texture_width":16,"texture_height":16,"visible_bounds_width":3,"visible_bounds_height":3,"visible_bounds_offset":[0,0.5,0]},"bones":[{"name":"bb_main","pivot":[0,0,0],"cubes":[{"origin":[-1,0,-1],"size":[15,7,11],"inflate":1,"pivot":[0,0,0],"rotation":[45,0,47.5],"uv":{"north":{"uv":[2,2],"uv_size":[15,7]},"east":{"uv":[-9,2],"uv_size":[11,7]},"south":{"uv":[28,2],"uv_size":[15,7]},"west":{"uv":[17,2],"uv_size":[11,7]},"up":{"uv":[2,-9],"uv_size":[15,11]},"down":{"uv":[17,2],"uv_size":[15,-11]}}}]},{"name":"bone","pivot":[0,0,0],"rotation":[0,0,-60],"cubes":[{"origin":[-9,0.90461,-6.1303],"size":[10,18,2],"pivot":[0,15,0],"rotation":[29.52015,-46.04179,-22.17599],"uv":{"north":{"uv":[2,2],"uv_size":[10,18]},"east":{"uv":[0,2],"uv_size":[2,18]},"south":{"uv":[14,2],"uv_size":[10,18]},"west":{"uv":[12,2],"uv_size":[2,18]},"up":{"uv":[2,0],"uv_size":[10,2]},"down":{"uv":[12,2],"uv_size":[10,-2]}}}]}]}]}"#,
            r#"{"format_version":"1.12.0","minecraft:geometry":[{"description":{"identifier":"geometry.test","texture_width":16,"texture_height":16,"visible_bounds_width":3,"visible_bounds_height":3,"visible_bounds_offset":[0,0.5,0]},"bones":[{"name":"bb_main","pivot":[0,0,0],"cubes":[{"origin":[-1,0,-1],"size":[15,7,11],"inflate":1,"pivot":[0,0,0],"rotation":[45,0,47.5],"uv":[-9,-9]}]},{"name":"bone","pivot":[0,0,0],"rotation":[0,0,-60],"cubes":[{"origin":[-9,0.90461,-6.1303],"size":[10,18,2],"pivot":[0,15,0],"rotation":[29.52015,-46.04179,-22.17599],"uv":[0,0]}]}]}]}"#,
        ];
        for (i, test_case) in test_cases.into_iter().enumerate() {
            let result = serde_json::from_str::<Geometries>(test_case);
            if let Err(err) = result {
                panic!("Could not deserialise geometry (test case index {i}): {err}");
            }
        }
    }
}
