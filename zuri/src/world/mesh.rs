use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// Generates the mesh for a solid block.
pub fn solid_block() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut uv = Vec::<[f32; 2]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();
    let mut triangles = Vec::<u32>::new();

    let mut start_index = 0;

    let x = 0;
    let y = 0;
    let z = 0;

    vertices.push([x as f32, y as f32, z as f32]);
    vertices.push([(x + 1) as f32, y as f32, z as f32]);
    vertices.push([x as f32, y as f32, (z + 1) as f32]);
    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);

    uv.push([0., 0.]);
    uv.push([1., 0.]);
    uv.push([0., 1.]);
    uv.push([1., 1.]);

    normals.push([0., -1., 0.]);
    normals.push([0., -1., 0.]);
    normals.push([0., -1., 0.]);
    normals.push([0., -1., 0.]);

    // Down 1
    triangles.push(start_index + 0);
    triangles.push(start_index + 1);
    triangles.push(start_index + 2);
    // Down 2
    triangles.push(start_index + 2);
    triangles.push(start_index + 1);
    triangles.push(start_index + 3);

    start_index += 4;

    vertices.push([x as f32, (y + 1) as f32, z as f32]);
    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);
    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);
    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);

    uv.push([0., 0.]);
    uv.push([1., 0.]);
    uv.push([0., 1.]);
    uv.push([1., 1.]);

    normals.push([0., 1.0, 0.]);
    normals.push([0., 1.0, 0.]);
    normals.push([0., 1.0, 0.]);
    normals.push([0., 1.0, 0.]);

    // Up 1
    triangles.push(start_index + 2);
    triangles.push(start_index + 1);
    triangles.push(start_index + 0);
    // Up 2
    triangles.push(start_index + 3);
    triangles.push(start_index + 1);
    triangles.push(start_index + 2);

    start_index += 4;

    vertices.push([x as f32, y as f32, z as f32]);
    vertices.push([(x + 1) as f32, y as f32, z as f32]);
    vertices.push([x as f32, (y + 1) as f32, z as f32]);
    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);

    uv.push([0., 0.]);
    uv.push([1., 0.]);
    uv.push([0., 1.]);
    uv.push([1., 1.]);

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

    vertices.push([x as f32, y as f32, (z + 1) as f32]);
    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);
    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);
    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);

    uv.push([0., 0.]);
    uv.push([1., 0.]);
    uv.push([0., 1.]);
    uv.push([1., 1.]);

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

    vertices.push([(x + 1) as f32, y as f32, z as f32]);
    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);
    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);
    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);

    uv.push([0., 0.]);
    uv.push([1., 0.]);
    uv.push([0., 1.]);
    uv.push([1., 1.]);

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

    vertices.push([x as f32, y as f32, z as f32]);
    vertices.push([x as f32, y as f32, (z + 1) as f32]);
    vertices.push([x as f32, (y + 1) as f32, z as f32]);
    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);

    uv.push([1., 1.]);
    uv.push([1., 0.]);
    uv.push([0., 1.]);
    uv.push([0., 0.]);

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

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(triangles)));

    mesh
}
