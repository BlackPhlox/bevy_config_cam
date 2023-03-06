use crate::Commands;
use bevy::ecs::entity::Entity;
use bevy::prelude::Component;
use bevy::window::PrimaryWindow;
use config_cam_derive::DriverMarker;
use std::any::TypeId;

use crate::driver::driver_core::{DriverMarker, DriverRigs};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct CCFpv;
impl Plugin for CCFpv {
    fn build(&self, app: &mut App) {
        app.add_rig_component(CCFpv)
            .add_startup_system(setup_fpv)
            .add_state::<MovementType>()
            .add_system(update_fpv_camera);
    }
}

fn setup_fpv(mut commands: Commands) {
    commands.spawn((
        Rig::builder()
            .with(Fpv::from_position_target(
                Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ))
            .build(),
        CCFpv,
    ));
}

#[derive(States, Default, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MovementType {
    #[default]
    FirstPerson,
    Free,
}

/*
Example :
commands.spawn((
    MainCamera,
    Rig::builder()
        .with(Fpv::from_position_target(transform))
        .build(),
    Camera3dBundle {
        transform,
        ..default()
    },
));
*/

pub fn update_fpv_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    fps_state: Res<State<MovementType>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rigs: DriverRigs,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let boost_mult = 5.0f32;
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    // Q: Is dolly left-handed so z is flipped?
    if keys.pressed(KeyCode::W) {
        move_vec.z -= 1.0;
    }
    if keys.pressed(KeyCode::S) {
        move_vec.z += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        move_vec.x += 1.0;
    }

    if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        move_vec.y -= 1.0;
    }

    let boost: f32 = if keys.pressed(KeyCode::LShift) {
        1.
    } else {
        0.
    };

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    rigs.try_for_each_driver_mut::<Fpv>(|r| {
        let move_vec = r.set_position(
            move_vec,
            boost,
            boost_mult,
            fps_state.0.eq(&MovementType::FirstPerson),
        );

        if let Ok(window) = windows.get_single() {
            if !window.cursor.visible {
                r.driver_mut::<Fpv>()
                    .set_rotation(delta, sensitivity, move_vec, time_delta_seconds);
            }
        }
    });
}
