mod library;
mod zooma_error;

use library::*;

use raylib::prelude::*;

// General TODO:
// - Delete temp screenshot once loaded

fn main() {
    set_temp_ss_path();

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

    // Load texture from temporary screenshot
    // TODO: Show formatted errors instead of panicking with .expect()
    let img = Image::load_image(&TMP_SS_PATH.get().unwrap().to_string_lossy())
        .expect("Failed to load temporary screenshot");
    let mut ss_texture = rl
        .load_texture_from_image(&rl_thread, &img)
        .expect("Failed to create texture");

    // Set positions & offsets for temp screenshot
    let mut image_position = I32Vector::new(0, 0);
    let mut drag_offset = I32Vector::new(0, 0);
    let original_size = I32Vector::new(ss_texture.width, ss_texture.height);

    let render_size = get_render_size(&rl);
    let mut overlay_tex = rl
        .load_render_texture(&rl_thread, render_size.0 as u32, render_size.1 as u32)
        .expect("Failed to create overlay texture");

    let mut circle_size: f32 = render_size.1 as f32 / 12.0;
    while !rl.window_should_close() {
        let mouse_pos = (rl.get_mouse_x(), rl.get_mouse_y());
        // Create overlay texture
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

        // Avoid calling is_key_down multiple times
        let ctrl_key_down = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);

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
        if wheel_move > 0.0 && !ctrl_key_down {
            // 20x inward limit
            if ss_texture.width < original_size.x * 20 {
                ss_texture.width = ss_texture
                    .width
                    .saturating_add((ss_texture.width as f32 * 0.05) as i32);
                ss_texture.height = ss_texture
                    .height
                    .saturating_add((ss_texture.height as f32 * 0.05) as i32);
            }
        } else if wheel_move < 0.0 && !ctrl_key_down {
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

        // Reset image if R is pressed
        if win.is_key_released(KeyboardKey::KEY_R) {
            ss_texture.width = original_size.x;
            ss_texture.height = original_size.y;
            drag_offset = I32Vector { x: 0, y: 0 };
            image_position = I32Vector { x: 0, y: 0 };
        }

        // Show image on screen
        win.draw_texture(
            &ss_texture,
            image_position.x,
            image_position.y,
            Color::RAYWHITE,
        );

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

            // TODO: Set clamps
            if win.get_mouse_wheel_move() > 0.0 {
                circle_size -= 10.0;
            } else if win.get_mouse_wheel_move() < 0.0 {
                circle_size += 10.0;
            }
        }
    }
}

fn get_render_size(w: &RaylibHandle) -> (i32, i32) {
    return (w.get_render_width(), w.get_render_height());
}
