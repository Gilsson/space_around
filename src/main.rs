use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_ldtk::prelude::*;
use bevy_editor_pls::EditorPlugin;
use player::PlayerPlugin;

pub mod player;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(EditorPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_map)
        .insert_resource(LevelSelection::Index(0))
        .run();
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 3.),
        projection: OrthographicProjection::default(),
        ..default()
    });
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        transform: Transform::from_xyz(-1000., -1000., 0.).with_scale(Vec3 {
            x: 2.,
            y: 2.,
            z: 1.,
        }),
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..Default::default()
    });
}
