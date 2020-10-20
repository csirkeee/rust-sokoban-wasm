use specs::{World, WorldExt};
use std::collections::HashMap;
use quad_snd::mixer::Sound;
use macroquad::file::load_file;
use quad_snd::decoder::read_wav;

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, Sound>,
}

impl AudioStore {
    pub fn play_sound(&mut self, sound: &String) {
        // let _ = self
        //     .sounds
        //     .get_mut(sound)
        //     .expect("expected sound")
        //     .play_detached();
    }
}

pub async fn initialize_sounds(world: &mut World) {
    let mut audio_store = world.write_resource::<AudioStore>();
    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("resources/sounds/{}.wav", sound_name);
        let sound_bytes = load_file(&sound_path).await.unwrap();

        audio_store.sounds.insert(sound_name, read_wav(&sound_bytes[..]).unwrap());
    }
}
