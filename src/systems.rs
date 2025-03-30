use std::collections::HashSet;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{Ground, GroundSensor, OnGround};

pub fn process_ground_start_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<GroundSensor>, Without<OnGround>)>,
    query_ground: Query<Entity, With<Ground>>,
    mut collision_start_event_reader: EventReader<CollisionStarted>,
) {
    let groundsensor_entities: HashSet<Entity> = query_groundsensor.iter().collect();
    let ground_entities: HashSet<Entity> = query_ground.iter().collect();

    for CollisionStarted(c1, c2) in collision_start_event_reader.read() {
        if groundsensor_entities.contains(c1) && ground_entities.contains(c2) {
            commands.entity(*c1).insert(OnGround);
        } else if groundsensor_entities.contains(c2) && ground_entities.contains(c1) {
            commands.entity(*c2).insert(OnGround);
        }
    }
}

pub fn process_ground_end_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<GroundSensor>, With<OnGround>)>,
    query_ground: Query<Entity, With<Ground>>,
    mut collision_ended_event_reader: EventReader<CollisionEnded>,
) {
    let groundsensor_entities: HashSet<Entity> = query_groundsensor.iter().collect();
    let ground_entities: HashSet<Entity> = query_ground.iter().collect();

    for CollisionEnded(c1, c2) in collision_ended_event_reader.read() {
        if groundsensor_entities.contains(c1) && ground_entities.contains(c2) {
            commands.entity(*c1).remove::<OnGround>();
        } else if groundsensor_entities.contains(c2) && ground_entities.contains(c1) {
            commands.entity(*c2).remove::<OnGround>();
        }
    }
}
