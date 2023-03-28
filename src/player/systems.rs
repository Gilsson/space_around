use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::app::LdtkEntity;
use bevy_ecs_ldtk::{LdtkEntity, LevelSelection};
use bevy_inspector_egui::egui::Shape;

use super::components::*;

pub const PLAYER_SPEED: f32 = 1.0;
pub const PLAYER_SIZE: f32 = 250.0;

pub fn spawn_player(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(100., 100., 2.),
            texture: asset_server.load("shooter_character.png"),
            ..Default::default()
        },
        Player {
            block: 123,
            name: "Stas".to_string(),
        },
    ));
}

pub fn resolve_border_colission(
    mut liver_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut liver) = liver_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_size = PLAYER_SIZE / 2.0;
        let x_max = window.width() - half_size;
        let x_min = half_size;
        let y_max = window.height() - half_size;
        let y_min = half_size;

        let mut translation = liver.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        liver.translation = translation;
    }
}

pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut block_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = block_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

#[derive(Component, Eq, PartialEq)]
pub enum AnimationState {
    Idle,
    Walking,
    JumpUp,
    JumpDown,
    Death,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[bundle]
    #[sprite_sheet_bundle("shooter_character.png", 16.0, 16.0, 8, 15, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub animated: Animated,
}

pub fn animation_state_update(
    mut query: Query<
        (&mut Animated, &AnimationState, &mut TextureAtlasSprite),
        Changed<AnimationState>,
    >,
) {
    for (mut animation, state, mut atlas) in query.iter_mut() {
        match state {
            AnimationState::Idle => {
                animation.start = 40;
                animation.end = 44;
            }
            AnimationState::Walking => {
                animation.start = 8;
                animation.end = 14;
            }
            AnimationState::JumpUp => {
                animation.start = 56;
                animation.end = 59;
            }
            AnimationState::JumpDown => {
                animation.start = 48;
                animation.end = 51;
            }
            AnimationState::Death => {
                animation.start = 0;
                animation.end = 8;
            }
        }
        atlas.index = animation.start;
    }
}

pub fn set_spawn(
    mut commands: Commands,
    level_selection: Res<LevelSelection>,
    mut query: Query<(Entity, &mut Player, &mut Transform), Added<Player>>,
    camera: Query<Entity, With<Camera>>,
) {
    for (entity, mut player, mut transform) in query.iter_mut() {
        // Note: for some reason player transform is wrong when this system runs so I've hard coded
        // it for now
        transform.translation.z = 7.0;
        for camera in camera.iter() {
            commands.entity(entity).add_child(camera);
        }
    }
}

pub fn system(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut Animated)>) {
    let delta = time.delta();
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(delta);
        if animation.timer.finished() {
            if sprite.index < animation.start {
                sprite.index = animation.start;
            }
            sprite.index = ((sprite.index - animation.start + 1)
                % (animation.end - animation.start))
                + animation.start;
            if animation.play_once && sprite.index + 1 == animation.end {
                animation.start = sprite.index;
            }
        }
    }
}
