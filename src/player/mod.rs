use bevy::{animation::animation_player, prelude::*};

pub mod components;
mod resources;
mod systems;

use bevy_ecs_ldtk::prelude::LdtkEntityAppExt;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(keyboard_input)
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_system(animation_state_update)
            .add_system(animation_player);
    }
}
