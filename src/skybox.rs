use crate::color::Color;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::mate::Vec3;

pub struct Skybox {
    // Ya no necesitamos las texturas de imagen
    // En su lugar, generaremos colores proceduralmente
}

impl Skybox {
    pub fn new() -> Self {
        Self {
            // No necesitamos inicializar texturas
        }
    }

    /// Sample the skybox based on ray direction and time of day
    pub fn sample(&self, ray: &Ray, day_time: f32, sun_dir: Vec3, _sun_color: Color, _sun_intensity: f32) -> Color {
        let direction = ray.direction.normalize();
        
        // === FONDO BÁSICO DÍA/NOCHE ===
        let base_color = if day_time < 0.5 {
            // DÍA: Azul cielo
            self.sample_day_sky(&direction)
        } else {
            // NOCHE: Púrpura oscuro
            self.sample_night_sky(&direction)
        };

        // === SOL Y LUNA VISIBLES ===
        let sun_dir = sun_dir.normalize();
        let cos_angle_to_sun = direction.dot(&sun_dir).max(-1.0).min(1.0);
        
        // Luna está en dirección opuesta al sol
        let moon_dir = -sun_dir;
        let cos_angle_to_moon = direction.dot(&moon_dir).max(-1.0).min(1.0);

        let mut final_color = base_color;

        // SOL - Solo visible durante el día
        let sun_radius_cos = (5.0f32.to_radians()).cos();
        if day_time < 0.5 && cos_angle_to_sun >= sun_radius_cos {
            let t = (cos_angle_to_sun - sun_radius_cos) / (1.0 - sun_radius_cos);
            let brightness = t.powf(0.3) * (1.0 - day_time * 2.0);
            let sun_color = Color::new(1.0, 1.0, 0.9) * (3.0 * brightness);
            final_color = final_color + sun_color;
        }

        // LUNA - Solo visible durante la noche
        let moon_radius_cos = (3.0f32.to_radians()).cos();
        if day_time > 0.5 && cos_angle_to_moon >= moon_radius_cos {
            let t = (cos_angle_to_moon - moon_radius_cos) / (1.0 - moon_radius_cos);
            let brightness = t.powf(0.5) * (day_time - 0.5) * 2.0;
            let moon_color = Color::new(0.9, 0.9, 1.0) * (1.5 * brightness);
            final_color = final_color + moon_color;
        }

        final_color.clamp()
    }

    /// Genera un cielo diurno azul
    fn sample_day_sky(&self, direction: &Vec3) -> Color {
        // Base: azul cielo
        let base_blue = Color::new(0.4, 0.6, 0.95);
        
        // Horizonte más claro
        let horizon_color = Color::new(0.7, 0.8, 1.0);
        
        // Gradiente vertical: más azul arriba, más claro en el horizonte
        let height_factor = direction.y.max(0.0); // 0 en horizonte, 1 arriba
        
        // Mezcla entre horizonte y cielo
        let r = horizon_color.r + (base_blue.r - horizon_color.r) * height_factor;
        let g = horizon_color.g + (base_blue.g - horizon_color.g) * height_factor;
        let b = horizon_color.b + (base_blue.b - horizon_color.b) * height_factor;
        
        Color::new(r, g, b)
    }

    /// Genera un cielo nocturno púrpura oscuro
    fn sample_night_sky(&self, direction: &Vec3) -> Color {
        // Base: púrpura oscuro
        let base_purple = Color::new(0.08, 0.03, 0.15);
        
        // Horizonte ligeramente más claro
        let horizon_color = Color::new(0.12, 0.05, 0.2);
        
        // Gradiente vertical
        let height_factor = direction.y.max(0.0);
        
        // Mezcla entre horizonte y cielo nocturno
        let r = horizon_color.r + (base_purple.r - horizon_color.r) * height_factor;
        let g = horizon_color.g + (base_purple.g - horizon_color.g) * height_factor;
        let b = horizon_color.b + (base_purple.b - horizon_color.b) * height_factor;
        
        let mut color = Color::new(r, g, b);
        
        // Añadir algunas estrellas (solo en la parte superior del cielo)
        if height_factor > 0.3 {
            // Generar "estrellas" basadas en la dirección (pseudo-aleatorio)
            let star_noise = (direction.x * 12345.0 + direction.y * 67890.0 + direction.z * 13579.0).sin().abs();
            if star_noise > 0.995 {
                let brightness = (star_noise - 0.995) / 0.005;
                color = color + Color::new(0.8, 0.8, 1.0) * brightness;
            }
        }
        
        color
    }
}

impl Default for Skybox {
    fn default() -> Self {
        Self::new()
    }
}
