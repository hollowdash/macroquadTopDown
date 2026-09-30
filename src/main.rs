// #![windows_subsystem = "windows"]
use macroquad::prelude::*;

#[macroquad::main("top down movement thing")]
async fn main() {
    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = 0.0;
    let mut x_vel: f32 = 0.0;
    let mut y_vel: f32 = 0.0;

    let mut cam_x: f32 = x_pos;
    let mut cam_y: f32 = y_pos;
    const CAMERA_SMOOTHING: f32 = 6.0;


    // basically i fucked up a bit and forgot to multiply line 84 and 85 by dt so frame_mult is just gonna be here so i can have the same sense of sensitivity lol
    const FRAME_MULT: f32 = 180.0;
    const ACCEL: f32 = 25.0 * FRAME_MULT;
    const FRICTION: f32 = 15.0 * FRAME_MULT;
    const TOPSPEED: f32 = 2.5 * FRAME_MULT;

    let mut smoothed_fps: f32 = 60.0;
    let mut fps_timer: f32 = 0.0;
    let mut fps_display: String = String::new();

    let mut debug_menu: bool = false;
    let tlightgray = Color { a: 0.5, ..LIGHTGRAY };


    let test_objects = vec![
        (100.0, 100.0, 50.0, 50.0, RED),
        (500.0, -200.0, 80.0, 120.0, GREEN),
        (-300.0, 400.0, 200.0, 40.0, DARKGRAY),
        (800.0, 600.0, 60.0, 60.0, PURPLE),
    ];

    loop {
        let dt: f32 = get_frame_time();

        if dt > 0.0 {
            smoothed_fps = smoothed_fps * 0.95 + (1.0 / dt) * 0.05;
        }

        fps_timer += dt;
        if fps_timer >= 0.2 {
            fps_display = format!("{:.0}", smoothed_fps);
            fps_timer = 0.0;
        }

        // x-axis movement
        let mut move_x = 0.0;
        if is_key_down(KeyCode::Right) { move_x += 1.0; }
        if is_key_down(KeyCode::Left)  { move_x -= 1.0; }

        if move_x != 0.0 {
            let is_turning_around = (x_vel > 0.0 && move_x < 0.0) || (x_vel < 0.0 && move_x > 0.0);
            if is_turning_around { x_vel = 0.0; }
            x_vel += move_x * (ACCEL * dt);
        } else {
            if x_vel > 0.0 { x_vel = (x_vel - FRICTION * dt).max(0.0); }
            else if x_vel < 0.0 { x_vel = (x_vel + FRICTION * dt).min(0.0); }
        }

        // y-axis movement
        let mut move_y = 0.0;
        if is_key_down(KeyCode::Up)   { move_y += 1.0; } // Swapped to invert direction
        if is_key_down(KeyCode::Down) { move_y -= 1.0; } // Swapped to invert direction

        if move_y != 0.0 {
            let is_turning_around = (y_vel > 0.0 && move_y < 0.0) || (y_vel < 0.0 && move_y > 0.0);
            if is_turning_around { y_vel = 0.0; }
            y_vel += move_y * (ACCEL * dt);
        } else {
            if y_vel > 0.0 { y_vel = (y_vel - FRICTION * dt).max(0.0); }
            else if y_vel < 0.0 { y_vel = (y_vel + FRICTION * dt).min(0.0); }
        }

        // fix diagonal speed bc that was dumb lol
        let current_speed = (x_vel * x_vel + y_vel * y_vel).sqrt();
        if current_speed > TOPSPEED {
            x_vel = (x_vel / current_speed) * TOPSPEED;
            y_vel = (y_vel / current_speed) * TOPSPEED;
        }

        x_pos += x_vel * dt;
        y_pos += y_vel * dt;

       // camera lerp
        cam_x += (x_pos - cam_x) * (CAMERA_SMOOTHING * dt);
        cam_y += (y_pos - cam_y) * (CAMERA_SMOOTHING * dt);

        clear_background(WHITE);

        let camera = Camera2D::from_display_rect(Rect::new(
            cam_x as f32 - screen_width() / 2.0,
            cam_y as f32 - screen_height() / 2.0,
            screen_width(),
            screen_height(),
        ));
        
        set_camera(&camera);

        for &(ox, oy, w, h, color) in &test_objects {
            draw_rectangle(ox, oy, w, h, color);
        }

        draw_circle(x_pos as f32, y_pos as f32, 25.0, BLUE);

        set_default_camera();

        if is_key_pressed(KeyCode::Home){
            debug_menu = !debug_menu;
        }

        if debug_menu {
            draw_rounded_rectangle(15.0, 15.0, 275.0, 230.0, 15.0, tlightgray);
            draw_text(&format!("FPS: {}", fps_display), 35.0, 45.0, 30.0, WHITE);
            draw_text(&format!("X Vel: {:.2}", x_vel), 35.0, 75.0, 30.0, WHITE);
            draw_text(&format!("Y Vel: {:.2}", y_vel), 35.0, 105.0, 30.0, WHITE);
            draw_text(&format!("X Pos: {:.2}", x_pos), 35.0, 135.0, 30.0, WHITE);
            draw_text(&format!("Y Pos: {:.2}", y_pos), 35.0, 165.0, 30.0, WHITE);
            draw_text(&format!("CAMX Pos: {:.2}", cam_x), 35.0, 195.0, 30.0, WHITE);
            draw_text(&format!("CAMY Pos: {:.2}", cam_y), 35.0, 225.0, 30.0, WHITE);
        }

        next_frame().await;
    }
}

fn draw_rounded_rectangle(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
    let max_radius = w.min(h) / 2.0;
    let r = radius.min(max_radius);

    let color_bytes = [
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        (color.a * 255.0) as u8,
    ];
    
    let normal = Vec4::new(0.0, 0.0, -1.0, 0.0); 

    // plus shape
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, r, h - 2.0 * r, color);
    draw_rectangle(x + w - r, y + r, r, h - 2.0 * r, color);

    let segments = 8; //smoothness of corners
    let centers = [
        (x + r, y + r, 180.0),         // top left
        (x + w - r, y + r, 270.0),     // top right
        (x + r, y + h - r, 90.0),      // bottom left
        (x + w - r, y + h - r, 0.0),   // bottom right
    ];

    for &(cx, cy, start_deg) in &centers {
        let mut vertices = Vec::with_capacity(segments + 2);
        let mut indices = Vec::with_capacity(segments * 3);

        vertices.push(Vertex {
            position: Vec3::new(cx, cy, 0.0),
            uv: Vec2::ZERO,
            color: color_bytes,
            normal,
        });

        for i in 0..=segments {
            let angle = (start_deg + (i as f32 / segments as f32) * 90.0).to_radians();
            let vx = cx + angle.cos() * r;
            let vy = cy + angle.sin() * r;
            vertices.push(Vertex {
                position: Vec3::new(vx, vy, 0.0),
                uv: Vec2::ZERO,
                color: color_bytes,
                normal,
            });
        }

        for i in 1..=segments {
            indices.push(0);
            indices.push(i as u16);
            indices.push((i + 1) as u16);
        }

        draw_mesh(&Mesh {
            vertices,
            indices,
            texture: None,
        });
    }
}
