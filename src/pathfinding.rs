use bevy::prelude::*;

use crate::NavmeshResource;

#[derive(Default, PartialEq, Eq)]
enum ClickPhase {
    #[default]
    PlacingStart,
    PlacingEnd,
    Dragging,
    Done,
}

#[derive(Resource, Default)]
pub struct PathState {
    pub start: Option<Vec3>,
    pub end: Option<Vec3>,
    pub path: Option<Vec<Vec3>>,
    phase: ClickPhase,
}

/// Click places start or end. Click-and-hold on end allows dragging.
pub fn on_mesh_press(
    press: On<Pointer<Press>>,
    mut path_state: ResMut<PathState>,
) {
    if press.button != PointerButton::Primary {
        return;
    }
    let Some(world_pos) = press.hit.position else {
        return;
    };

    match path_state.phase {
        ClickPhase::PlacingStart => {
            path_state.start = Some(world_pos);
            path_state.end = None;
            path_state.path = None;
            path_state.phase = ClickPhase::PlacingEnd;
        }
        ClickPhase::Done => {
            // Reset and place new start.
            path_state.start = Some(world_pos);
            path_state.end = None;
            path_state.path = None;
            path_state.phase = ClickPhase::PlacingEnd;
        }
        ClickPhase::PlacingEnd => {
            // Start dragging the end point.
            path_state.phase = ClickPhase::Dragging;
        }
        ClickPhase::Dragging => {}
    }
}

pub fn on_mesh_release(
    release: On<Pointer<Release>>,
    mut path_state: ResMut<PathState>,
) {
    if release.button != PointerButton::Primary {
        return;
    }
    if path_state.phase == ClickPhase::Dragging {
        path_state.phase = ClickPhase::Done;
    }
}

/// As the pointer moves, update end point and pathfind.
pub fn on_mesh_move(
    hover: On<Pointer<Move>>,
    mut path_state: ResMut<PathState>,
    navmesh: Option<Res<NavmeshResource>>,
) {
    if path_state.phase != ClickPhase::PlacingEnd && path_state.phase != ClickPhase::Dragging {
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
    let start_color = Color::srgb(0.2, 1.0, 0.2);
    let end_color = Color::srgb(1.0, 0.3, 0.3);
    let path_color = Color::srgb(1.0, 0.0, 1.0);
    let lift = Vec3::Y * 0.5;

    if let Some(start) = path_state.start {
        gizmos.sphere(Isometry3d::from_translation(start + lift), 0.5, start_color);
    }
    if let Some(end) = path_state.end {
        gizmos.sphere(Isometry3d::from_translation(end + lift), 0.5, end_color);
    }

    if let Some(ref path) = path_state.path {
        for window in path.windows(2) {
            gizmos.line(window[0] + lift, window[1] + lift, path_color);
        }
    }
}
