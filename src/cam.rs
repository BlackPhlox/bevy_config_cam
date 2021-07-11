use bevy::{
    core::Time,
    input::Input,
    math::Vec3,
    prelude::{KeyCode, Query, Res, Transform},
    window::Windows,
};

use crate::{validate_key, StaticCam};

/// Mouse sensitivity and movement speed
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub dist: f32,
    pub map: CamKeyMap,
    //pub force_cam: &'static[CameraState],
    pub disable_move: bool,
    pub disable_look: bool,
    pub locked_to_player: bool,
    pub lerp: f32,

    pub ltp: bool,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
            dist: 10.,
            map: CamKeyMap::default(),
            disable_move: false,
            disable_look: false,
            locked_to_player: false,
            lerp: 0.5,
            ltp: false,
        }
    }
}

pub struct CamKeyMap {
    pub forward: &'static [KeyCode],
    pub backward: &'static [KeyCode],
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub up: &'static [KeyCode],
    pub down: &'static [KeyCode],
    pub next_cam: &'static [KeyCode],
    pub next_setting: &'static [KeyCode],
}

impl Default for CamKeyMap {
    fn default() -> Self {
        Self {
            forward: &[KeyCode::W],
            backward: &[KeyCode::S],
            left: &[KeyCode::A],
            right: &[KeyCode::D],
            up: &[KeyCode::Space],
            down: &[KeyCode::LShift],
            next_cam: &[KeyCode::C],
            next_setting: &[KeyCode::E],
        }
    }
}

/// Handles keyboard input and movement
pub fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut query: Query<(&StaticCam, &mut Transform)>,
) {
    if settings.disable_move {
        return;
    }
    let window = windows.get_primary().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            if window.cursor_locked() {
                if validate_key(settings.map.forward, key) {
                    velocity += forward
                }
                if validate_key(settings.map.backward, key) {
                    velocity -= forward
                }
                if validate_key(settings.map.left, key) {
                    velocity -= right
                }
                if validate_key(settings.map.right, key) {
                    velocity += right
                }
                if validate_key(settings.map.up, key) {
                    velocity += Vec3::Y
                }
                if validate_key(settings.map.down, key) {
                    velocity -= Vec3::Y
                }
            }
        }

        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * settings.speed
        }
    }
}
