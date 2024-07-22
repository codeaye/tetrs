use std::ops::Add;

use macroquad::audio::play_sound_once;

use crate::sounds::Sounds;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub row: f32,
    pub col: f32,
}

impl Vec2 {
    pub fn new(row: f32, col: f32) -> Self {
        Self { row, col }
    }
}

impl From<(usize, usize)> for Vec2 {
    fn from(value: (usize, usize)) -> Self {
        Vec2::new(value.0 as f32, value.1 as f32)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.row + rhs.row, self.col + rhs.col)
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub struct GameData {
    pub level: u8,
    pub score: u32,
    frames_since_last_fall: u16,
    collected: u16,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            level: 0,
            score: 0,
            frames_since_last_fall: 0,
            collected: 9,
        }
    }
}

impl GameData {
    pub fn frames_per_drop(&self) -> u16 {
        match self.level {
            0 => 48,
            1 => 43,
            2 => 38,
            3 => 33,
            4 => 28,
            5 => 23,
            6 => 18,
            7 => 13,
            8 => 8,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1,
        }
    }

    pub fn has_dropped(&mut self) -> bool {
        if self.frames_since_last_fall >= self.frames_per_drop() {
            self.frames_since_last_fall = 0;
            true
        } else {
            self.frames_since_last_fall += 1;
            false
        }
    }

    pub fn inc_level(&mut self, sounds: &Sounds) {
        self.collected -= 10;
        if self.level < 29 {
            self.level += 1;
            play_sound_once(sounds.get(1))
        }
    }

    pub fn add_to_score(&mut self, score: u32) {
        self.score += score;
    }

    pub fn inc_score(&mut self, sounds: &Sounds, n: usize) {
        let k = (self.level + 1) as u32;
        self.collected += n as u16;

        if self.collected >= 10 {
            self.inc_level(sounds)
        }

        self.score += match n {
            0 => 0,
            1 => {
                play_sound_once(sounds.get(5));
                40 * k
            }
            2 => {
                play_sound_once(sounds.get(5));
                100 * k
            }
            3 => {
                play_sound_once(sounds.get(7));
                300 * k
            }
            _ => {
                play_sound_once(sounds.get(6));
                1200 * k
            }
        };
    }
}
