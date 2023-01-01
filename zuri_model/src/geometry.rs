use bevy::math::{Affine3A, Vec2, Vec3};
use bevy::prelude::{EulerRot, Mesh, Quat};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GeometryList {
    pub format_version: String,
    #[serde(rename = "minecraft:geometry")]
    pub geometry: Vec<Geometry>,
}

impl GeometryList {
    pub fn new(data: &str) -> Vec<Geometry> {
        let mut list: GeometryList = serde_json::from_str(data).unwrap();
        list.geometry.iter_mut().for_each(|geometry| {
            for bone in &mut geometry.bones {
                if bone.cubes.is_some() {
                    for cube in bone.cubes.as_mut().unwrap() {
                        match &cube.uv {
                            UV::Box(uv) => {
                                cube.uv_list = uv.per_face(&cube)
                            }
                            UV::Face(uv) => {
                                cube.uv_list = (*uv).clone()
                            }
                        }
                    }
                }
            }
        });
        list.geometry
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Geometry {
    pub description: Description,
    pub bones: Vec<Bone>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Description {
    pub identifier: String,
    pub texture_width: u32,
    pub texture_height: u32,
    pub visible_bounds_width: f32,
    pub visible_bounds_height: f32,
    pub visible_bounds_offset: [f32; 3],
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Bone {
    pub name: String,
    pub parent: Option<String>,
    pub pivot: Option<Vec3>,
    pub rotation: Option<Vec3>,
    pub cubes: Option<Vec<Cube>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cube {
    pub origin: Vec3,
    pub size: Vec3,
    pub pivot: Option<Vec3>,
    pub rotation: Option<Vec3>,
    pub inflate: Option<f32>,
    pub uv: UV,
    #[serde(skip_deserializing,skip_serializing)]
    pub uv_list: FaceUVList,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct FaceUVList {
    pub north: Option<FaceUV>,
    pub south: Option<FaceUV>,
    pub east: Option<FaceUV>,
    pub west: Option<FaceUV>,
    pub up: Option<FaceUV>,
    pub down: Option<FaceUV>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct FaceUV {
    pub uv: Vec2,
    pub uv_size: Vec2,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BoxUV {
    pub offset_x: f32,
    pub offset_y: f32,
}

impl BoxUV {
    pub fn per_face(&self, cube: &Cube) -> FaceUVList {
        let size = cube.size.abs();
        let size_ud = Vec2::new(size.x, size.z);
        let size_ns = Vec2::new(size.x, size.y);
        let size_ew = Vec2::new(size.z, size.y);
        FaceUVList {
            north: Some(FaceUV{uv: Vec2::new(self.offset_x + size.z + size.x + size.z, self.offset_y + size.z), uv_size: size_ns}),
            south: Some(FaceUV{uv: Vec2::new(self.offset_x + size.z, self.offset_y + size.z), uv_size: size_ns}),
            east: Some(FaceUV{uv: Vec2::new(self.offset_x + size.z + size.x, self.offset_y + size.z), uv_size: size_ew}),
            west: Some(FaceUV{uv: Vec2::new(self.offset_x, self.offset_y + size.z), uv_size: size_ew}),
            up: Some(FaceUV{uv: Vec2::new(self.offset_x + size.z, self.offset_y), uv_size: size_ud}),
            down: Some(FaceUV{uv: Vec2::new(self.offset_x + size.z + size.x, self.offset_y), uv_size: size_ud}),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum UV {
    Box(BoxUV),
    Face(FaceUVList),
}
