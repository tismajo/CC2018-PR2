use raylib::prelude::*;

mod camera;
mod ray;
mod material;
mod texture;
mod color;
mod scene;
mod cube;
mod light;
mod point_light;
mod skybox;
mod obj_loader;
mod intersection;
mod renderer;
mod utils;

use camera::Camera;
use scene::Scene;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Casa del Leñador - Raytracer")
        .build();

    rl.set_target_fps(60);

    let mut scene = Scene::new();
    scene.build_lumberjack_house_scene();

    let mut camera = Camera::new(
        utils::Vec3::new(0.0, 5.0, 15.0),
        utils::Vec3::new(0.0, 0.0, 0.0),
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

    // Colores del tema azul
    let bg_color = Color::new(20, 25, 45, 255);        // Azul oscuro fondo
    let panel_color = Color::new(30, 40, 70, 220);     // Azul panel
    let accent_color = Color::new(65, 105, 225, 255);  // Azul royal
    let light_blue = Color::new(100, 150, 255, 255);   // Azul claro
    let cyan = Color::new(0, 255, 255, 255);           // Cian para highlights
    let text_color = Color::new(200, 220, 255, 255);   // Texto azul claro

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

        // Toggle modo auto performance
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            auto_quality = !auto_quality;
            if !auto_quality {
                quality_level = manual_quality_level;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) { 
            use_threading = !use_threading; 
        }

        // Control tiempo día/noche
        if rl.is_key_down(KeyboardKey::KEY_N) {
            day_time = (day_time + 0.01) % 1.0;
        }

        // === Ajuste Automático de Calidad ===
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
                    println!("Auto: Bajando calidad (FPS: {})", avg_fps);
                }
                else if avg_fps > HIGH_FPS_THRESHOLD && quality_level > 0 {
                    if quality_level > manual_quality_level {
                        quality_level -= 1;
                        println!("Auto: Subiendo calidad (FPS: {})", avg_fps);
                    }
                }
            }
        }

        scene.update_sun_position(day_time);

        let render_scale = match quality_level {
            0 => 4,  // Baja: 4x downscale
            1 => 2,  // Media: 2x downscale
            _ => 1,  // Alta: Resolución nativa
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
        
        // Fondo azul oscuro
        d.clear_background(bg_color);
        
        // Dibujar buffer de renderizado
        draw_buffer(&mut d, &image_buffer, WIDTH, HEIGHT);

        // === PANEL DE INFORMACIÓN - Estilo azul ===
        let panel_width = 250;
        let panel_height = 180;
        let panel_x = 10;
        let panel_y = 10;
        
        // Panel semi-transparente
        d.draw_rectangle(panel_x, panel_y, panel_width, panel_height, panel_color);
        d.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, accent_color);

        // Título del panel
        d.draw_text("CASA DEL LEÑADOR", panel_x + 10, panel_y + 5, 16, light_blue);

        // Información de rendimiento
        let fps = d.get_fps();
        let fps_color = if fps >= 50 {
            Color::GREEN
        } else if fps >= 25 {
            Color::YELLOW
        } else {
            Color::RED
        };
        
        d.draw_text(&format!("FPS: {}", fps), panel_x + 15, panel_y + 30, 18, fps_color);

        // Información de calidad
        let (quality_text, quality_color) = match quality_level {
            0 => ("BAJA (4x)", Color::ORANGE),
            1 => ("MEDIA (2x)", light_blue),
            _ => ("ALTA (1x)", Color::LIME),
        };
        
        d.draw_text(&format!("CALIDAD: {}", quality_text), panel_x + 15, panel_y + 55, 16, quality_color);

        // Estado auto-calidad
        if auto_quality {
            d.draw_text("MODO AUTO", panel_x + 150, panel_y + 55, 14, cyan);
        }

        // Información de renderizado
        let pixels_rendered = ((WIDTH * HEIGHT) / (render_scale * render_scale)) as f32;
        let percentage = (pixels_rendered / (WIDTH * HEIGHT) as f32) * 100.0;
        d.draw_text(
            &format!("PIXELS: {:.0}%", percentage),
            panel_x + 15, panel_y + 80,
            14,
            text_color,
        );

        // Información adicional
        d.draw_text(&format!("HILOS: {}", if use_threading { "ON" } else { "OFF" }), 
                   panel_x + 15, panel_y + 100, 14, text_color);
        d.draw_text(&format!("HORA: {:.2}", day_time), 
                   panel_x + 15, panel_y + 120, 14, Color::YELLOW);

        // Dirección del sol (debug)
        d.draw_text(&format!("SOL: ({:.1}, {:.1}, {:.1})", 
            -scene.sun.direction.x, -scene.sun.direction.y, -scene.sun.direction.z), 
            panel_x + 15, panel_y + 140, 12, Color::ORANGE);

        // === PANEL DE CONTROLES - Parte inferior ===
        let controls_panel_height = 90;
        let controls_y = HEIGHT - controls_panel_height - 10;
        
        d.draw_rectangle(panel_x, controls_y, panel_width, controls_panel_height, panel_color);
        d.draw_rectangle_lines(panel_x, controls_y, panel_width, controls_panel_height, accent_color);
        
        d.draw_text("=== CONTROLES ===", panel_x + 10, controls_y + 5, 16, light_blue);
        
        // Controles en dos columnas
        d.draw_text("WASD: Mirar", panel_x + 15, controls_y + 25, 14, text_color);
        d.draw_text("Q/E: Subir/Bajar", panel_x + 15, controls_y + 45, 14, text_color);
        d.draw_text("FLECHAS: Rotar/Zoom", panel_x + 120, controls_y + 25, 14, text_color);
        d.draw_text("N: Día/Noche", panel_x + 120, controls_y + 45, 14, text_color);
        
        d.draw_text("1/2/3: Calidad | P: Auto | T: Hilos", 
                   panel_x + 15, controls_y + 65, 12, Color::LIGHTGRAY);

        // === INFORMACIÓN ADICIONAL EN ESQUINA SUPERIOR DERECHA ===
        let info_x = WIDTH - 200;
        d.draw_text("RENDER: RAYTRACING", info_x, 15, 14, cyan);
        d.draw_text("ESCENA: BOSQUE", info_x, 35, 14, text_color);
        d.draw_text("TEXTURAS: 64x64", info_x, 55, 14, text_color);
    }
}

fn handle_camera_input(rl: &RaylibHandle, camera: &mut Camera, delta_time: f32) {
    // Velocidades de control (unidades/grados por segundo)
    let rotation_speed = 60.0;
    let zoom_speed = 10.0;
    let vertical_speed = 5.0;

    // Calcular cantidades basadas en delta_time
    let rotate_amount = rotation_speed * delta_time;
    let zoom_amount = zoom_speed * delta_time;
    let vertical_amount = vertical_speed * delta_time;

    // === WASD - Control de Vista ===
    if rl.is_key_down(KeyboardKey::KEY_W) {
        camera.rotate_vertical(rotate_amount); // Mirar ARRIBA
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        camera.rotate_vertical(-rotate_amount); // Mirar ABAJO
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        camera.rotate_around_target(-rotate_amount); // Mirar IZQUIERDA
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        camera.rotate_around_target(rotate_amount); // Mirar DERECHA
    }

    // === Flechas - Rotación y Zoom ===
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        camera.rotate_around_target(-rotate_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        camera.rotate_around_target(rotate_amount);
    }

    // === Flechas - Zoom ===
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        camera.zoom(-zoom_amount); // Zoom IN
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        camera.zoom(zoom_amount); // Zoom OUT
    }

    // === Q/E - Mover Cámara Arriba/Abajo ===
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
