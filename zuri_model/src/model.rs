use bevy::math::Affine3A;
use bevy::prelude::{Component, Mesh, Rect, Vec2, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use zuri_math::facing::Face;
use crate::geometry::{FaceUV, FaceUVList, Geometry, UV};

#[derive(Component, Debug)]
pub struct Model {
    geometry: Geometry,
    options: ModelOptions,
}

#[derive(Debug, Default)]
pub struct ModelOptions {
    pub texture_size: Option<Vec2>,
    pub texture_offset: Option<Vec2>,
}

impl Model {
    pub fn new(geometry: Geometry, options: ModelOptions) -> Self {
        return Self { geometry, options };
    }

    pub fn build_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut uv = Vec::<[f32; 2]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut vertices = Vec::<[f32; 3]>::new();
        let mut triangles = Vec::<u32>::new();
        for bone in &self.geometry.bones {
            if bone.cubes.is_some() {
                for cube in (&mut bone.cubes.as_ref()).unwrap() {
                    let mut start_index = vertices.len() as u32;

                    let mut min = cube.origin.min(cube.origin + cube.size);
                    let mut max = cube.origin.max(cube.origin + cube.size);
                    if cube.inflate.is_some() {
                        let inflate = cube.inflate.unwrap();
                        min = min - inflate;
                        max = max + inflate;
                    }

                    let mut corners = Vec::<Vec3>::new();
                    corners.push(Vec3::new(min.x, min.y, min.z));
                    corners.push(Vec3::new(max.x, min.y, min.z));
                    corners.push(Vec3::new(max.x, max.y, min.z));
                    corners.push(Vec3::new(min.x, max.y, min.z));
                    corners.push(Vec3::new(min.x, min.y, max.z));
                    corners.push(Vec3::new(max.x, min.y, max.z));
                    corners.push(Vec3::new(max.x, max.y, max.z));
                    corners.push(Vec3::new(min.x, max.y, max.z));
                    if cube.rotation.is_some() {
                        let pivot = cube.pivot.unwrap_or(Vec3::ZERO);
                        corners = corners.into_iter().map(|pos| {
                            let mut pos = pos - pivot;
                            pos = Self::rotate_axis(pos, cube.rotation.unwrap().x, 0);
                            pos = Self::rotate_axis(pos, cube.rotation.unwrap().y, 1);
                            pos = Self::rotate_axis(pos, -cube.rotation.unwrap().z, 2);
                            pos + pivot
                        }).collect();
                    }

                    if cube.uv_list.down.is_some() {
                        vertices.push(corners[0].into());
                        vertices.push(corners[1].into());
                        vertices.push(corners[4].into());
                        vertices.push(corners[5].into());

                        self.project_uv(&mut uv, cube.uv_list, Face::Down);

                        normals.push([0., -1., 0.]);
                        normals.push([0., -1., 0.]);
                        normals.push([0., -1., 0.]);
                        normals.push([0., -1., 0.]);

                        triangles.push(start_index + 0);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);

                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 3);

                        start_index += 4;
                    }
                    if cube.uv_list.up.is_some() {
                        vertices.push(corners[3].into());
                        vertices.push(corners[2].into());
                        vertices.push(corners[7].into());
                        vertices.push(corners[6].into());

                        self.project_uv(&mut uv, cube.uv_list, Face::Up);

                        normals.push([0., 1.0, 0.]);
                        normals.push([0., 1.0, 0.]);
                        normals.push([0., 1.0, 0.]);
                        normals.push([0., 1.0, 0.]);

                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 0);

                        triangles.push(start_index + 3);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);

                        start_index += 4;
                    }
                    if cube.uv_list.south.is_some() {
                        vertices.push(corners[0].into());
                        vertices.push(corners[1].into());
                        vertices.push(corners[3].into());
                        vertices.push(corners[2].into());

                        self.project_uv(&mut uv, cube.uv_list, Face::North);

                        normals.push([0., 0., -1.0]);
                        normals.push([0., 0., -1.0]);
                        normals.push([0., 0., -1.0]);
                        normals.push([0., 0., -1.0]);

                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 0);

                        triangles.push(start_index + 3);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);

                        start_index += 4;
                    }
                    if cube.uv_list.south.is_some() {
                        vertices.push(corners[4].into());
                        vertices.push(corners[5].into());
                        vertices.push(corners[7].into());
                        vertices.push(corners[6].into());

                        self.project_uv(&mut uv, cube.uv_list, Face::South);

                        normals.push([0., 0., 1.0]);
                        normals.push([0., 0., 1.0]);
                        normals.push([0., 0., 1.0]);
                        normals.push([0., 0., 1.0]);

                        triangles.push(start_index + 0);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);

                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 3);

                        start_index += 4;
                    }
                    if cube.uv_list.east.is_some() {
                        vertices.push(corners[1].into());
                        vertices.push(corners[5].into());
                        vertices.push(corners[2].into());
                        vertices.push(corners[6].into());

                        self.project_uv(&mut uv, cube.uv_list, Face::East);

                        normals.push([1., 0., 0.]);
                        normals.push([1., 0., 0.]);
                        normals.push([1., 0., 0.]);
                        normals.push([1., 0., 0.]);

                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 0);

                        triangles.push(start_index + 3);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);

                        start_index += 4;
                    }
                    if cube.uv_list.west.is_some() {
                        vertices.push(corners[0].into());
                        vertices.push(corners[4].into());
                        vertices.push(corners[3].into());
                        vertices.push(corners[7].into());

                        self.project_uv(&mut uv, cube.uv_list, Face::West);

                        normals.push([-1., 0., 0.]);
                        normals.push([-1., 0., 0.]);
                        normals.push([-1., 0., 0.]);
                        normals.push([-1., 0., 0.]);

                        triangles.push(start_index + 0);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);

                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 3);
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

    fn project_uv(&self, uv: &mut Vec<[f32; 2]>, uv_list: FaceUVList, face: Face) {
        let texture_size = if self.options.texture_size.is_some() {
            self.options.texture_size.unwrap()
        } else {
            Vec2::new(self.geometry.description.texture_width as f32, self.geometry.description.texture_height as f32)
        };
        let texture_offset = if self.options.texture_offset.is_some() {
            self.options.texture_offset.unwrap()
        } else {
            Vec2::default()
        };
        let face_uv = match face {
            Face::Down => uv_list.down,
            Face::Up => uv_list.up,
            Face::North => uv_list.north,
            Face::South => uv_list.south,
            Face::East => uv_list.east,
            Face::West => uv_list.west,
        }.unwrap();
        let min = face_uv.uv.min(face_uv.uv + face_uv.uv_size) / texture_size + texture_offset;
        let max = face_uv.uv.max(face_uv.uv + face_uv.uv_size) / texture_size + texture_offset;

        match face {
            Face::South => {
                uv.push([min.x, max.y]);
                uv.push([max.x, max.y]);
                uv.push([min.x, min.y]);
                uv.push([max.x, min.y]);
            }
            Face::West | Face::Up => {
                uv.push([max.x, max.y]);
                uv.push([min.x, max.y]);
                uv.push([max.x, min.y]);
                uv.push([min.x, min.y]);
            }
            Face::East => {
                uv.push([max.x, max.y]);
                uv.push([min.x, max.y]);
                uv.push([max.x, min.y]);
                uv.push([min.x, min.y]);
            }
            Face::North => {
                uv.push([max.x, max.y]);
                uv.push([min.x, max.y]);
                uv.push([max.x, min.y]);
                uv.push([min.x, min.y]);
            }
            Face::Down => {
                uv.push([max.x, min.y]);
                uv.push([min.x, min.y]);
                uv.push([max.x, max.y]);
                uv.push([min.x, max.y]);
            }
        }
    }

    fn rotate_axis(mut pos: Vec3, angle: f32, axis: u8) -> Vec3 {
        let transform = match axis {
            1 => Affine3A::from_rotation_y(angle.to_radians()),
            2 => Affine3A::from_rotation_z(angle.to_radians()),
            _ => Affine3A::from_rotation_x(angle.to_radians())
        };
        transform.transform_vector3(pos)
    }
}