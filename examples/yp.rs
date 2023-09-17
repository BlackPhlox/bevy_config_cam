use bevy::{input::Input, prelude::*, DefaultPlugins};
use bevy_config_cam::driver::driver_core::DriverMarker;
use bevy_config_cam::{
    driver::driver_core::{DriverRigs, Drivers},
    drivers::{fpv::CCFpv, orbit::CCOrbit},
    ConfigCam,
};
use bevy_config_cam::{driver_vec, type_vec, MainCamera};
use bevy_dolly::prelude::*;
use config_cam_derive::DriverMarker;

pub use std::any::TypeId;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .add_dolly_component(YP)
        .insert_resource(Drivers::new(
            driver_vec![CCOrbit, CCFpv, YP],
            type_vec![MainCamera],
        ))
        .add_startup_system(setup)
        .add_system(update_yaw_driver)
        .run();
}

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct YP;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Should be default player entity
    //Default player entity : Cone
    //commands.spawn().insert(Target);

    commands.spawn((
        Rig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-45.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(Vec3::Z * 4.0))
            .with(LookAt::new(Vec3::new(0., 0., 0.)))
            .build(),
        YP,
    ));

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            ..Default::default()
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

pub(crate) fn update_yaw_driver(keys: Res<Input<KeyCode>>, mut rigs: DriverRigs) {
    rigs.try_for_each_driver_mut::<YawPitch>(|yp| {
        if keys.just_pressed(KeyCode::Z) {
            yp.rotate_yaw_pitch(-90.0, 0.0);
        }
        if keys.just_pressed(KeyCode::X) {
            yp.rotate_yaw_pitch(90.0, 0.0);
        }
    });
}
