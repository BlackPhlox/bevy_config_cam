
use crate::{FPV, Pinned};
use std::any::TypeId;

use bevy::{prelude::{
    Resource, Entity,
    Commands, Component, Query}, ecs::{component::TableStorage, system::SystemParam}};
use bevy_dolly::prelude::Rig;


pub trait DriverMarker: Component<Storage = TableStorage> + Sync + Send + 'static {
    fn get_id(&self) -> TypeId;
    fn get_name(&self) -> &str;
    fn add_to(&self, commands: &mut Commands, entity: Entity);
    fn remove_from(&self, commands: &mut Commands, entity: Entity);
}

#[derive(Resource)]
pub struct Drivers{
    // access must happen in this folder or children
    pub (super) marker: Vec<Box<dyn DriverMarker>>,
} 

impl Default for Drivers {
    fn default() -> Self {
        Self {
            marker: vec![Box::new(Pinned), Box::new(FPV)]
        }
    }
}

impl Drivers {
    pub fn new(marker: Vec<Box<dyn DriverMarker>>) -> Self {
        Self{marker}
    }
}

#[derive(Default, Resource)]
pub struct DriverIndex {
    // access must happen in this folder or children
    pub (super) index: usize,
} 

impl DriverIndex {

    /// Goes to next index, loops back to index `0` if already at the last index.
    pub fn next(&mut self, len: usize) {
        if self.index >= len - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}

#[derive(SystemParam)]
pub struct DriverRigs<'w, 's> {
    rigs: Query<'w, 's, &'static mut Rig>,
}

impl<'w, 's> DriverRigs<'w, 's> {
    pub fn try_for_each_driver_mut<T>(&mut self, f: impl FnOnce(&mut T) + std::marker::Copy)
    where
        T: bevy_dolly::prelude::RigDriver<bevy_dolly::prelude::RightHanded>,
    {
        for mut rig in &mut self.rigs {
            if let Some(camera_driver) = rig.try_driver_mut::<T>() {
                f(camera_driver);
            }
        }
    }
}
