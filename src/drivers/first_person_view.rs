use bevy::prelude::Component;
use driver_marker_derive::DriverMarker;
use std::any::TypeId;
use crate::Commands;
use bevy::ecs::entity::Entity;

use crate::driver::driver_core::DriverMarker;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct FPV;

// TODO impl