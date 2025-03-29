use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::{Ground, GroundSensor, OnGround};

pub fn process_ground_start_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<GroundSensor>, Without<OnGround>)>,
    query_ground: Query<Entity, With<Ground>>,
    mut collision_start_event_reader: EventReader<CollisionStarted>,
) {
    for CollisionStarted(c1, c2) in collision_start_event_reader.read() {
        if query_groundsensor.contains(*c1) {
            if query_ground.contains(*c2) {
                commands.entity(*c1).insert(OnGround);
            }
        } else if query_groundsensor.contains(*c2) {
            if query_ground.contains(*c1) {
                commands.entity(*c2).insert(OnGround);
            }
        }
    }
}

pub fn process_ground_end_sensor(
    mut commands: Commands,
    query_groundsensor: Query<Entity, (With<GroundSensor>, With<OnGround>)>,
    query_ground: Query<Entity, With<Ground>>,
    mut collision_ended_event_reader: EventReader<CollisionEnded>,
) {
    for CollisionEnded(c1, c2) in collision_ended_event_reader.read() {
        if query_groundsensor.contains(*c1) {
            if query_ground.contains(*c2) {
                commands.entity(*c1).remove::<OnGround>();
            }
        } else if query_groundsensor.contains(*c2) {
            if query_ground.contains(*c1) {
                commands.entity(*c2).remove::<OnGround>();
            }
        }
    }
}
