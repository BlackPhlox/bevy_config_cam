use bevy::{
    ecs::component::TableStorage,
    input::Input,
    prelude::{
        App, Camera, Commands, Component, Entity, KeyCode, Plugin, Query, ReflectComponent, Res,
        ResMut,
    },
    reflect::{reflect_trait, Reflect, TypeRegistry},
    render::camera,
};
use bevy_dolly::prelude::*;
use driver_marker_derive::DriverMarker;

pub struct ConfigCam;
impl Plugin for ConfigCam {
    fn build(&self, app: &mut App) {
        app.init_resource::<DriverIndex>()
            .init_resource::<Drivers>()
            .add_dolly_component(FPV)
            .add_dolly_component(FPV2)
            .add_system(change_driver_system)
            .add_system(update_driver_system); //.add_dolly_component(FPV)
    }
}

#[derive(Component, DriverMarker, Clone, Copy, Debug)]
pub struct FPV;

#[derive(Component, DriverMarker, Clone, Copy, Debug)]
pub struct FPV2;

pub struct Drivers(Vec<Box<dyn DriverMarker>>);

impl Default for Drivers {
    fn default() -> Self {
        Self(vec![Box::new(FPV), Box::new(FPV2)])
    }
}

pub trait DriverMarker: Sync + Send + 'static {
    fn get_component(&self) -> &str;
    fn add_component(&self, commands: &mut Commands, entity: Entity);
    fn remove_component(&self, commands: &mut Commands, entity: Entity);
}

#[derive(Default)]
pub struct DriverIndex(usize);

//Use collection with and keeping an index component or the like
fn change_driver_system(
    mut index: ResMut<DriverIndex>,
    drivers: Res<Drivers>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::T) {
        if index.0 >= drivers.0.len() - 1 {
            index.0 = 0;
        } else {
            index.0 += 1;
        }
    }
}

fn update_driver_system(
    q: Query<(Entity, &Camera)>,
    mut commands: Commands,
    index: Res<DriverIndex>,
    drivers: Res<Drivers>,
) {
    if index.is_changed() {
        for component in &drivers.0 {
            let a = component.as_ref();
            let b = a.get_component();

            if let Some(h) = drivers.0.get(index.0) {
                if b.eq(h.get_component()) {
                    //Add new driver component
                    //Remove old driver component
                    for (entity, camera) in q.iter() {
                        if camera.is_active {
                            a.add_component(&mut commands, entity);
                            println!("Adding {:?} to Camera {:?}", b, entity);
                        }
                    }
                } else {
                    for (entity, camera) in q.iter() {
                        if camera.is_active {
                            a.remove_component(&mut commands, entity);
                            println!("Remove {:?} from Camera {:?}", b, entity);
                        }
                    }
                }
            }
        }
    }
}
