use crate::{driver::driver_core::DriverMarker, Commands};
use bevy::{ecs::entity::Entity, prelude::Component};
use driver_marker_derive::DriverMarker;
use std::any::TypeId;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct Pinned;
//Substates:
//Locked Rotation
//Free
