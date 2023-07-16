use three_d::*;

#[allow(dead_code)]
pub fn cube(ht: f32, wt: f32) -> CpuMesh {
    let height = ht;
    let width = wt;
    let length = wt;

    let positions = vec![
        // Up
        vec3(width / 2.0, height / 2.0, -length / 2.0),
        vec3(-width / 2.0, height / 2.0, -length / 2.0),
        vec3(width / 2.0, height / 2.0, length / 2.0),
        vec3(-width / 2.0, height / 2.0, length / 2.0),
        vec3(width / 2.0, height / 2.0, length / 2.0),
        vec3(-width / 2.0, height / 2.0, -length / 2.0),
        // Down
        vec3(-width / 2.0, -height / 2.0, -length / 2.0),
        vec3(width / 2.0, -height / 2.0, -length / 2.0),
        vec3(width / 2.0, -height / 2.0, length / 2.0),
        vec3(width / 2.0, -height / 2.0, length / 2.0),
        vec3(-width / 2.0, -height / 2.0, length / 2.0),
        vec3(-width / 2.0, -height / 2.0, -length / 2.0),
        // Back
        vec3(width / 2.0, -height / 2.0, -length / 2.0),
        vec3(-width / 2.0, -height / 2.0, -length / 2.0),
        vec3(width / 2.0, height / 2.0, -length / 2.0),
        vec3(-width / 2.0, height / 2.0, -length / 2.0),
        vec3(width / 2.0, height / 2.0, -length / 2.0),
        vec3(-width / 2.0, -height / 2.0, -length / 2.0),
        // Front
        vec3(-width / 2.0, -height / 2.0, length / 2.0),
        vec3(width / 2.0, -height / 2.0, length / 2.0),
        vec3(width / 2.0, height / 2.0, length / 2.0),
        vec3(width / 2.0, height / 2.0, length / 2.0),
        vec3(-width / 2.0, height / 2.0, length / 2.0),
        vec3(-width / 2.0, -height / 2.0, length / 2.0),
        // Right
        vec3(width / 2.0, -height / 2.0, -length / 2.0),
        vec3(width / 2.0, height / 2.0, -length / 2.0),
        vec3(width / 2.0, height / 2.0, length / 2.0),
        vec3(width / 2.0, height / 2.0, length / 2.0),
        vec3(width / 2.0, -height / 2.0, length / 2.0),
        vec3(width / 2.0, -height / 2.0, -length / 2.0),
        // Left
        vec3(-width / 2.0, height / 2.0, -length / 2.0),
        vec3(-width / 2.0, -height / 2.0, -length / 2.0),
        vec3(-width / 2.0, height / 2.0, length / 2.0),
        vec3(-width / 2.0, -height / 2.0, length / 2.0),
        vec3(-width / 2.0, height / 2.0, length / 2.0),
        vec3(-width / 2.0, -height / 2.0, -length / 2.0),
    ];

    let uvs = vec![
        // Up
        Vec2::new(0.25, 0.0),
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.25, 1.0 / 3.0),
        // Down
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.25, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        // Back
        Vec2::new(0.0, 2.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.0, 1.0 / 3.0),
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.0, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        // Front
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.75, 2.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 2.0 / 3.0),
        // Right
        Vec2::new(1.0, 2.0 / 3.0),
        Vec2::new(1.0, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 2.0 / 3.0),
        Vec2::new(1.0, 2.0 / 3.0),
        // Left
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
    ];
    let mut mesh = three_d_asset::geometry::TriMesh {
        positions: Positions::F32(positions),
        uvs: Some(uvs),
        ..Default::default()
    };
    mesh.compute_normals();
    mesh.compute_tangents();
    return mesh;
}
