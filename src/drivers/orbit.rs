use crate::{CCSetupLabel, Commands, MainCamera};
use bevy::ecs::entity::Entity;
use bevy::prelude::Component;
use config_cam_derive::DriverMarker;
use std::any::TypeId;

use crate::driver::driver_core::DriverMarker;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_dolly::prelude::*;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct CCOrbit;
impl Plugin for CCOrbit {
    fn build(&self, app: &mut App) {
        app.add_rig_component(CCOrbit)
            .add_state::<Pan>()
            .add_startup_system(setup_orbit.after(CCSetupLabel))
            .add_system(handle_mouse_scroll)
            .add_system(update_orbit_camera);
    }
}

fn setup_orbit(mut commands: Commands) {
    commands.spawn((
        Rig::builder()
            .with(Position::new(Vec3::ZERO))
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_position(0.3))
            .with(Smooth::new_rotation(0.3))
            .with(Arm::new(Vec3::Z * 4.0))
            .build(),
        CCOrbit,
    ));
}

#[derive(States, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Pan {
    #[default]
    Mouse,
    Keys,
}

fn handle_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut q_main: Query<&mut Projection, With<MainCamera>>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for mut projection in &mut q_main.iter_mut() {
            match &mut projection.as_mut() {
                Projection::Perspective(pers) => {
                    pers.fov = (pers.fov - mouse_wheel_event.y * 0.01).abs();
                }
                Projection::Orthographic(orth) => {
                    orth.scale = (orth.scale - mouse_wheel_event.y * 0.1).abs();
                }
            }
        }
    }
}

#[allow(unused_must_use)]
fn update_orbit_camera(
    keys: Res<Input<KeyCode>>,
    mut pan: ResMut<State<Pan>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rig_q: Query<&mut Rig, With<CCOrbit>>,
    transform: Query<Option<&Transform>, With<DollyPosCtrlMove>>,
    mut config: ResMut<DollyPosCtrlConfig>,
    grab_config: Res<DollyCursorGrabConfig>,
) {
    if let Ok(mut rig) = rig_q.get_single_mut() {
        let camera_driver = rig.driver_mut::<YawPitch>();
        let sensitivity = Vec2::splat(2.0);

        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }

        config.transform.rotation = Quat::from_rotation_y(delta.x);

        if pan.get().eq(&Pan::Keys) {
            if keys.just_pressed(KeyCode::Z) {
                camera_driver.rotate_yaw_pitch(-90.0, 0.0);
            }
            if keys.just_pressed(KeyCode::X) {
                camera_driver.rotate_yaw_pitch(90.0, 0.0);
            }
        } else if !grab_config.visible {
            camera_driver.rotate_yaw_pitch(
                -0.1 * delta.x * sensitivity.x,
                -0.1 * delta.y * sensitivity.y,
            );
        }

        if keys.just_pressed(KeyCode::E) {
            let result = if pan.get().eq(&Pan::Keys) {
                Pan::Mouse
            } else {
                Pan::Keys
            };
            *pan = State::new(result);
            println!("State:{result:?}");
        }

        if keys.just_pressed(KeyCode::P) {
            config.pin = !config.pin;
            println!("Pinned:{:?}", config.pin);
        }

        if config.pin {
            let camera_driver_2 = rig.driver_mut::<Position>();

            for opt_transform in transform.iter().flatten() {
                camera_driver_2.position = opt_transform.translation;
            }
        }
    }
}
