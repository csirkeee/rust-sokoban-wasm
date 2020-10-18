use crate::audio::AudioStore;
use crate::events::Event;
use macroquad::{Texture2D};
use specs::World;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

// Resources
pub enum GameplayState {
    Playing,
    Won,
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: f64,
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Default)]
pub struct ImageStore {
    pub images: HashMap<String, Texture2D>,
}

pub fn register_resources(world: &mut World) {
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
    world.insert(ImageStore::default());
}
