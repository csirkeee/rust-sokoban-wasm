use specs::{System, Write};
use crate::audio::AudioStore;

pub struct AudioSystem {}

impl<'a> System<'a> for AudioSystem {
    type SystemData = (Write<'a, AudioStore>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut audio_store) = data;

        audio_store.frame();
    }
}