use crate::color::Color;
use crate::cubo::Cube;
use crate::intersection::Intersection;
use crate::luz::DirectionalLight;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::fuente_luz::PointLight;
use crate::ray::Ray;
use crate::skybox::Skybox;
use crate::texture::Texture;
use crate::mate::Vec3;

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub meshes: Vec<Mesh>,
    pub sun: DirectionalLight,
    pub point_lights: Vec<PointLight>,
    pub skybox: Skybox,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cubes: Vec::new(),
            meshes: Vec::new(),
            sun: DirectionalLight::sun(Vec3::new(-1.0, -1.0, -0.5).normalize(), 1.2),
            point_lights: Vec::new(),
            skybox: Skybox::new(),
        }
    }

    pub fn build_lumberjack_house_scene(&mut self) {
        // === SUELO DE PASTO ===
        let grass_top = Material::new(Color::new(0.3, 0.7, 0.3))
            .with_texture(Texture::load("assets/pasto.png"));
        let grass_side = Material::new(Color::new(0.5, 0.6, 0.4))
            .with_texture(Texture::load("assets/pasto.png"));
        let dirt_bottom = Material::new(Color::new(0.4, 0.3, 0.2))
            .with_texture(Texture::load("assets/pasto.png"));

        // Crear plano de pasto más grande
        for x in -15..15 {
            for z in -15..15 {
                self.cubes.push(Cube::new_multi_texture(
                    Vec3::new(x as f32, -0.5, z as f32),
                    1.0,
                    grass_top.clone(),
                    grass_side.clone(),
                    dirt_bottom.clone(),
                ));
            }
        }

        // === CASA DEL LEÑADOR ===
        self.build_lumberjack_house();

        // === PILA DE TRONCOS AL LADO DE LA CASA ===
        self.build_wood_pile();

        // === ÁRBOLES ALREDEDOR ===
        self.build_surrounding_trees();

        // === CAMINO DE PIEDRA ===
        self.build_stone_path();
    }

    fn build_lumberjack_house(&mut self) {
        // Materiales para la casa
        let wall_mat = Material::new(Color::new(0.7, 0.5, 0.3))
            .with_texture(Texture::load("assets/pared.png"))
            .with_specular(0.1, 16.0);

        let roof_mat = Material::new(Color::new(0.5, 0.5, 0.5))
            .with_texture(Texture::load("assets/piedra.png"))
            .with_specular(0.3, 32.0);

        let wood_mat = Material::new(Color::new(0.4, 0.3, 0.2))
            .with_texture(Texture::load("assets/tronco.png"))
            .with_specular(0.2, 24.0);

        let window_mat = Material::new(Color::new(0.8, 0.9, 1.0))
            .with_transparency(0.7, 1.5)
            .with_reflectivity(0.1)
            .with_specular(0.8, 64.0);

        // Posición y tamaño de la casa
        let house_x = 0.0;
        let house_z = 0.0;
        let house_width = 7;
        let house_depth = 9;
        let house_height = 5;

        // CIMENTACIÓN DE PIEDRA
        for x in 0..house_width {
            for z in 0..house_depth {
                self.cubes.push(Cube::new(
                    Vec3::new(house_x + x as f32, 0.0, house_z + z as f32),
                    1.0,
                    roof_mat.clone(),
                ));
            }
        }

        // PAREDES DE MADERA
        for y in 1..house_height {
            let y_pos = y as f32;

            // Pared frontal (z = house_z)
            for x in 0..house_width {
                let x_pos = house_x + x as f32;
                // Dejar espacio para la puerta
                if !(y < 3 && x >= 2 && x <= 4) {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }

            // Pared trasera (z = house_z + depth)
            for x in 0..house_width {
                let x_pos = house_x + x as f32;
                // Ventana en la pared trasera
                let is_window = y >= 2 && y <= 3 && (x == 2 || x == 4);
                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z + house_depth as f32 - 1.0),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z + house_depth as f32 - 1.0),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }

            // Pared izquierda (x = house_x)
            for z in 1..(house_depth - 1) {
                let z_pos = house_z + z as f32;
                // Ventana en la pared izquierda
                let is_window = y >= 2 && y <= 3 && z == 4;
                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x, y_pos, z_pos),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x, y_pos, z_pos),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }

            // Pared derecha (x = house_x + width)
            for z in 1..(house_depth - 1) {
                let z_pos = house_z + z as f32;
                // Ventana en la pared derecha
                let is_window = y >= 2 && y <= 3 && z == 4;
                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x + house_width as f32 - 1.0, y_pos, z_pos),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x + house_width as f32 - 1.0, y_pos, z_pos),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }
        }

        // TECHO INCLINADO DE PIEDRA
        let roof_height = 3;
        for roof_level in 0..roof_height {
            let y_pos = house_height as f32 + roof_level as f32;
            let overhang = roof_level as i32;
            
            for x in -overhang..(house_width + overhang) {
                for z in -overhang..(house_depth + overhang) {
                    if x >= 0 && x < house_width && z >= 0 && z < house_depth {
                        continue; // Saltar el área interior
                    }
                    
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x + x as f32, y_pos, house_z + z as f32),
                        1.0,
                        roof_mat.clone(),
                    ));
                }
            }
        }

        // PUERTA DE MADERA
        for y in 0..3 {
            for x in 2..5 {
                self.cubes.push(Cube::new(
                    Vec3::new(house_x + x as f32, y as f32 + 1.0, house_z - 0.1),
                    1.0,
                    wood_mat.clone(),
                ));
            }
        }

        // CHIMENEA
        let chimney_x = house_x + 1.0;
        let chimney_z = house_z + house_depth as f32 - 2.0;
        for y in house_height..(house_height + 4) {
            self.cubes.push(Cube::new(
                Vec3::new(chimney_x, y as f32, chimney_z),
                1.0,
                roof_mat.clone(),
            ));
            self.cubes.push(Cube::new(
                Vec3::new(chimney_x + 1.0, y as f32, chimney_z),
                1.0,
                roof_mat.clone(),
            ));
        }
    }

    fn build_wood_pile(&mut self) {
        let wood_mat = Material::new(Color::new(0.4, 0.3, 0.2))
            .with_texture(Texture::load("assets/tronco.png"));

        // Pilas de troncos al lado derecho de la casa
        let pile_x = 8.0;
        let pile_z = 2.0;

        // Primera pila
        for i in 0..3 {
            for j in 0..3 {
                self.cubes.push(Cube::new(
                    Vec3::new(pile_x + i as f32, 0.5, pile_z + j as f32),
                    1.0,
                    wood_mat.clone(),
                ));
            }
        }

        // Segunda pila (más alta)
        for i in 0..2 {
            for j in 0..2 {
                self.cubes.push(Cube::new(
                    Vec3::new(pile_x + 4.0 + i as f32, 0.5, pile_z + j as f32),
                    1.0,
                    wood_mat.clone(),
                ));
                self.cubes.push(Cube::new(
                    Vec3::new(pile_x + 4.0 + i as f32, 1.5, pile_z + j as f32),
                    1.0,
                    wood_mat.clone(),
                ));
            }
        }
    }

    fn build_surrounding_trees(&mut self) {
        let trunk_mat = Material::new(Color::new(0.4, 0.3, 0.2))
            .with_texture(Texture::load("assets/tronco.png"));
        let leaves_mat = Material::new(Color::new(0.3, 0.5, 0.2))
            .with_texture(Texture::load("assets/pasto.png"));

        // Posiciones de árboles alrededor de la casa
        let tree_positions = [
            (-8.0, -8.0),
            (10.0, -6.0),
            (-6.0, 10.0),
            (12.0, 8.0),
            (-12.0, 4.0),
        ];

        for (x, z) in tree_positions.iter() {
            // Tronco
            for y in 0..4 {
                self.cubes.push(Cube::new(
                    Vec3::new(*x, y as f32, *z),
                    1.0,
                    trunk_mat.clone(),
                ));
            }

            // Copa del árbol
            for dx in -2..=2 {
                for dz in -2..=2 {
                    for dy in 3..6 {
                        if dx * dx + dz * dz <= 4 {
                            self.cubes.push(Cube::new(
                                Vec3::new(*x + dx as f32, dy as f32, *z + dz as f32),
                                1.0,
                                leaves_mat.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }

    fn build_stone_path(&mut self) {
        let stone_mat = Material::new(Color::new(0.6, 0.6, 0.6))
            .with_texture(Texture::load("assets/piedra.png"));

        // Camino desde la puerta hacia el sur
        for step in 1..8 {
            self.cubes.push(Cube::new(
                Vec3::new(3.0, 0.0, -step as f32),
                1.0,
                stone_mat.clone(),
            ));
            self.cubes.push(Cube::new(
                Vec3::new(4.0, 0.0, -step as f32),
                1.0,
                stone_mat.clone(),
            ));
        }
    }

    pub fn update_sun_position(&mut self, day_time: f32) {
        let angle = day_time * std::f32::consts::PI * 2.0;

        let sun_dir = Vec3::new(
            -angle.sin() * 1.0,
            -(angle.cos() + 0.5).max(0.3),
            -0.5,
        )
        .normalize();

        let sun_height = (angle.cos() + 0.5).max(0.0);
        let intensity = (sun_height * 1.2).min(1.2).max(0.3);

        self.sun = DirectionalLight::sun(sun_dir, intensity);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;
        let mut closest_t = f32::INFINITY;

        for cube in &self.cubes {
            if let Some(intersection) = cube.intersect(ray) {
                if intersection.t < closest_t {
                    closest_t = intersection.t;
                    closest = Some(intersection);
                }
            }
        }

        for mesh in &self.meshes {
            if let Some(intersection) = mesh.intersect(ray) {
                if intersection.t < closest_t {
                    closest_t = intersection.t;
                    closest = Some(intersection);
                }
            }
        }

        closest
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
