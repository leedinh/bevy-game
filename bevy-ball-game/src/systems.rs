use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::events::*;
use  bevy::app::AppExit;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: i32 = 10;
pub const ENEMIES_SPEED: f32 = 200.0;
pub const ENEMIES_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS:i32 = 6;
pub const  STAR_SIZE: f32 = 32.;

pub fn spawn_enemies (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites\\ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), 
                },
            )
        );
    }
}


pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();


    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites\\star.png"),
                    ..default()
                },
                Star{},
            )
        );
    }
}


pub fn spawn_player (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
                texture: asset_server.load("sprites\\ball_blue_large.png"),
                ..default()
            },
            Player {},
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,

) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            ..default()
        }
    );
}

pub fn enemies_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMIES_SPEED * time.delta_seconds();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0,0.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0,0.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0,1.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0,-1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize()
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds()
    }
}


pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        
        let half_player_size = PLAYER_SIZE / 2.;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        //Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if  translation.x > x_max 
        { translation.x = x_max}
            
        // Bound the player y position
        if translation.y < y_min { translation.y = y_min}
        else if  translation.y > y_max {
            translation.y = y_max
        }

            player_transform.translation = translation;
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
        
        let half_player_size = ENEMIES_SIZE / 2.;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        for (transform, mut enemy) in enemy_query.iter_mut() {
            let mut direction_changed = false;
            let translation = transform.translation;
            if  translation.x < x_min || translation.x > x_max {
                enemy.direction.x *= -1.0;
                direction_changed = true;
            }
            if translation.y < y_min || translation.y > y_max {
                enemy.direction.y *= -1.0;
                direction_changed = true;
            }

            if direction_changed {
                let sound_effect_1 = asset_server.load("audio\\pluck_001.ogg");
                let sound_effect_2 = asset_server.load("audio\\pluck_002.ogg");
                
                let sound_effect = if random::<f32>() > 0.5 {
                    sound_effect_1
                } else {
                    sound_effect_2
                };
                audio.play(sound_effect);
            }
        }

}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    
        let window = window_query.get_single().unwrap();
        
        let half_player_size = PLAYER_SIZE / 2.;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        for mut transform in enemy_query.iter_mut(){
            let mut translation = transform.translation;

            //Bound the player x position
            if translation.x < x_min {
                translation.x = x_min;
            } else if  translation.x > x_max 
            { translation.x = x_max}
                
            // Bound the player y position
            if translation.y < y_min { translation.y = y_min}
            else if  translation.y > y_max {
                translation.y = y_max
            }

                transform.translation = translation;
        }
}


pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform),With<Player>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
        if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
            for enemy_transform in enemy_query.iter() {
                let distance = player_transform
                                .translation
                                .distance(enemy_transform.translation);
                let player_radius = PLAYER_SIZE /2.0;
                let enemy_radius = ENEMIES_SIZE/ 2.0;
                if distance < enemy_radius + player_radius {
                    println!("Enemy hit player! Game over");
                    let sound_effect = asset_server.load("audio\\explosionCrunch_000.ogg");
                    audio.play(sound_effect);
                    commands.entity(player_entity).despawn();
                    game_over_event_writer.send(GameOver { score: score.value })
                }

            }
        }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform ,With<Player>>,
    star_query: Query<(Entity, &Transform),With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>
) {
        if let Ok(player_transform) = player_query.get_single() {
            for (star_entity, star_transform) in star_query.iter() {
                let distance = player_transform
                                .translation
                                .distance(star_transform.translation);
                let player_radius = PLAYER_SIZE /2.0;
                let star_radius = STAR_SIZE/ 2.0;
                if distance < star_radius + player_radius {
                    score.value += 1;
                    println!("Enemy hit player! Game over");
                    let sound_effect = asset_server.load("audio\\impactGlass_light_004.ogg");
                    audio.play(sound_effect);
                    commands.entity(star_entity).despawn();
                }

            }
        }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time> ) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time> ) {
    enemy_spawn_timer.timer.tick(time.delta());
}


pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();
    
            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(random_x, random_y, 0.0),
                        texture: asset_server.load("sprites\\star.png"),
                        ..default()
                    },
                    Star{},
                )
            );
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();


            let random_x = random::<f32>() * window.width();
            let random_y = random::<f32>() * window.height();
    
            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(random_x, random_y, 0.0),
                        texture: asset_server.load("sprites\\ball_red_large.png"),
                        ..default()
                    },
                    Enemy {
                        direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(), 
                    },
                )
            );
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit)
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

pub fn update_high_scores (
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScore>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScore> 
) {
    if high_scores.is_changed() {
        println!("High scores updated: {:?}", high_scores);
    }
}