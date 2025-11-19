use crate::obj_loader::Mesh;
use crate::material::Material;
use crate::color::Color;
use crate::utils::Vec3;
use crate::ray::Ray;
use crate::intersection::Intersection;

pub struct SunMoonSystem {
    pub sun_mesh: Mesh,
    pub moon_mesh: Mesh,
    pub sun_material: Material,
    pub moon_material: Material,
    pub sun_position: Vec3,
    pub moon_position: Vec3,
    pub sun_radius: f32,
    pub moon_radius: f32,
}

impl SunMoonSystem {
    pub fn new() -> Self {
        // Material del sol (emissivo y brillante)
        let sun_mat = Material::new(Color::new(1.0, 0.9, 0.1))
            .with_emissive(Color::new(1.0, 0.8, 0.2))
            .with_specular(1.0, 256.0);

        // Material de la luna (suave y menos brillante)
        let moon_mat = Material::new(Color::new(0.9, 0.9, 0.95))
            .with_emissive(Color::new(0.3, 0.3, 0.4))
            .with_specular(0.3, 64.0);

        // Cargar el modelo de esfera para ambos
        let sun_mesh = Mesh::load_obj(
            "sphere-1.obj",
            Vec3::new(0.0, 0.0, 0.0),
            1.0,
            sun_mat.clone(),
        );

        let moon_mesh = Mesh::load_obj(
            "sphere-1.obj", 
            Vec3::new(0.0, 0.0, 0.0),
            0.8, // Luna ligeramente más pequeña
            moon_mat.clone(),
        );

        Self {
            sun_mesh,
            moon_mesh,
            sun_material: sun_mat,
            moon_material: moon_mat,
            sun_position: Vec3::new(0.0, 0.0, 0.0),
            moon_position: Vec3::new(0.0, 0.0, 0.0),
            sun_radius: 50.0,  // Radio de la órbita del sol
            moon_radius: 45.0, // Radio de la órbita de la luna
        }
    }

    pub fn update_positions(&mut self, day_time: f32) {
        // day_time va de 0.0 a 1.0, donde:
        // 0.0 = amanecer, 0.25 = mediodía, 0.5 = atardecer, 0.75 = medianoche, 1.0 = amanecer otra vez
        
        let sun_angle = day_time * 2.0 * std::f32::consts::PI;
        let moon_angle = sun_angle + std::f32::consts::PI; // Luna opuesta al sol

        // Órbita del sol (más alta durante el día)
        let sun_height = if day_time < 0.5 {
            // Día: sol más alto en el cielo
            (sun_angle.sin() * 0.5 + 0.5).max(0.3)
        } else {
            // Noche: sol bajo el horizonte
            -0.5
        };

        // Órbita de la luna (más alta durante la noche)
        let moon_height = if day_time > 0.5 {
            // Noche: luna más alta en el cielo
            (moon_angle.sin() * 0.5 + 0.5).max(0.3)
        } else {
            // Día: luna bajo el horizonte
            -0.5
        };

        // Posiciones en órbita circular
        self.sun_position = Vec3::new(
            self.sun_radius * sun_angle.cos(),
            sun_height * 30.0, // Altura escalada
            self.sun_radius * sun_angle.sin(),
        );

        self.moon_position = Vec3::new(
            self.moon_radius * moon_angle.cos(),
            moon_height * 30.0, // Altura escalada
            self.moon_radius * moon_angle.sin(),
        );

        // Actualizar posiciones de los meshes
        self.sun_mesh.position = self.sun_position;
        self.moon_mesh.position = self.moon_position;

        // Ajustar intensidad basada en la posición
        let sun_intensity = self.calculate_sun_intensity(day_time);
        let moon_intensity = self.calculate_moon_intensity(day_time);

        self.sun_material.emissive = Color::new(1.0, 0.8, 0.2) * sun_intensity;
        self.moon_material.emissive = Color::new(0.3, 0.3, 0.4) * moon_intensity;
    }

    fn calculate_sun_intensity(&self, day_time: f32) -> f32 {
        // Intensidad máxima al mediodía (0.25), mínima en la noche
        let normalized_time = (day_time * 4.0) % 1.0; // 0-1 durante el día
        if day_time < 0.25 || day_time > 0.75 {
            // Amanecer/atardecer
            (1.0 - (normalized_time * 2.0 - 1.0).abs()).powf(2.0) * 0.8
        } else if day_time < 0.5 {
            // Día
            1.0
        } else {
            // Noche
            0.0
        }
    }

    fn calculate_moon_intensity(&self, day_time: f32) -> f32 {
        // Intensidad máxima a medianoche (0.75), mínima durante el día
        let normalized_time = ((day_time + 0.5) * 4.0) % 1.0; // Desplazado 12 horas
        if day_time > 0.625 && day_time < 0.875 {
            // Noche completa
            1.0
        } else if (day_time > 0.5 && day_time <= 0.625) || (day_time >= 0.875 && day_time < 1.0) {
            // Anochecer/amanecer lunar
            (1.0 - (normalized_time * 2.0 - 1.0).abs()).powf(2.0) * 0.6
        } else {
            // Día
            0.0
        }
    }

    pub fn get_sun_direction(&self) -> Vec3 {
        // Dirección desde el origen hacia el sol (para iluminación)
        -self.sun_position.normalize()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // Verificar intersección con el sol
        if let Some(mut intersection) = self.sun_mesh.intersect(ray) {
            // El sol siempre es visible si está sobre el horizonte
            if self.sun_position.y > -5.0 {
                return Some(intersection);
            }
        }

        // Verificar intersección con la luna
        if let Some(mut intersection) = self.moon_mesh.intersect(ray) {
            // La luna siempre es visible si está sobre el horizonte
            if self.moon_position.y > -5.0 {
                return Some(intersection);
            }
        }

        None
    }
}

impl Default for SunMoonSystem {
    fn default() -> Self {
        Self::new()
    }
}
