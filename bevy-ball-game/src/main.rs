use bevy::prelude::*;
pub mod events;
mod systems;
mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use  systems::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
    .add_plugin(GamePlugin)
    .add_plugin(MainMenuPlugin)
    .add_startup_system(spawn_camera)
    .add_system(transition_to_game_state)
    // .add_system(transition_to_main_menu_state)
    .add_system(exit_game)
    .add_system(handle_game_over)
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}
