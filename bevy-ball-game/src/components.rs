use  bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Star {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2
}