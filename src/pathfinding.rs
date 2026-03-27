use bevy::prelude::*;

use crate::NavmeshResource;

#[derive(Resource, Default)]
pub struct PathState {
    pub start: Option<Vec3>,
    pub end: Option<Vec3>,
    pub path: Option<Vec<Vec3>>,
    /// Once start is placed, true until next click clears it.
    pub pinned: bool,
}

pub fn on_mesh_click(
    click: On<Pointer<Click>>,
    mut path_state: ResMut<PathState>,
) {
    if click.button != PointerButton::Primary {
        return;
    }
    let Some(world_pos) = click.hit.position else {
        return;
    };

    if !path_state.pinned {
        // Place start pin.
        path_state.start = Some(world_pos);
        path_state.end = None;
        path_state.path = None;
        path_state.pinned = true;
    } else {
        // Lock end and reset — next click places a new start.
        path_state.pinned = false;
    }
}

pub fn on_mesh_move(
    hover: On<Pointer<Move>>,
    mut path_state: ResMut<PathState>,
    navmesh: Option<Res<NavmeshResource>>,
) {
    if !path_state.pinned {
        return;
    }
    let Some(world_pos) = hover.hit.position else {
        return;
    };
    let Some(navmesh) = navmesh else {
        return;
    };
    let Some(start) = path_state.start else {
        return;
    };

    path_state.end = Some(world_pos);

    let from = Vec2::new(start.x, start.z);
    let to = Vec2::new(world_pos.x, world_pos.z);

    match navmesh.0.path(from, to) {
        Some(path) => {
            let path_3d = path.path_with_height(
                Vec3::new(start.x, start.y, start.z),
                Vec3::new(world_pos.x, world_pos.y, world_pos.z),
                &navmesh.0,
            );
            path_state.path = Some(path_3d);
        }
        None => {
            path_state.path = None;
        }
    }
}

pub fn draw_path_gizmos(mut gizmos: Gizmos, path_state: Res<PathState>) {
    let marker_color = Color::srgb(1.0, 1.0, 0.0);
    let path_color = Color::srgb(1.0, 0.0, 1.0);
    let lift = Vec3::Y * 0.5;

    if let Some(start) = path_state.start {
        gizmos.sphere(Isometry3d::from_translation(start + lift), 0.5, marker_color);
    }
    if let Some(end) = path_state.end {
        gizmos.sphere(Isometry3d::from_translation(end + lift), 0.5, marker_color);
    }

    if let Some(ref path) = path_state.path {
        for window in path.windows(2) {
            gizmos.line(window[0] + lift, window[1] + lift, path_color);
        }
    }
}
