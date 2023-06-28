use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS:i32 = 6;
pub const  STAR_SIZE: f32 = 32.;

use systems::*;
use resources::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_star)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time);
    

    }
}