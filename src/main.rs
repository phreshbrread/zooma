use raylib::prelude::*;
use std::env;
use std::process::Command;

const TMP_SS_PATH: &str = "/tmp/zooma.png";

// TODO:
// - Image panning
// - Flashlight effect
// - Screenshot on Windows

enum Environment {
    X11,
    Wayland,
    Windows,
}

fn main() {
    take_screenshot(determine_environment());

    // Initialize Raylib
    let (mut rl, rl_thread) = raylib::init()
        .title("Zooma")
        .resizable()
        .fullscreen()
        .build();
    rl.set_target_fps(60);

    let img = Image::load_image(TMP_SS_PATH).expect("Failed to load temporary screenshot");
    let ss_texture = rl
        .load_texture_from_image(&rl_thread, &img)
        .expect("Failed to create texture");

    while !rl.window_should_close() {
        let mut win = rl.begin_drawing(&rl_thread);

        win.draw_texture(&ss_texture, 0, 0, Color::RAYWHITE);
    }
}

fn determine_environment() -> Environment {
    let e = env::var("XDG_SESSION_TYPE").expect("Failed to read $XDG_SESSION_TYPE");

    match e.as_str() {
        "x11" => return Environment::X11,
        "wayland" => return Environment::Wayland,
        _ => todo!(),
    }
}

fn take_screenshot(e: Environment) {
    match e {
        Environment::X11 => {
            Command::new("scrot")
                .arg("-Z")
                .arg("0")
                .arg(TMP_SS_PATH)
                .arg("-o")
                .output()
                .expect("Failed to capture screen");
        }
        Environment::Wayland => {
            Command::new("grim")
                .arg("-l")
                .arg("0")
                .arg(TMP_SS_PATH)
                .output()
                .expect("Failed to capture screen");
        }
        Environment::Windows => {
            todo!("Windows");
        }
    }
}
