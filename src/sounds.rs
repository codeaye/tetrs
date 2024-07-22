use macroquad::audio::Sound;

pub struct Sounds {
    values: Vec<Sound>,
}

impl Sounds {
    pub async fn new() -> Self {
        macro_rules! use_sounds {
            ($($s: expr),+) => {
                vec![$(macroquad::audio::load_sound_from_bytes(include_bytes!($s)).await.unwrap()),*]
            };
        }

        let values = use_sounds!(
            /* 0 */ "../resources/sounds/gameover.wav",
            /* 1 */ "../resources/sounds/nextlevel.wav",
            /* 2 */ "../resources/sounds/sfx2rotate.wav",
            /* 3 */ "../resources/sounds/sfx4move.wav",
            /* 4 */ "../resources/sounds/sfx8drop.wav",
            /* 5 */ "../resources/sounds/sfx9double.wav",
            /* 6 */ "../resources/sounds/sfx10tetro.wav",
            /* 7 */ "../resources/sounds/sfx11triple.wav",
            /* 8 */ "../resources/sounds/maintheme.wav"
        );

        Self { values }
    }

    pub fn get(&self, id: usize) -> &Sound {
        &self.values[id]
    }
}
