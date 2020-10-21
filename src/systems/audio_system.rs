use crate::audio::AudioStore;
use specs::{System, Write};

pub struct AudioSystem {}

impl<'a> System<'a> for AudioSystem {
    type SystemData = Write<'a, AudioStore>;

    fn run(&mut self, mut audio_store: Self::SystemData) {
        audio_store.frame();
    }
}
