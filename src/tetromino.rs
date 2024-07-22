use macroquad::prelude::*;

use crate::animator::AnimationRegulator;
use crate::constants::*;
use crate::data::Vec2;
use crate::{CELL_SIZE, POSSIBLE_POSITIONS};

#[derive(Debug, Clone, Copy)]
pub enum TType {
    I,
    O,
    T,
    L,
    S,
    Z,
    J,
}

impl From<TType> for Color {
    fn from(value: TType) -> Self {
        use TType::*;
        match value {
            I => *GOLDEN_GRASS,
            O => *BURNT_SIENNA2,
            T => *CASCADE,
            L => *CLAY_CREEK,
            S => *FLAME_PEA,
            Z => *BURNT_SIENNA1,
            J => *RAVEN,
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    State1, State2, State3, State4,
}

impl Rotation {
    pub fn next(&self) -> Self {
        use Rotation::*;
        match self {
            State1 => State2,
            State2 => State3,
            State3 => State4,
            State4 => State1,
        }
    }

    pub fn prev(&self) -> Self {
        use Rotation::*;
        match self {
            State1 => State4,
            State2 => State1,
            State3 => State2,
            State4 => State3,
        }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::State1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    pub _type: TType,
    pub rotation: Rotation,
    pub offset: Vec2,
}

impl Tetromino {
    pub fn new(_type: TType) -> Self {
        let offset = match _type {
            TType::I => Vec2::new(-1., 3.),
            TType::O => Vec2::new(0., 4.),
            _ => Vec2::new(0., 3.),
        };

        Self {
            _type,
            rotation: Rotation::default(),
            offset,
        }
    }

    pub fn get_curr_positions(&self) -> [Vec2; 4] {
        POSSIBLE_POSITIONS[self._type as usize][self.rotation as usize]
            .map(Vec2::from)
            .map(|v| v + self.offset)
    }

    pub fn rotate(&mut self) {
        self.rotation = self.rotation.next()
    }

    pub fn undo_rotate(&mut self) {
        self.rotation = self.rotation.prev()
    }

    pub fn move_pos(&mut self, vec: Vec2) {
        self.offset = self.offset + vec
    }

    pub async fn draw(&self) {
        for tile in self.get_curr_positions() {
            draw_rectangle(
                tile.col * CELL_SIZE + 11.,
                tile.row * CELL_SIZE + 11.,
                CELL_SIZE - 1.,
                CELL_SIZE - 1.,
                self._type.into(),
            )
        }
    }

    pub async fn draw_outline(&self, reg: &AnimationRegulator) {
        for tile in self.get_curr_positions() {
            draw_rectangle_lines(
                tile.col * CELL_SIZE + 11.,
                tile.row * CELL_SIZE + 11.,
                CELL_SIZE - 1.,
                CELL_SIZE - 1.,
                5.,
                reg.colour_blink_map(1, self._type.into()),
            )
        }
    }

    pub async fn draw_with_offset(&self, offset: Vec2) {
        for tile in self.get_curr_positions() {
            draw_rectangle(
                offset.col + tile.col * CELL_SIZE,
                offset.row + tile.row * CELL_SIZE,
                CELL_SIZE - 1.,
                CELL_SIZE - 1.,
                self._type.into(),
            )
        }
    }
}
