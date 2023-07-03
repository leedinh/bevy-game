use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use  enemy::EnemyPlugin;
use  player::PlayerPlugin;
use  score::ScorePlugin;
use  star::StarPlugin;
use crate::{events::GameOver, AppState, systems::pause_simulation, systems::resume_simulation};
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }

}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}