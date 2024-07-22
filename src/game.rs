use macroquad::{
    audio::{play_sound, play_sound_once, set_sound_volume, PlaySoundParams},
    prelude::*,
};
use macroquad_text::Fonts;

use crate::{
    animator::AnimationRegulator,
    constants::*,
    data::{GameData, GameState, Vec2},
    grid::Grid,
    selector::Selector,
    sounds::Sounds,
    tetromino::{TType, Tetromino},
};

pub struct Game<'a> {
    selector: Selector,
    grid: Grid,
    state: GameState,
    fonts: Fonts<'a>,
    data: GameData,
    sounds: Sounds,
    has_current_changed: bool,
    animation_handler: AnimationRegulator,
}

impl Game<'_> {
    pub async fn new() -> Self {
        let mut fonts = Fonts::default();
        fonts.load_font_from_bytes("Main", FONT).unwrap();
        Self {
            fonts,
            has_current_changed: false,
            sounds: Sounds::new().await,
            selector: Selector::default(),
            grid: Grid::default(),
            state: GameState::default(),
            data: GameData::default(),
            animation_handler: AnimationRegulator::default(),
        }
    }

    pub fn is_inside(&self, block: &Tetromino) -> bool {
        block
            .get_curr_positions()
            .iter()
            .all(|tile| self.grid.is_cell_contained(tile) && self.grid.is_empty(tile))
    }

    pub fn is_block_inside(&self) -> bool {
        self.is_inside(&self.selector.current)
    }

    pub fn is_ghost_inside(&self) -> bool {
        self.is_inside(&self.selector.ghost)
    }

    pub fn lock_block(&mut self) {
        let ty = self.selector.current._type;
        for tile in self.selector.current.get_curr_positions() {
            self.grid.set_type(&tile, ty)
        }
        self.selector.block_locked();
        if !self.is_block_inside() {
            self.state = GameState::GameOver;
            self.animation_handler
                .reset_animation(0, GAMEOVER_ANIM_DURATION);
            play_sound_once(self.sounds.get(0));
            set_sound_volume(self.sounds.get(0), 0.5);
        } else {
            play_sound_once(self.sounds.get(4));
            self.has_current_changed = true;
        }
    }

    pub async fn draw_game_ui(&self) {
        self.fonts.draw_text("LEVEL:", 350., 20.0, 50, *GOLD_SAND);
        draw_rectangle(340., 80., 130., 40., *CRATER_BROWN);
        self.fonts
            .draw_text(&format!("{}", self.data.level), 350., 80.0, 32, *ZOMBIE);

        self.fonts.draw_text("SCORE:", 350., 140.0, 50, *GOLD_SAND);
        draw_rectangle(340., 200., 130., 40., *CRATER_BROWN);
        self.fonts
            .draw_text(&format!("{}", self.data.score), 350., 200.0, 32, *ZOMBIE);

        self.fonts.draw_text("NEXT:", 350., 280.0, 50, *GOLD_SAND);
        draw_rectangle(340., 340., 130., 140., *CRATER_BROWN);

        self.selector
            .next
            .draw_with_offset(match self.selector.next._type {
                TType::I => Vec2::new(395., 255.),
                TType::O => Vec2::new(380., 255.),
                _s => Vec2::new(380., 270.0),
            })
            .await
    }

    pub async fn draw_game_over(&self) {
        draw_rectangle(
            0.,
            0.,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
            self.animation_handler.colour_map(0, *EERIE_BLACK),
        );

        self.fonts.draw_text(
            "GAME OVER",
            (WINDOW_WIDTH as f32 - 380.) / 2.,
            150.,
            100,
            self.animation_handler.colour_map(0, *BURNT_SIENNA1),
        );
        let zombie = self.animation_handler.colour_map(0, *ZOMBIE);
        self.fonts
            .draw_text("LEVEL", (WINDOW_WIDTH as f32 - 380.) / 2., 250., 60, zombie);
        self.fonts.draw_text(
            &format!(": {}", self.data.level),
            ((WINDOW_WIDTH as f32 - 380.) / 2.) + 140.,
            250.,
            60,
            zombie,
        );
        self.fonts
            .draw_text("SCORE", (WINDOW_WIDTH as f32 - 380.) / 2., 310., 60, zombie);
        self.fonts.draw_text(
            &format!(": {}", self.data.score),
            ((WINDOW_WIDTH as f32 - 380.) / 2.) + 140.,
            310.,
            60,
            zombie,
        );
        self.fonts.draw_text(
            "Press [R] to retry!",
            (WINDOW_WIDTH as f32 - 132.5) / 2.,
            400.,
            20,
            self.animation_handler
                .colour_blink_map(0, *crate::constants::BROWN),
        );
    }

    pub async fn draw(&mut self) {
        // print!("\x1B[2J\x1B[1;1H");
        // println!("{:?}", self.grid);

        self.grid.draw().await;
        self.selector.current.draw().await;

        self.selector
            .ghost
            .draw_outline(&self.animation_handler)
            .await;

        self.draw_game_ui().await;
        self.grid
            .draw_row_collapse_animation(&mut self.animation_handler);

        if self.state == GameState::GameOver {
            self.draw_game_over().await;
        }
    }

    pub async fn update(&mut self) {
        if is_key_pressed(KeyCode::Right) {
            self.selector.current.move_pos(Vec2::new(0., 1.));
            if !self.is_block_inside() {
                self.selector.current.move_pos(Vec2::new(0., -1.));
            } else {
                play_sound_once(self.sounds.get(3))
            }
        }
        if is_key_pressed(KeyCode::Left) {
            self.selector.current.move_pos(Vec2::new(0., -1.));
            if !self.is_block_inside() {
                self.selector.current.move_pos(Vec2::new(0., 1.));
            } else {
                play_sound_once(self.sounds.get(3))
            }
        }
        if is_key_down(KeyCode::Down) {
            if !self.has_current_changed {
                self.selector.current.move_pos(Vec2::new(1., 0.));
                if !self.is_block_inside() {
                    self.selector.current.move_pos(Vec2::new(-1., 0.));
                    self.lock_block();
                } else {
                    self.data.add_to_score(1);
                }
            }
        } else {
            self.has_current_changed = false;
        }
        if is_key_pressed(KeyCode::Up) {
            self.selector.current.rotate();
            if !self.is_block_inside() {
                self.selector.current.undo_rotate();
            } else {
                play_sound_once(self.sounds.get(2))
            }
        }

        if self.data.has_dropped() {
            self.selector.current.move_pos(Vec2::new(1., 0.));
            if !self.is_block_inside() {
                self.selector.current.move_pos(Vec2::new(-1., 0.));
                self.lock_block();
            }
        }

        self.selector.ghost.offset = self.selector.current.offset;
        self.selector.ghost.rotation = self.selector.current.rotation;
        while self.is_ghost_inside() {
            self.selector.ghost.move_pos(Vec2::new(1., 0.))
        }
        self.selector.ghost.move_pos(Vec2::new(-1., 0.));

        self.data.inc_score(
            &self.sounds,
            self.grid.check_complete(&mut self.animation_handler),
        );
    }

    pub async fn run(&mut self) {
        self.animation_handler.reset_animation(1, 0.4);
        let render_target = render_target(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams2::default(),
        )
        .unwrap();
        let camera = Camera2D {
            zoom: vec2(2. / WINDOW_WIDTH as f32, 2. / WINDOW_HEIGHT as f32),
            target: vec2(screen_width() / 2., screen_height() / 2.),
            render_target: Some(render_target.clone()),
            ..Default::default()
        };

        play_sound(
            self.sounds.get(8),
            PlaySoundParams {
                looped: true,
                volume: 0.4,
            },
        );

        loop {
            // set_window_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
            let playing = self.state == GameState::Playing;
            if playing {
                self.update().await;
            }

            // Restart

            if is_key_pressed(KeyCode::R) && !playing {
                self.selector = Selector::default();
                self.grid = Grid::default();
                self.data = GameData::default();
                self.animation_handler = AnimationRegulator::default();
                self.state = GameState::default();
            }

            set_camera(&camera);
            clear_background(*COCOA_BROWN);
            self.draw().await;
            set_default_camera();

            clear_background(*COCOA_BROWN);
            gl_use_material(&material);
            draw_texture_ex(
                &render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                    ..Default::default()
                },
            );
            gl_use_default_material();

            #[cfg(debug_assertions)]
            draw_text(&format!("FPS: {}", get_fps()), 5., 20., 20., YELLOW);

            next_frame().await;
        }
    }
}
