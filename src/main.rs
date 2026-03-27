mod camera;
mod data;
mod navmesh;
mod pathfinding;
mod visualization;

use bevy::prelude::*;

#[derive(Resource)]
pub struct NavmeshResource(pub polyanya::Mesh);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .init_resource::<pathfinding::PathState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (camera::orbit_camera_system, pathfinding::draw_path_gizmos),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dump = data::load_dump("assets/navmesh_dump.json");
    let nav = navmesh::build_mesh(&dump);

    visualization::spawn_navmesh(&mut commands, &mut meshes, &mut materials, &nav);

    commands.insert_resource(NavmeshResource(nav));

    camera::spawn_orbit_camera(&mut commands);

    // Key light — warm, from above-right.
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.95, 0.9),
            ..default()
        },
        Transform::from_xyz(60.0, 120.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Fill light — cooler, from the opposite side, dimmer.
    commands.spawn((
        DirectionalLight {
            illuminance: 1500.0,
            shadows_enabled: false,
            color: Color::srgb(0.85, 0.9, 1.0),
            ..default()
        },
        Transform::from_xyz(-40.0, 80.0, -60.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
