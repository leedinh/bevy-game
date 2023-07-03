use bevy::prelude::*;

pub mod components;
mod systems;

use super::SimulationState;
use crate::AppState;
use systems::*;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
   fn build(&self, app: &mut App) {
    app
        // Configure System Sets
        .configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
        // On Enter State
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_systems(
            (
                player_movement.in_set(PlayerSystemSet::Movement),
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        )
        .add_systems(
            (enemy_hit_player, player_hit_star)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
        )
        // On Exit State
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
}
}