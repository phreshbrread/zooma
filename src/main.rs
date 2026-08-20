mod library;
mod zooma_error;
use library::*;

use raylib::prelude::*;

fn main() {
    match take_screenshot() {
        Ok(()) => (), // Success
        Err(e) => handle_zooma_error(e),
    }

    // Initialize Raylib
    let (mut rl, rl_thread) = raylib::init()
        .title("Zooma")
        .resizable()
        .fullscreen()
        .vsync()
        .build();
    rl.set_target_fps(60);

    let img = Image::load_image(TMP_SS_PATH).expect("Failed to load temporary screenshot");
    let mut ss_texture = rl
        .load_texture_from_image(&rl_thread, &img)
        .expect("Failed to create texture");

    let mut image_position = I32Vector::new(0, 0);
    let mut drag_offset = I32Vector::new(0, 0);
    let original_size = I32Vector::new(ss_texture.width, ss_texture.height);

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
        // TODO: Zoom image to cursor location
        let wheel_move = win.get_mouse_wheel_move();
        if wheel_move > 0.0 || win.is_key_down(KeyboardKey::KEY_EQUAL) {
            // 20x inward limit
            if ss_texture.width < original_size.x * 20 {
                ss_texture.width = ss_texture
                    .width
                    .saturating_add((ss_texture.width as f32 * 0.05) as i32);
                ss_texture.height = ss_texture
                    .height
                    .saturating_add((ss_texture.height as f32 * 0.05) as i32);
            }
        } else if wheel_move < 0.0 || win.is_key_down(KeyboardKey::KEY_MINUS) {
            // 2x outward limit
            if ss_texture.height > original_size.y / 2 {
                ss_texture.width = ss_texture
                    .width
                    .saturating_sub((ss_texture.width as f32 * 0.05) as i32);
                ss_texture.height = ss_texture
                    .height
                    .saturating_sub((ss_texture.height as f32 * 0.05) as i32);
            }
        }
        // -------------------------------------------------------------

        // Set image display position
        // TODO: Set reasonable position clamps
        image_position.x = 0 + drag_offset.x;
        image_position.y = 0 + drag_offset.y;

        // TODO: Spotlight effect

        // Show image on screen
        win.draw_texture(
            &ss_texture,
            image_position.x,
            image_position.y,
            Color::RAYWHITE,
        );

        if win.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            todo!("Flashlight effect");
        }
    }
}
