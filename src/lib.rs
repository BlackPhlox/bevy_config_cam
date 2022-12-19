pub mod driver;
pub mod drivers;

use crate::drivers::first_person_view::Fpv;
use crate::drivers::pinned::Pinned;
use bevy::{
    input::Input,
    prelude::{App, Commands, Component, KeyCode, Plugin, Query, Res, Transform, Vec3, With},
};
use bevy_dolly::{dolly::glam, prelude::*};
use driver::{
    driver_core::{DriverIndex, DriverRigs, Drivers},
    driver_resources::{change_driver_system, update_driver_system},
};
pub use std::any::TypeId;

// TODO documentation

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut App) {
        app.init_resource::<DriverIndex>()
            .init_resource::<Drivers>()
            .add_rig_component(Pinned)
            .add_rig_component(FPV)
            .add_startup_system(default_setup)
            .add_system(change_driver_system)
            .add_system(update_driver_system)
            .add_system(update_look_at)
            .add_system(update_yaw_driver);
    }
}

pub(crate) fn update_look_at(
    mut targets: Query<(&mut Transform, With<Target>)>,
    mut rigs: DriverRigs,
) {
    let mut avg = Vec3::ZERO;

    for (t, _b) in &mut targets {
        avg += t.translation;
    }

    //https://math.stackexchange.com/questions/80923/average-of-multiple-vectors
    //let total_targets = targets.iter().count();
    //avg /= total_targets as f32;

    rigs.try_for_each_driver_mut::<bevy_dolly::prelude::LookAt>(|la| {
        la.target = avg;
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

// Target at is just a valid option for Follow, Orbit and FPV
// Have the camera point at one or the summed vector direction
// of all entities with the Target Component
#[derive(Component)]
pub struct Target;

fn default_setup(mut commands: Commands) {
    //Should be default player entity
    //Default player entity : Cone
    //commands.spawn().insert(Target);

    commands.spawn((
        Pinned,
        Rig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-45.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(glam::Vec3::Z * 4.0))
            .with(bevy_dolly::prelude::LookAt::new(glam::Vec3::new(
                0., 0., 0.,
            )))
            .build(),
    ));

    //Missing FPV
    commands.spawn(Fpv);
}
