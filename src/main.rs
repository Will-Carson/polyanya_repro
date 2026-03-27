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

    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
