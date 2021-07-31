use bevy::{
    core::Time,
    input::Input,
    math::{Quat, Vec3},
    prelude::{Entity, KeyCode, Query, Res, Transform},
};

use crate::{validate_key, PlayerMove};

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
}

pub struct PlayerSettings {
    pub speed: f32,
    pub map: PlayerKeyMap,
    pub pos: Vec3,
    pub cam_fwd: bool,
    pub disable_move: bool,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            speed: 4.0,
            map: PlayerKeyMap::default(),
            pos: Default::default(),
            cam_fwd: false,
            disable_move: false,
        }
    }
}

pub struct PlayerKeyMap {
    pub forward: &'static [KeyCode],
    pub backward: &'static [KeyCode],
    pub left: &'static [KeyCode],
    pub right: &'static [KeyCode],
    pub up: &'static [KeyCode],
    pub down: &'static [KeyCode],
    pub rot_left: &'static [KeyCode],
    pub rot_right: &'static [KeyCode],
}

impl Default for PlayerKeyMap {
    fn default() -> Self {
        Self {
            forward: &[KeyCode::Up],
            backward: &[KeyCode::Down],
            left: &[KeyCode::Comma],
            right: &[KeyCode::Period],
            up: &[KeyCode::RShift],
            down: &[KeyCode::Minus],
            rot_left: &[KeyCode::Left],
            rot_right: &[KeyCode::Right],
        }
    }
}

// control the cam logic character
pub fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    settings: Res<PlayerSettings>,
    mut transforms: Query<(&PlayerMove, &mut Transform)>,
) {
    if settings.disable_move {
        return;
    }
    for (_player, mut transform) in transforms.iter_mut() {
        let (_, mut rotation) = transform.rotation.to_axis_angle();
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        //Forward should be togglable either xyz or cam direction xyz
        let forward = if settings.cam_fwd {
            Vec3::new(local_z.x, 0., local_z.z)
        } else {
            -Vec3::new(local_z.x, 0., local_z.z)
        };

        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
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
            if validate_key(settings.map.rot_left, key) {
                //Wrapping around
                if rotation > std::f32::consts::FRAC_PI_2 * 4.0 - 0.05 {
                    rotation = 0.0;
                }
                rotation += 0.1
            }
            if validate_key(settings.map.rot_right, key) {
                //Wrapping around
                if rotation < 0.05 {
                    rotation = std::f32::consts::FRAC_PI_2 * 4.0;
                }
                rotation -= 0.1
            }
        }

        velocity = velocity.normalize();

        transform.rotation = Quat::from_rotation_y(rotation);

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * settings.speed;
        }
    }
}
