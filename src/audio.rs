use specs::{World, WorldExt};
use std::collections::HashMap;
use quad_snd::mixer::{Sound, SoundMixer};
use macroquad::file::load_file;
use quad_snd::decoder::read_wav;
use std::sync::{Arc, Mutex};

pub struct AudioStore {
    pub sounds: HashMap<String, Sound>,
    pub mixer: Arc<Mutex<SoundMixer>>,
}

impl Default for AudioStore {
    fn default() -> Self {
        Self {
            sounds: HashMap::new(),
            mixer: Arc::new(Mutex::new(SoundMixer::new())),
        }
    }
}

impl AudioStore {
    pub fn play_sound(&mut self, sound: &String) {
        self.mixer
            .lock()
            .unwrap()
            .play(self.sounds.get(sound).unwrap().clone());
    }

    pub fn frame(&mut self) {
        self.mixer
            .lock()
            .unwrap().frame();
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
