use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

const ORBIT_SENSITIVITY: f32 = 0.005;
const ORBIT_LERP_SPEED: f32 = 5.0;

#[derive(Component)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.6,
            distance: 80.0,
        }
    }
}

pub fn spawn_orbit_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        OrbitCamera::default(),
        Transform::from_xyz(0.0, 60.0, 80.0).looking_at(Vec3::ZERO, Vec3::Y),
        AmbientLight {
            color: Color::WHITE,
            brightness: 8000.0,
            ..Default::default()
        },
    ));
}

pub fn orbit_camera_system(
    time: Res<Time>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut q_camera: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let Ok((mut cam_transform, mut orbit)) = q_camera.single_mut() else {
        return;
    };

    // Right-click drag to rotate.
    if mouse_button.pressed(MouseButton::Right) {
        let delta = mouse_motion.delta;
        orbit.yaw -= delta.x * ORBIT_SENSITIVITY;
        orbit.pitch -= delta.y * ORBIT_SENSITIVITY;
        orbit.pitch = orbit.pitch.clamp(-1.4, -0.1);
    }

    // Compute orbit position from spherical coordinates around origin.
    let offset = Vec3::new(
        orbit.distance * orbit.pitch.cos() * orbit.yaw.sin(),
        orbit.distance * -orbit.pitch.sin(),
        orbit.distance * orbit.pitch.cos() * orbit.yaw.cos(),
    );

    let t = (ORBIT_LERP_SPEED * time.delta_secs()).min(1.0);
    cam_transform.translation = cam_transform.translation.lerp(offset, t);
    cam_transform.look_at(Vec3::ZERO, Vec3::Y);
}
