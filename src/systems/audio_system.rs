use specs::{System, Write};
use crate::audio::AudioStore;

pub struct AudioSystem {}

impl<'a> System<'a> for AudioSystem {
    type SystemData = Write<'a, AudioStore>;

    fn run(&mut self, mut audio_store: Self::SystemData) {
        audio_store.frame();
    }
}