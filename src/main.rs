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

    // TODO: Make these into i32 vectors
    let mut render_size    = Vector2::new(0.0, 0.0);
    let mut image_position = Vector2::new(0.0, 0.0);
    let mut drag_offset    = Vector2::new(0.0, 0.0);

    while !rl.window_should_close() {
        let mut win = rl.begin_drawing(&rl_thread);

        (render_size.x, render_size.x) = get_render_size(&win);

        // --- Panning -------------------------------------------------
        if win.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            win.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_RESIZE_ALL);

            let delta = win.get_mouse_delta();
            drag_offset.x += delta.x as i32;
            drag_offset.y += delta.y as i32;
        }

        if win.is_key_down(KeyboardKey::KEY_W) {
            drag_offset.y -= 10;
        }
        if win.is_key_down(KeyboardKey::KEY_A) {
            drag_offset.x -= 10;
        }
        if win.is_key_down(KeyboardKey::KEY_S) {
            drag_offset.y += 10;
        }
        if win.is_key_down(KeyboardKey::KEY_D) {
            drag_offset.x += 10;
        }
        // -------------------------------------------------------------


        // --- Zooming -------------------------------------------------
        // TODO: Set limit on size to prevent overflow crash

        let wheel_move = win.get_mouse_wheel_move();
        if wheel_move > 0.0 || win.is_key_down(KeyboardKey::KEY_EQUAL){
            img_tex.width  += (img_tex.width  as f32 * 0.05) as i32;
            img_tex.height += (img_tex.height as f32 * 0.05) as i32;
        } else if wheel_move < 0.0 || win.is_key_down(KeyboardKey::KEY_MINUS) {
            img_tex.width  -= (img_tex.width  as f32 * 0.05) as i32;
            img_tex.height -= (img_tex.height as f32 * 0.05) as i32;
        }
        // -------------------------------------------------------------

        // Set image display position
        image_position.x = render_size.w / 2 - img_tex.width  / 2 + drag_offset.x;
        image_position.y = render_size.h / 2 - img_tex.height / 2 + drag_offset.y;

        // Show image on screen
        win.draw_texture(
            &ss_texture,
            image_position.x,
            image_position.y,
            Color::RAYWHITE);
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

fn get_render_size(rdh: &RaylibDrawHandle) -> (i32, i32) {
    return (rdh.get_render_width(), rdh.get_render_height());
}
