use bevy::prelude::KeyCode;

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