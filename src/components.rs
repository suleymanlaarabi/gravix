use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Collider)]
pub struct GroundSensor;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct OnGround;
