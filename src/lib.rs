use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

#[derive(Default)]
pub struct GravixPlugin;

impl Plugin for GravixPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (process_ground_start_sensor, process_ground_end_sensor),
        );
    }
}
