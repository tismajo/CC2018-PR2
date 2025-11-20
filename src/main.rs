use raylib::prelude::*;

mod camara;
mod ray;
mod material;
mod texture;
mod color;
mod minecraft;
mod cubo;
mod luz;
mod fuente_luz;
mod skybox;
mod mesh;
mod intersection;
mod renderer;
mod mate;

use camara::Camera;
use minecraft::Scene;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Farmeador de experiencia MAICRA")
        .build();

    rl.set_target_fps(60);

    let mut scene = Scene::new();
    scene.build_lumberjack_house_scene();

    let mut camera = Camera::new(
        mate::Vec3::new(0.0, 5.0, 15.0),
        mate::Vec3::new(0.0, 0.0, 0.0),
        70.0,
        WIDTH as f32 / HEIGHT as f32,
    );

    let mut quality_level = 1;
    let mut manual_quality_level = 1;
    let mut use_threading = true;
    let mut day_time = 0.0f32;
    let mut auto_quality = false;

    // FPS tracking para auto quality
    let mut fps_history: Vec<u32> = Vec::new();
    let mut fps_check_timer = 0.0f32;
    const FPS_CHECK_INTERVAL: f32 = 0.5;
    const LOW_FPS_THRESHOLD: u32 = 20;
    const HIGH_FPS_THRESHOLD: u32 = 45;

    let mut image_buffer = vec![Color::BLACK; (WIDTH * HEIGHT) as usize];

    // === TEMA AZUL MEJORADO ===
    let bg_color       = Color::new(15, 20, 35, 255);     // Fondo azul muy oscuro
    let panel_color    = Color::new(25, 35, 60, 180);     // Panel semitransparente
    let panel_border   = Color::new(90, 130, 255, 220);   // Bordes azul brillante
    let title_color    = Color::new(120, 160, 255, 255);  // Azul claro para títulos
    let text_color     = Color::new(180, 210, 255, 255);  // Texto azul claro
    let cyan           = Color::new(0, 220, 255, 255);    // Highlights
    let light_blue     = Color::new(100, 150, 255, 255);  // Azul para labels

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        let current_fps = rl.get_fps();

        handle_camera_input(&rl, &mut camera, delta_time);

        // === Control de Calidad ===
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            manual_quality_level = 0;
            if !auto_quality { quality_level = 0; }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            manual_quality_level = 1;
            if !auto_quality { quality_level = 1; }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            manual_quality_level = 2;
            if !auto_quality { quality_level = 2; }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            auto_quality = !auto_quality;
            if !auto_quality { quality_level = manual_quality_level; }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) { 
            use_threading = !use_threading; 
        }

        if rl.is_key_down(KeyboardKey::KEY_N) {
            day_time = (day_time + 0.01) % 1.0;
        }

        // === Auto Calidad ===
        if auto_quality {
            fps_check_timer += delta_time;
            fps_history.push(current_fps);

            if fps_history.len() > 10 {
                fps_history.remove(0);
            }

            if fps_check_timer >= FPS_CHECK_INTERVAL && fps_history.len() >= 5 {
                fps_check_timer = 0.0;

                let avg_fps: u32 = fps_history.iter().sum::<u32>() / fps_history.len() as u32;

                if avg_fps < LOW_FPS_THRESHOLD && quality_level < 2 {
                    quality_level += 1;
                } else if avg_fps > HIGH_FPS_THRESHOLD && quality_level > 0 {
                    if quality_level > manual_quality_level {
                        quality_level -= 1;
                    }
                }
            }
        }

        scene.update_sun_position(day_time);

        let render_scale = match quality_level {
            0 => 4,
            1 => 2,
            _ => 1,
        };

        renderer::render_scene(
            &scene,
            &camera,
            &mut image_buffer,
            WIDTH,
            HEIGHT,
            render_scale,
            use_threading,
            day_time,
        );

        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(bg_color);
        draw_buffer(&mut d, &image_buffer, WIDTH, HEIGHT);

        // === PANEL DE INFORMACIÓN ===
        let panel_x = 10;
        let panel_y = 10;
        let panel_width = 250;
        let panel_height = 180;

        d.draw_rectangle(panel_x, panel_y, panel_width, panel_height, panel_color);
        d.draw_rectangle_lines_ex(
            Rectangle::new(panel_x as f32, panel_y as f32, panel_width as f32, panel_height as f32),
            2.0,
            panel_border
        );

        d.draw_text("Farming de experiencia", panel_x + 10, panel_y + 5, 16, title_color);

        let fps = d.get_fps();
        let fps_color = if fps >= 50 {
            Color::GREEN
        } else if fps >= 25 {
            Color::YELLOW
        } else {
            Color::RED
        };

        d.draw_text(&format!("FPS: {}", fps), panel_x + 15, panel_y + 30, 18, text_color);

        let (quality_text, quality_color) = match quality_level {
            0 => ("BAJA (4x)", Color::ORANGE),
            1 => ("MEDIA (2x)", light_blue),
            _ => ("ALTA (1x)", Color::LIME),
        };

        d.draw_text(
            &format!("CALIDAD: {}", quality_text),
            panel_x + 15,
            panel_y + 55,
            16,
            quality_color
        );

        let pixels_rendered = ((WIDTH * HEIGHT) / (render_scale * render_scale)) as f32;
        let percentage = (pixels_rendered / (WIDTH * HEIGHT) as f32) * 100.0;

        d.draw_text(&format!("PIXELS: {:.0}%", percentage),
            panel_x + 15, panel_y + 80, 14, text_color);

        d.draw_text(&format!("HILOS: {}", if use_threading { "ON" } else { "OFF" }),
            panel_x + 15, panel_y + 100, 14, text_color);

        d.draw_text(&format!("HORA: {:.2}", day_time),
            panel_x + 15, panel_y + 120, 14, text_color);
            
        // === PANEL CONTROLES ===
        let controls_panel_height = 90;
        let controls_y = HEIGHT - controls_panel_height - 10;

        d.draw_rectangle(panel_x, controls_y, panel_width, controls_panel_height, panel_color);
        d.draw_rectangle_lines_ex(
            Rectangle::new(panel_x as f32, controls_y as f32, panel_width as f32, controls_panel_height as f32),
            2.0,
            panel_border
        );
    }
}

fn handle_camera_input(rl: &RaylibHandle, camera: &mut Camera, delta_time: f32) {
    let rotation_speed = 60.0;
    let zoom_speed = 10.0;
    let vertical_speed = 5.0;

    let rotate_amount = rotation_speed * delta_time;
    let zoom_amount = zoom_speed * delta_time;
    let vertical_amount = vertical_speed * delta_time;

    if rl.is_key_down(KeyboardKey::KEY_W) {
        camera.rotate_vertical(rotate_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        camera.rotate_vertical(-rotate_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        camera.rotate_around_target(-rotate_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        camera.rotate_around_target(rotate_amount);
    }

    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        camera.rotate_around_target(-rotate_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        camera.rotate_around_target(rotate_amount);
    }

    if rl.is_key_down(KeyboardKey::KEY_UP) {
        camera.zoom(-zoom_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        camera.zoom(zoom_amount);
    }

    if rl.is_key_down(KeyboardKey::KEY_Q) {
        camera.move_up(vertical_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_E) {
        camera.move_down(vertical_amount);
    }
}

fn draw_buffer(d: &mut RaylibDrawHandle, buffer: &[Color], width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            d.draw_pixel(x, y, buffer[idx]);
        }
    }
}
