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
    driver_index: usize,
    pub cameras: Vec<TypeId>,
    camera_index: usize,
}

impl Default for Drivers {
    fn default() -> Self {
        Self {
            enabled_drivers: vec![Box::new(CCOrbit), Box::new(CCFpv)],
            cameras: vec![TypeId::of::<MainCamera>()],
            driver_index: Default::default(),
            camera_index: Default::default(),
        }
    }
}

impl Drivers {
    pub fn new(enabled_drivers: Vec<Box<dyn DriverMarker>>, cameras: Vec<TypeId>) -> Self {
        Self {
            enabled_drivers,
            driver_index: Default::default(),
            cameras,
            camera_index: Default::default(),
        }
    }

    pub fn next(&mut self) {
        if self.driver_index >= self.enabled_drivers.len() - 1 {
            self.driver_index = 0;
        } else {
            self.driver_index += 1;
        }
    }

    pub fn change_camera<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.cameras.push(type_id);
        self.cameras.dedup();
        if let Some((x, _)) = self
            .cameras
            .iter()
            .enumerate()
            .find(|(_, id)| (*id).clone().eq(&type_id))
        {
            self.camera_index = x;
        }
    }

    pub fn index(&self) -> usize {
        self.driver_index
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
