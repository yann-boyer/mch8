extern crate sdl2;

use sdl2::mixer::{Channel, Chunk, MAX_VOLUME};

pub struct AudioSystem {
    beep_sound: Chunk,
    channels: Channel,
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioSystem {
    pub fn new() -> AudioSystem {
        AudioSystem {
            beep_sound: sdl2::mixer::Chunk::from_file("beep.wav").unwrap(),
            channels: Channel::all(),
        }
    }

    pub fn init(&mut self) {
        sdl2::mixer::allocate_channels(2);
        self.beep_sound.set_volume(MAX_VOLUME / 4);
    }

    pub fn play_beep_sound(&self) {
        self.channels.play(&self.beep_sound, 0).unwrap();
    }
}
