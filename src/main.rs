use raylib::prelude::*;
use miniscreenshot::{Screenshot, ImageFormat};

fn main() {
    let mut ss: Screenshot;
    ss.save_as("zooma.png", ImageFormat::Png);

    // Initialize Raylib
    let (mut rl, rl_thread) = raylib::init().title("Zooma").build();
    rl.set_target_fps(60);

    let img = Image::load_image("zooma.png").expect("Failed to load image");
    let ss_texture = rl.load_texture_from_image(&rl_thread, &img).expect("Failed to create texture");

    while !rl.window_should_close() {
        let mut win = rl.begin_drawing(&rl_thread);


        win.draw_texture(&ss_texture, 10, 10, Color::RAYWHITE);
    }
}
