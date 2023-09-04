use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use geo::{polygon, BooleanOps, ChamberlainDuquetteArea, Coord, MultiPolygon, Triangle};
use zuri_world::block;

/// Stores a block's geometry. This geometry is the geometry that will be used in the world for this
/// block.
#[derive(Debug, Clone, block::Component)]
pub struct Geometry {
    /// The actual block mesh.
    pub(super) mesh: Mesh,

    pub(super) x_pos_solid: bool,
    pub(super) x_neg_solid: bool,
    pub(super) y_pos_solid: bool,
    pub(super) y_neg_solid: bool,
    pub(super) z_pos_solid: bool,
    pub(super) z_neg_solid: bool,
}

impl Geometry {
    /// Creates a new geometry component from a mesh.
    ///
    /// Performs the necessary calculations in order to make the mesh optimal. This is a slow
    /// operation.
    pub fn from_mesh(mesh: Mesh) -> Self {
        let mut geo = Self {
            mesh,
            x_pos_solid: false,
            x_neg_solid: false,
            y_pos_solid: false,
            y_neg_solid: false,
            z_pos_solid: false,
            z_neg_solid: false,
        };

        let positions = geo.mesh.attribute(Mesh::ATTRIBUTE_POSITION);
        let Some(VertexAttributeValues::Float32x3(positions)) = positions else {
            panic!("Unexpected value for ATTRIBUTE_POSITION: {positions:?}");
        };

        let mut vertex_candidate_faces = Vec::with_capacity(positions.len());
        for vertex in positions {
            let vx = vertex[0];
            let vy = vertex[1];
            let vz = vertex[2];

            let mut mask = 0u8;
            if (vx - 1.).abs() <= f32::EPSILON {
                mask |= 0b1;
            } else if vx.abs() <= f32::EPSILON {
                mask |= 0b10;
            }
            if (vy - 1.).abs() <= f32::EPSILON {
                mask |= 0b100;
            } else if vy.abs() <= f32::EPSILON {
                mask |= 0b1000;
            }
            if (vz - 1.).abs() <= f32::EPSILON {
                mask |= 0b10000;
            } else if vz.abs() <= f32::EPSILON {
                mask |= 0b100000;
            }
            vertex_candidate_faces.push(mask);
        }

        let faces: Box<dyn Iterator<Item = [usize; 3]>> = match geo.mesh.indices() {
            Some(Indices::U16(v)) => Box::from(
                v.chunks_exact(3)
                    .map(|v| [v[0] as usize, v[1] as usize, v[2] as usize]),
            ),
            Some(Indices::U32(v)) => Box::from(
                v.chunks_exact(3)
                    .map(|v| [v[0] as usize, v[1] as usize, v[2] as usize]),
            ),
            None => {
                // If the geometry has zero faces, we can skip this block.
                return geo;
            }
        };

        // Store if a face is 'half-solid': whe have one out of the needed two triangles.
        let mut x_pos_surface = Vec::new();
        let mut x_neg_surface = Vec::new();
        let mut y_pos_surface = Vec::new();
        let mut y_neg_surface = Vec::new();
        let mut z_pos_surface = Vec::new();
        let mut z_neg_surface = Vec::new();
        for face in faces {
            let mut mask = !0u8;
            for index in face {
                mask &= vertex_candidate_faces[index];
            }

            if (mask | 0b1) != 0 {
                x_pos_surface.push(face)
            }
            if (mask | 0b01) != 0 {
                x_neg_surface.push(face);
            }
            if (mask | 0b001) != 0 {
                y_pos_surface.push(face);
            }
            if (mask | 0b0001) != 0 {
                y_neg_surface.push(face);
            }
            if (mask | 0b00001) != 0 {
                z_pos_surface.push(face);
            }
            if (mask | 0b000001) != 0 {
                z_neg_surface.push(face);
            }
        }

        fn check_solid(triangles: &[[usize; 3]], map: impl Fn(usize) -> [f32; 2]) -> bool {
            let face = MultiPolygon::new(vec![polygon!(
                (x: 0., y: 0.),
                (x: 0., y: 1.),
                (x: 1., y: 1.),
                (x: 1., y: 0.),
            )]);

            let mut polygon = Vec::new();

            for triangle in triangles {
                let pos0 = map(triangle[0]);
                let pos1 = map(triangle[1]);
                let pos2 = map(triangle[2]);

                polygon.push(
                    Triangle::new(
                        Coord {
                            x: pos0[0],
                            y: pos0[1],
                        },
                        Coord {
                            x: pos1[0],
                            y: pos1[1],
                        },
                        Coord {
                            x: pos2[0],
                            y: pos2[1],
                        },
                    )
                    .to_polygon(),
                )
            }

            let multi_polygon = MultiPolygon::new(polygon);
            return multi_polygon
                .difference(&face)
                .chamberlain_duquette_unsigned_area()
                <= f32::EPSILON;
        }

        let map_x = |vertex: usize| {
            let pos = positions[vertex];
            [pos[1], pos[2]]
        };
        let map_y = |vertex: usize| {
            let pos = positions[vertex];
            [pos[0], pos[2]]
        };
        let map_z = |vertex: usize| {
            let pos = positions[vertex];
            [pos[0], pos[1]]
        };

        geo.x_pos_solid = check_solid(&x_pos_surface, map_x);
        geo.x_neg_solid = check_solid(&x_neg_surface, map_x);
        geo.y_pos_solid = check_solid(&y_pos_surface, map_y);
        geo.y_neg_solid = check_solid(&y_neg_surface, map_y);
        geo.z_pos_solid = check_solid(&z_pos_surface, map_z);
        geo.z_neg_solid = check_solid(&z_neg_surface, map_z);
        geo
    }

    /// Returns the bevy mesh that the block will show up with.
    #[allow(unused)]
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}
