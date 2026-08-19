mod library;
mod zooma_error;

use library::*;
use raylib::prelude::*;
use std::process;
use zooma_error::ZoomaError;

// TODO:
// - Image panning
// - Circle effect
// - Screenshot on Windows

fn main() {
    match take_screenshot() {
        Ok(_) => (), // Success
        Err(e) => match e {
            ZoomaError::NoWlroots => todo!("Handle non-wlroots Wayland"),
            ZoomaError::MissingXdgSessionType => {
                println!("Failed to read $XDG_SESSION_TYPE environment variable");
                process::exit(1);
            },
            ZoomaError::InvalidXdgSessionType(session_value) => {
                println!("Invalid $XDG_SESSION_TYPE, expected either value \"x11\" or \"wayland\", got \"{:}\"", session_value);
                process::exit(1);
            },
            ZoomaError::MissingDependency(dep) => {
                println!("Missing dependency: \'{:}\'", dep);
                process::exit(1);
            },
        },
    }

    // Initialize Raylib
    let (mut rl, rl_thread) = raylib::init()
        .title("Zooma")
        .resizable()
        .fullscreen()
        .build();
    rl.set_target_fps(60);

    let img = Image::load_image(TMP_SS_PATH).expect("Failed to load temporary screenshot");
    let mut ss_texture = rl
        .load_texture_from_image(&rl_thread, &img)
        .expect("Failed to create texture");

    //let mut render_size    = I32Vector::new(0, 0);
    let mut image_position = I32Vector::new(0, 0);
    let mut drag_offset = I32Vector::new(0, 0);

    while !rl.window_should_close() {
        let mut win = rl.begin_drawing(&rl_thread);
        win.clear_background(Color::BLACK);
        win.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);

        // --- Panning -------------------------------------------------
        if win.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            win.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_RESIZE_ALL);

            let delta = win.get_mouse_delta();
            drag_offset.x += delta.x as i32;
            drag_offset.y += delta.y as i32;
        }
        // -------------------------------------------------------------

        // --- Zooming -------------------------------------------------
        // TODO: Set limit on size to prevent overflow crash
        // TODO: Centered zoom: offset image position so zoom occurs at cursor position

        let wheel_move = win.get_mouse_wheel_move();
        if wheel_move > 0.0 || win.is_key_down(KeyboardKey::KEY_EQUAL) {
            ss_texture.width += (ss_texture.width as f32 * 0.05) as i32;
            ss_texture.height += (ss_texture.height as f32 * 0.05) as i32;
        } else if wheel_move < 0.0 || win.is_key_down(KeyboardKey::KEY_MINUS) {
            ss_texture.width -= (ss_texture.width as f32 * 0.05) as i32;
            ss_texture.height -= (ss_texture.height as f32 * 0.05) as i32;
        }
        // -------------------------------------------------------------

        // Set image display position
        image_position.x = 0 + drag_offset.x;
        image_position.y = 0 + drag_offset.y;

        // Show image on screen
        win.draw_texture(
            &ss_texture,
            image_position.x,
            image_position.y,
            Color::RAYWHITE,
        );
    }
}
