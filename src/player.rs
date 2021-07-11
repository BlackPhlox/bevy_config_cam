use bevy::{math::Vec3, prelude::{Entity, KeyCode}};

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
}

pub struct PlayerSettings {
    pub player_asset: &'static str,
    pub speed: f32,
    pub map: PlayerKeyMap,
    pub pos: Vec3,
    pub cam_fwd: bool,
    pub disable_default: bool,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            player_asset: "",
            speed: 12.0,
            map: PlayerKeyMap::default(),
            pos: Default::default(),
            cam_fwd: false,
            disable_default: false,
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
