use std::fmt::Debug;

use crate::{
    animator::AnimationRegulator,
    constants::{CELL_SIZE, NUM_COLS, NUM_ROWS},
    data::Vec2,
    tetromino::TType,
    ROW_DISSAPEAR_ANIM_DURATION,
};
use macroquad::prelude::*;

pub type GridValues = Vec<Vec<Option<TType>>>;

pub struct Grid {
    values: GridValues,
    collapsed: Vec<(u16, usize, Vec<Option<TType>>)>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "╔{}╗\n",
            (0..(NUM_COLS * 2 + 1)).map(|_| "═").collect::<String>()
        ))?;
        for i in &self.values {
            f.write_str("║ ")?;
            for j in i {
                f.write_fmt(format_args!(
                    "{} ",
                    match j {
                        Some(_) => "▣",
                        None => "•",
                    }
                ))?;
            }
            f.write_str("║\n")?;
        }
        f.write_fmt(format_args!(
            "╚{}╝\n",
            (0..(NUM_COLS * 2 + 1)).map(|_| "═").collect::<String>()
        ))
    }
}

#[rustfmt::skip]
#[cfg(debug_assertions)]
impl Default for Grid {
    fn default() -> Self {
        Self {
            values: vec![
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![None,None,None,None,None,None,None, None, None, None,],
                vec![Some(TType::I),Some(TType::I),None,Some(TType::I),Some(TType::I),Some(TType::I),Some(TType::I), Some(TType::I), Some(TType::I), Some(TType::I),],
                vec![Some(TType::I),Some(TType::I),None,Some(TType::I),Some(TType::I),Some(TType::I),Some(TType::I), Some(TType::I), Some(TType::I), Some(TType::I),],
                vec![Some(TType::I),Some(TType::I),None,Some(TType::I),Some(TType::I),Some(TType::I),Some(TType::I), Some(TType::I), Some(TType::I), Some(TType::I),],
                vec![Some(TType::I),Some(TType::I),None,Some(TType::I),Some(TType::I),Some(TType::I),Some(TType::I), Some(TType::I), Some(TType::I), Some(TType::I),],
                vec![Some(TType::I),Some(TType::I),None,Some(TType::I),Some(TType::I),Some(TType::I),Some(TType::I), Some(TType::I), Some(TType::I), Some(TType::I),],
                vec![Some(TType::I),Some(TType::I),None,Some(TType::I),Some(TType::I),Some(TType::I),Some(TType::I), Some(TType::I), Some(TType::I), Some(TType::I),],
                ],
            collapsed: Vec::with_capacity(NUM_COLS),
        }
    }
}

#[cfg(not(debug_assertions))]
impl Default for Grid {
    fn default() -> Self {
        Self {
            values: vec![vec![None; NUM_COLS]; NUM_ROWS],
            collapsed: Vec::with_capacity(NUM_COLS),
        }
    }
}

impl Grid {
    pub async fn draw(&self) {
        for (row, valr) in self.values.iter().enumerate() {
            for (col, val) in valr.iter().enumerate() {
                draw_rectangle(
                    col as f32 * CELL_SIZE + 11.,
                    row as f32 * CELL_SIZE + 11.,
                    CELL_SIZE - 1.,
                    CELL_SIZE - 1.,
                    match val {
                        Some(t) => (*t).into(),
                        _ => *crate::CRATER_BROWN,
                    },
                )
            }
        }
    }

    pub fn is_cell_contained(&self, pos: &Vec2) -> bool {
        if (0.0..(NUM_ROWS as f32)).contains(&pos.row)
            && (0.0..(NUM_COLS as f32)).contains(&pos.col)
        {
            return true;
        }
        false
    }

    pub fn is_empty(&self, pos: &Vec2) -> bool {
        self.values[pos.row as usize][pos.col as usize].is_none()
    }

    pub fn check_complete(&mut self, reg: &mut AnimationRegulator) -> usize {
        let completed = self
            .values
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|f| f.is_some()))
            .map(|(n, _)| n)
            .collect::<Vec<_>>();

        let total = completed.len();
        for row in completed {
            let res = self.values.remove(row);
            let id = ((get_time() * 100.) as u16 % 10000) + 2;
            self.collapsed.push((id, row, res));
            reg.reset_animation(id, ROW_DISSAPEAR_ANIM_DURATION);
            self.values.insert(0, vec![None; NUM_COLS]);
        }
        total
    }

    pub fn draw_row_collapse_animation(&mut self, reg: &mut AnimationRegulator) {
        let mut done = Vec::new();
        for (n, (id, orig_row, collapsed)) in self.collapsed.iter().enumerate() {
            let ratio = reg.animation_completed_ratio(*id) as f32;
            if ratio >= 1. {
                done.push((n, *id));
            } else {
                let k = *orig_row as f32;
                for (col, val) in collapsed.iter().enumerate() {
                    let c_size = (CELL_SIZE - 1.) * (1. - ratio);
                    let mut color: Color = val.unwrap().into();
                    color.a = (0.5) * (1. - ratio);
                    draw_rectangle(
                        col as f32 * CELL_SIZE + (0.5 * ratio) * CELL_SIZE + 11.,
                        (k + 3. * ratio).max(0.) * CELL_SIZE + 11.,
                        c_size,
                        c_size,
                        color,
                    )
                }
            }
        }

        for (l, (i, id)) in done.iter().enumerate() {
            self.collapsed.remove(i - l);
            reg.remove_animation(*id);
        }
    }

    pub fn set_type(&mut self, pos: &Vec2, col: TType) {
        self.values[pos.row as usize][pos.col as usize] = Some(col)
    }
}
