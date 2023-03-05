use std::any::TypeId;

use bevy::{
    ecs::{component::TableStorage, system::SystemParam},
    prelude::{Commands, Component, Entity, Query, Resource},
};
use bevy_dolly::prelude::*;

use crate::{
    drivers::{fpv::CCFpv, orbit::CCOrbit},
    MainCamera,
};

pub trait DriverMarker: Component<Storage = TableStorage> + Sync + Send + 'static {
    fn get_id(&self) -> TypeId;
    fn get_name(&self) -> &str;
    fn add_to(&self, commands: &mut Commands, entity: Entity);
    fn remove_from(&self, commands: &mut Commands, entity: Entity);
}

#[derive(Resource)]
pub struct Drivers {
    // access must happen in this folder or children
    pub enabled_drivers: Vec<Box<dyn DriverMarker>>,
    pub current: TypeId,
    index: usize,
}

impl Default for Drivers {
    fn default() -> Self {
        Self {
            enabled_drivers: vec![Box::new(CCOrbit), Box::new(CCFpv)],
            index: Default::default(),
            current: TypeId::of::<MainCamera>(),
        }
    }
}

impl Drivers {
    pub fn new(marker: Vec<Box<dyn DriverMarker>>) -> Self {
        Self {
            enabled_drivers: marker,
            index: Default::default(),
            current: TypeId::of::<MainCamera>(),
        }
    }

    pub fn next(&mut self) {
        if self.index >= self.enabled_drivers.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }

    pub fn change_camera<T: 'static>(&mut self) {
        self.current = TypeId::of::<T>();
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(SystemParam)]
pub struct DriverRigs<'w, 's> {
    rigs: Query<'w, 's, &'static mut Rig>,
}

impl<'w, 's> DriverRigs<'w, 's> {
    pub fn try_for_each_driver_mut<T>(&mut self, f: impl FnOnce(&mut T) + std::marker::Copy)
    where
        T: RigDriver,
    {
        for mut rig in &mut self.rigs {
            if let Some(camera_driver) = rig.try_driver_mut::<T>() {
                f(camera_driver);
            }
        }
    }

    pub fn driver_exists<T>(self) -> bool
    where
        T: RigDriver,
    {
        for rig in &self.rigs {
            if rig.try_driver::<T>().is_some() {
                return true;
            }
        }
        false
    }
}
