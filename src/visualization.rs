use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

const MESH_COLOR: [f32; 3] = [0.3, 0.6, 0.4];

pub fn spawn_navmesh(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    navmesh: &polyanya::Mesh,
) {
    let layer = &navmesh.layers[0];
    if layer.polygons.is_empty() {
        eprintln!("Viz: navmesh is empty");
        return;
    }

    let has_heights = layer.height.len() == layer.vertices.len();

    let positions: Vec<[f32; 3]> = layer
        .vertices
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let y = if has_heights { layer.height[i] } else { 0.0 };
            [v.coords.x, y, v.coords.y]
        })
        .collect();

    let mut indices: Vec<u32> = Vec::new();
    for polygon in &layer.polygons {
        let verts = &polygon.vertices;
        if verts.len() < 3 {
            continue;
        }
        for i in 1..verts.len() - 1 {
            // Reverse winding: polyanya CCW in 2D (X,Y) becomes CW
            // when mapped to Bevy 3D (X, height, Z).
            indices.push(verts[0]);
            indices.push(verts[i + 1] as u32);
            indices.push(verts[i] as u32);
        }
    }

    if indices.is_empty() {
        return;
    }

    let normals: Vec<[f32; 3]> = vec![[0.0, 1.0, 0.0]; positions.len()];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));

    let material = StandardMaterial {
        base_color: Color::srgb(MESH_COLOR[0], MESH_COLOR[1], MESH_COLOR[2]),
        double_sided: true,
        cull_mode: None,
        ..default()
    };

    commands
        .spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(material)),
        ))
        .observe(crate::pathfinding::on_mesh_click)
        .observe(crate::pathfinding::on_mesh_move);

    eprintln!(
        "Viz: {} verts, {} polys",
        layer.vertices.len(),
        layer.polygons.len(),
    );
}
