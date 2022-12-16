use crate::{Commands, driver::driver_core::DriverMarker};
use bevy::{prelude::Component, ecs::entity::Entity};
use driver_marker_derive::DriverMarker;
use std::any::TypeId;


#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct Pinned;
//Substates:
//Locked Rotation
//Free