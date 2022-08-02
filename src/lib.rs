use std::any::TypeId;

use bevy::{
    input::Input,
    prelude::{App, Camera, Commands, Component, Entity, KeyCode, Plugin, Query, Res, ResMut},
};
use bevy_dolly::{dolly::glam, prelude::*};
use driver_marker_derive::DriverMarker;

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut App) {
        app.init_resource::<DriverIndex>()
            .init_resource::<Drivers>()
            .add_dolly_component(LookAt)
            .add_dolly_component(Orbit)
            .add_startup_system(default_setup)
            .add_system(change_driver_system)
            .add_system(update_driver_system)
            .add_system(update_yaw_driver);
    }
}

fn update_yaw_driver(keys: Res<Input<KeyCode>>, mut query: Query<&mut Rig>) {
    for mut rig in &mut query {
        if let Some(camera_driver) = rig.try_driver_mut::<YawPitch>() {
            if keys.just_pressed(KeyCode::Z) {
                camera_driver.rotate_yaw_pitch(-90.0, 0.0);
            }
            if keys.just_pressed(KeyCode::X) {
                camera_driver.rotate_yaw_pitch(90.0, 0.0);
            }
        }
    }
}

#[derive(Component, DriverMarker, Clone, Copy, Debug)]
pub struct LookAt;

#[derive(Component, DriverMarker, Clone, Copy, Debug)]
pub struct Orbit;

fn default_setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(
            Rig::builder()
                .with(Position::new(glam::Vec3::new(1., 1., 1.) * 3.0))
                .with(bevy_dolly::prelude::LookAt::new(glam::Vec3::new(
                    0., 0., 0.,
                )))
                .build(),
        )
        .insert(LookAt);

    commands
        .spawn()
        .insert(
            Rig::builder()
                .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-45.0))
                .with(Smooth::new_rotation(1.5))
                .with(Arm::new(glam::Vec3::Z * 4.0))
                .build(),
        )
        .insert(Orbit);
}

pub struct Drivers(Vec<Box<dyn DriverMarker>>);

impl Default for Drivers {
    fn default() -> Self {
        Self(vec![Box::new(LookAt), Box::new(Orbit)])
    }
}

impl Drivers {
    pub fn new(driver_markers: Vec<Box<dyn DriverMarker>>) -> Self {
        Self(driver_markers)
    }
}

pub trait DriverMarker: Sync + Send + 'static {
    fn get_id(&self) -> TypeId;
    fn get_name(&self) -> &str;
    fn add_to(&self, commands: &mut Commands, entity: Entity);
    fn remove_from(&self, commands: &mut Commands, entity: Entity);
}

#[derive(Default)]
pub struct DriverIndex(usize);

impl DriverIndex {
    pub fn next(&mut self, len: usize) {
        if self.0 >= len - 1 {
            self.0 = 0;
        } else {
            self.0 += 1;
        }
    }
}

//Use collection with and keeping an index component or the like
fn change_driver_system(
    mut index: ResMut<DriverIndex>,
    drivers: Res<Drivers>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::T) {
        index.next(drivers.0.len());
    }
}

fn update_driver_system(
    q: Query<(Entity, &Camera)>,
    mut commands: Commands,
    index: Res<DriverIndex>,
    drivers: Res<Drivers>,
) {
    if index.is_changed() {
        for box_component in &drivers.0 {
            let component = box_component.as_ref();
            let component_id = component.get_id();

            if let Some(h) = drivers.0.get(index.0) {
                if component_id.eq(&h.get_id()) {
                    //Add new driver component
                    //Remove old driver component
                    for (entity, camera) in q.iter() {
                        if camera.is_active {
                            component.add_to(&mut commands, entity);
                            println!("Adding {:?} to Camera {:?}", component.get_name(), entity);
                        }
                    }
                } else {
                    for (entity, camera) in q.iter() {
                        if camera.is_active {
                            component.remove_from(&mut commands, entity);
                            println!("Remove {:?} from Camera {:?}", component.get_name(), entity);
                        }
                    }
                }
            }
        }
    }
}
