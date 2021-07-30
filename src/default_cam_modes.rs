use bevy::{
    math::{Quat, Vec3},
    prelude::{Query, QuerySet, ResMut, Transform},
    render::camera::Camera,
};

use crate::{cam::MovementSettings, CameraMode, Config};

const RESET_FOCUS: [f32; 3] = [0., 0., 0.];

/// Use the mouse to look and move the camera freely
pub struct Free;
impl CameraMode for Free {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        _config: ResMut<Config>,
        mut move_config: ResMut<MovementSettings>,
        _transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        move_config.disable_look = false;
        (_config.to_owned(), Transform::identity())
    }

    fn name(&self) -> &str {
        "Free"
    }
}

/// Look at player and other targets if set
pub struct LookAt;
impl CameraMode for LookAt {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        mut config: ResMut<Config>,
        move_config: ResMut<MovementSettings>,
        transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        // if there is both a player and a bonus, target the mid-point of them
        if let (Some(player_entity), Some(bonus_entity)) = (config.target, config.external_target) {
            if let (Ok(player_transform), Ok(bonus_transform)) = (
                transforms.q1().get(player_entity),
                transforms.q1().get(bonus_entity),
            ) {
                config.camera_settings.camera_should_focus = player_transform
                    .translation
                    .lerp(bonus_transform.translation, move_config.lerp);
            }
        // otherwise, if there is only a player, target the player
        } else if let Some(player_entity) = config.target {
            if let Ok(player_transform) = transforms.q1().get(player_entity) {
                config.camera_settings.camera_should_focus = player_transform.translation;
            }
        // otherwise, target the middle
        } else {
            config.camera_settings.camera_should_focus = Vec3::from(RESET_FOCUS);
        }
        (config.to_owned(), Transform::identity())
    }

    fn name(&self) -> &str {
        "LookAt"
    }
}

/// Camera at same position as player, enables to use the mouse to look (WIP)
pub struct Fps;
impl CameraMode for Fps {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        config: ResMut<Config>,
        mut move_config: ResMut<MovementSettings>,
        transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        let mut delta_trans = Transform::identity();
        if let Some(player_entity) = config.target {
            if let Ok(player_transform) = transforms.q1().get(player_entity) {
                delta_trans.translation = player_transform.translation;
                delta_trans.rotation = player_transform.rotation;
            }
        }
        move_config.disable_move = true;
        move_config.disable_look = false;
        delta_trans.translation += Vec3::new(/*-4.*/ 0., 1., 0.);
        (config.to_owned(), delta_trans)
    }

    fn name(&self) -> &str {
        "Fps"
    }
}

/// Camera is moved above and pointed down, rotation bound to one axis
pub struct TopDown;
impl CameraMode for TopDown {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        config: ResMut<Config>,
        mut move_config: ResMut<MovementSettings>,
        transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        let mut delta_trans = Transform::identity();
        if let Some(player_entity) = config.target {
            if let Ok(player_transform) = transforms.q1().get(player_entity) {
                delta_trans.translation = player_transform.translation;
                delta_trans.translation += Vec3::new(/*-4.*/ 0., move_config.dist, 0.);
                delta_trans.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
            }
        }
        move_config.disable_move = true;
        move_config.disable_look = false;
        (config.to_owned(), delta_trans)
    }

    fn name(&self) -> &str {
        "TopDown"
    }
}

/// Same as TopDown but follows the players direction
pub struct TopDownDirection;
impl CameraMode for TopDownDirection {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        _config: ResMut<Config>,
        mut move_config: ResMut<MovementSettings>,
        _transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        let mut delta_trans = Transform::identity();
        delta_trans.translation += Vec3::new(/*-4.*/ 0., move_config.dist, 0.);
        delta_trans.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
        move_config.locked_to_player = true;
        move_config.disable_move = true;
        move_config.disable_look = false;
        (_config.to_owned(), delta_trans)
    }

    fn name(&self) -> &str {
        "TopDownDirection"
    }
}

//Follows behind the player a certain distance
pub struct FollowBehind;
impl CameraMode for FollowBehind {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        _config: ResMut<Config>,
        mut move_config: ResMut<MovementSettings>,
        _transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        let mut delta_trans = Transform::identity();
        move_config.disable_move = true;
        move_config.locked_to_player = true;
        delta_trans.translation += Vec3::new(/*-4.*/ 0., 1., 4.);
        (_config.to_owned(), delta_trans)
    }

    fn name(&self) -> &str {
        "FollowBehind"
    }
}

//Follow the player only
pub struct FollowStatic;
impl CameraMode for FollowStatic {
    #[allow(clippy::type_complexity)]
    fn update(
        &self,
        mut config: ResMut<Config>,
        _move_config: ResMut<MovementSettings>,
        transforms: &QuerySet<(Query<(&mut Transform, &Camera)>, Query<&Transform>)>,
    ) -> (Config, Transform) {
        if let Some(player_entity) = config.target {
            if let Ok(player_transform) = transforms.q1().get(player_entity) {
                config.camera_settings.camera_should_focus = player_transform.translation;
            }
        }
        (config.to_owned(), Transform::identity())
    }

    fn name(&self) -> &str {
        "FollowStatic"
    }
}
