use std::{env::temp_dir, fs::remove_file, path::PathBuf, process};
use raylib::prelude::*;

use zooma::{self::*, zooma_error::ZoomaError};

// TODO: Smooth zooming
fn main() {
    // Determine temporary screenshot path
    let ss_path: PathBuf = PathBuf::from(temp_dir().join("zooma.png"));

    match take_screenshot(&ss_path) {
        Err(e) => handle_zooma_error(e),
        Ok(()) => (), // Success
    }

    // Initialize Raylib
    let (mut rl, rl_thread) = raylib::init()
        .title("Zooma")
        .resizable()
        .fullscreen()
        .vsync()
        .build();
    rl.set_target_fps(60);

    // Load texture from temporary screenshot
    let img = match Image::load_image(&ss_path.to_string_lossy()) {
        Err(e) => {
            println!("Failed to load temporary screenshot: {:?}", e);
            process::exit(1);
        }
        Ok(o) => o,
    };
    let mut ss_texture = match rl.load_texture_from_image(&rl_thread, &img) {
        Err(e) => {
            println!("Failed to create texture: {:?}", e);
            process::exit(1);
        }
        Ok(o) => o,
    };
    match remove_file(ss_path) {
        Err(e) => {
            println!("Failed to remove temporary screenshot file {:?}", e);
            process::exit(1);
        }
        Ok(_) => (),
    }

    // Set positions & offsets for image
    let original_size = I32Vector2::new(ss_texture.width, ss_texture.height);
    let mut img_origin: I32Vector2;
    let mut new_origin = I32Vector2::default();
    let mut drag_offset = I32Vector2::default();

    let render_size: (u32, u32) = (rl.get_render_width() as u32, rl.get_render_height() as u32);

    let mut overlay_tex = rl
        .load_render_texture(&rl_thread, render_size.0, render_size.1)
        .expect("Failed to create overlay texture");

    let original_circle_size: f32 = render_size.1 as f32 / 12.0;
    let mut circle_size = original_circle_size;

    // TODO: Draw some sort of indicator that the program is active
    while !rl.window_should_close() {
        let mouse_pos: (i32, i32) = (rl.get_mouse_x(), rl.get_mouse_y());

        // --- Create overlay texture --------------------------------------------
        {
            let mut dt = rl.begin_texture_mode(&rl_thread, &mut overlay_tex);

            // Set a black background
            dt.clear_background(Color::BLACK);

            // Cut circle from the black background
            {
                let mut b = dt.begin_blend_mode(BlendMode::BLEND_SUBTRACT_COLORS);
                b.draw_circle(mouse_pos.0, mouse_pos.1, circle_size, Color::WHITE);
            }
        }
        // -----------------------------------------------------------------------

        // Avoid calling is_key_down multiple times
        let ctrl_key_down = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        let shift_key_down = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);

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
        if wheel_move > 0.0 && !shift_key_down {
            // 20x inward limit
            if ss_texture.width < original_size.x * 20 {
                ss_texture.width = ss_texture
                    .width
                    .saturating_add((ss_texture.width as f32 * 0.05) as i32);
                ss_texture.height = ss_texture
                    .height
                    .saturating_add((ss_texture.height as f32 * 0.05) as i32);
            }
        } else if wheel_move < 0.0 && !shift_key_down {
            // 1.2x outward limit
            if ss_texture.height > (original_size.y as f32 / 1.2) as i32 {
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
        new_origin.x = 0 + drag_offset.x;
        new_origin.y = 0 + drag_offset.y;

        // Reset image if R is pressed
        if win.is_key_released(KeyboardKey::KEY_R) {
            ss_texture.width = original_size.x;
            ss_texture.height = original_size.y;

            circle_size = original_circle_size;

            drag_offset.reset();
            new_origin.reset();
        }

        // Overwrite old origin with the new one
        img_origin = new_origin;

        // Show image on screen
        win.draw_texture(&ss_texture, img_origin.x, img_origin.y, Color::RAYWHITE);

        // Spotlight effect
        if ctrl_key_down {
            win.draw_texture_pro(
                &overlay_tex,
                Rectangle::new(
                    0.0,
                    0.0,
                    overlay_tex.width() as f32,
                    -overlay_tex.height() as f32,
                ),
                Rectangle::new(
                    0.0,
                    0.0,
                    overlay_tex.width() as f32,
                    overlay_tex.height() as f32,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE.alpha(0.7),
            );
        }

        if shift_key_down {
            // TODO: Set clamps
            if win.get_mouse_wheel_move() > 0.0 {
                circle_size -= 10.0;
            } else if win.get_mouse_wheel_move() < 0.0 {
                circle_size += 10.0;
            }
        }
    }
}

fn handle_zooma_error(ze: ZoomaError) -> ! {
    match ze {
        ZoomaError::MissingXdgSessionType => {
            println!("Failed to read $XDG_SESSION_TYPE environment variable");
            process::exit(1);
        }
        ZoomaError::InvalidXdgSessionType(session_value) => {
            println!(
                "Invalid $XDG_SESSION_TYPE, expected \
                \"x11\" or \"wayland\", got \"{:}\"",
                session_value
            );
            process::exit(1);
        }
        ZoomaError::MissingDependency(dep) => {
            println!("Missing dependency: \'{:}\'", dep);
            process::exit(1);
        }
        ZoomaError::UnsupportedEnvironment => {
            println!("The running environment is currently unsupported");
            process::exit(1);
        }
        ZoomaError::MissingXdgCurrentDesktop => {
            println!(
                "Failed to read $XDG_CURRENT_DESKTOP, please \
                make sure it is set correctly"
            );
            process::exit(1);
        }
    }
}
