mod animator;
mod constants;
mod data;
mod game;
mod grid;
mod selector;
mod sounds;
mod tetromino;

use constants::*;
use game::Game;
use macroquad::prelude::*;
use miniquad::conf::Icon;

fn load_img(bytes: &'static [u8]) -> Image {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap()
}

fn populate_array(img: Image, array: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.get_image_data() {
        for value in pixel.iter() {
            array[index] = *value;
            index += 1;
        }
    }
}

fn window_conf() -> Conf {
    let mut small: [u8; 1024] = [0; 1024];
    let mut medium: [u8; 4096] = [0; 4096];
    let mut big: [u8; 16384] = [0; 16384];

    populate_array(
        load_img(include_bytes!("../resources/icons/16x16.png")),
        &mut small,
    );
    populate_array(
        load_img(include_bytes!("../resources/icons/32x32.png")),
        &mut medium,
    );
    populate_array(
        load_img(include_bytes!("../resources/icons/64x64.png")),
        &mut big,
    );
    Conf {
        window_title: "❤️ Tetrs ❤️".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        high_dpi: true,
        icon: Some(Icon { small, medium, big }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    game.run().await
}
