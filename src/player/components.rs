use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub block: u32,
}

#[derive(Component)]
pub struct Animated {
    pub timer: Timer,
    pub start: usize,
    pub end: usize,
    pub play_once: bool,
}
impl Animated {
    pub fn new(seconds_per_frame: f32, start: usize, end: usize, play_once: bool) -> Self {
        Self {
            timer: Timer::from_seconds(seconds_per_frame, TimerMode::Repeating),
            start,
            end,
            play_once,
        }
    }
}

impl From<&EntityInstance> for Animated {
    fn from(entity_instance: &EntityInstance) -> Self {
        match entity_instance.identifier.as_ref() {
            "Player" => Animated::new(0.1, 0, 1, false),
            _ => Animated::new(0.1, 0, 1, false),
        }
    }
}
