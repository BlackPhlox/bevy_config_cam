use driver_marker_derive::DriverMarker;
use super::drivers::DriverMarker;

use std::any::TypeId;

use bevy::{
    prelude::Component,
    ecs::{system::Commands, entity::Entity}
};

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct FPV;

#[derive(DriverMarker, Component, Clone, Copy, Debug)]
pub struct Pinned;
//Substates:
//Locked Rotation
//Free