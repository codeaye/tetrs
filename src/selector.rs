use crate::tetromino::{TType, Tetromino};
use macroquad::rand::gen_range;

pub struct Selector {
    choices: Vec<TType>,
    pub current: Tetromino,
    pub ghost: Tetromino,
    pub next: Tetromino,
}

impl Selector {
    pub fn reset(&mut self) {
        use TType::*;
        self.choices.push(I);
        self.choices.push(O);
        self.choices.push(T);
        self.choices.push(L);
        self.choices.push(S);
        self.choices.push(Z);
        self.choices.push(J);
    }

    pub fn next(&mut self) -> TType {
        if self.choices.is_empty() {
            self.reset();
        }
        let chosen = gen_range(0, self.choices.len());
        let cloned = self.choices[chosen];
        self.choices.remove(chosen);
        cloned
    }

    pub fn block_locked(&mut self) {
        self.ghost = self.next;
        self.current = self.next;
        self.next = Tetromino::new(self.next());
    }
}

impl Default for Selector {
    fn default() -> Self {
        use TType::*;

        let mut choices = vec![I, O, T, L, S, Z, J];

        fn random(choices: &mut Vec<TType>) -> TType {
            let chosen = gen_range(0, choices.len());
            let cloned = choices[chosen];
            choices.remove(chosen);
            cloned
        }

        let current = Tetromino::new(random(&mut choices));

        Self {
            ghost: current,
            current,
            next: Tetromino::new(random(&mut choices)),
            choices,
        }
    }
}
