use bevy::math::Vec2;
use polyanya::Trimesh;

use crate::data::NavmeshDump;

pub fn build_mesh(dump: &NavmeshDump) -> polyanya::Mesh {
    let verts: Vec<Vec2> = dump
        .stairs
        .vertices
        .iter()
        .map(|v| Vec2::new(v[0], v[1]))
        .collect();

    if dump.stairs.triangles.is_empty() {
        eprintln!("Navmesh repro: no triangles");
        let mut mesh = polyanya::Mesh::default();
        mesh.search_delta = 0.5;
        mesh.search_steps = 10;
        return mesh;
    }

    eprintln!(
        "Navmesh repro: {} verts, {} triangles",
        verts.len(),
        dump.stairs.triangles.len(),
    );

    let trimesh = Trimesh {
        vertices: verts,
        triangles: dump.stairs.triangles.clone(),
    };
    let mut mesh: polyanya::Mesh = trimesh
        .try_into()
        .expect("trimesh conversion failed");
    mesh.layers[0].height = dump.stairs.heights.clone();
    mesh.bake();

    mesh.search_delta = 0.5;
    mesh.search_steps = 10;

    mesh
}
