use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
   fn build(&self, app: &mut App) {
       app
       .add_startup_system(spawn_player)
        // .add_system(player_movement)
        // .add_system(confine_player_movement.after(player_movement))
        .add_systems((
            player_movement,
            confine_player_movement).chain())
        .add_system(enemy_hit_player)
        .add_system(player_hit_star);
   }
}