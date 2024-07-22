use macroquad::{color::Color, time::get_time};
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct AnimationRegulator {
    // (duration, start_time)
    data: FxHashMap<u16, (f64, f64)>,
}

impl AnimationRegulator {
    pub fn remove_animation(&mut self, id: u16) {
        self.data.remove(&id);
    }
    pub fn reset_animation(&mut self, id: u16, duration: f64) {
        self.data.insert(id, (duration, get_time()));
    }
    pub fn animation_completed_ratio(&self, id: u16) -> f64 {
        if let Some((duration, start_time)) = self.data.get(&id) {
            return (1. - (((start_time + duration) - get_time()) / duration)).clamp(0., 1.);
        }
        1.
    }

    pub fn colour_map(&self, id: u16, mut colour: Color) -> Color {
        colour.a = (self.animation_completed_ratio(id)) as f32;
        colour
    }

    pub fn as_blink(&self, id: u16) -> f64 {
        if let Some((duration, start_time)) = self.data.get(&id) {
            let a = (start_time + duration) - get_time();
            let (div, modl) = (a / duration, a % duration);
            let val = (modl / duration).abs();
            return match div.abs() as u32 % 2 == 0 {
                false => val,
                _ => 1. - val,
            }
            .clamp(0., 1.);
        }
        1.
    }

    pub fn colour_blink_map(&self, id: u16, colour: Color) -> Color {
        let mut colour = colour;
        colour.a = (self.as_blink(id)) as f32;
        colour
    }
}
