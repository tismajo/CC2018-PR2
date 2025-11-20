use crate::mate::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::intersection::Intersection;

// ===== TRIÁNGULO =====

/// Representa un triángulo en el espacio 3D con sus vértices y normal
pub struct Triangle {
    /// Primer vértice del triángulo
    pub v0: Vec3,
    /// Segundo vértice del triángulo
    pub v1: Vec3,
    /// Tercer vértice del triángulo
    pub v2: Vec3,
    /// Vector normal de la superficie del triángulo
    pub normal: Vec3,
}

impl Triangle {
    /// Construye un nuevo triángulo a partir de tres vértices
    /// Calcula automáticamente la normal de la superficie
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(&edge2).normalize();

        Self { v0, v1, v2, normal }
    }

    /// Implementa el algoritmo Möller-Trumbore para intersección rayo-triángulo
    /// Retorna el parámetro t de intersección si existe
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let ray_cross_edge2 = ray.direction.cross(&edge2);
        let determinant = edge1.dot(&ray_cross_edge2);

        // Verificar si el rayo es paralelo al triángulo
        if determinant.abs() < 0.00001 {
            return None;
        }

        let inv_determinant = 1.0 / determinant;
        let origin_to_v0 = ray.origin - self.v0;
        
        // Calcular coordenada barycéntrica U
        let u = inv_determinant * origin_to_v0.dot(&ray_cross_edge2);
        
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let origin_cross_edge1 = origin_to_v0.cross(&edge1);
        
        // Calcular coordenada barycéntrica V
        let v = inv_determinant * ray.direction.dot(&origin_cross_edge1);
        
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Calcular distancia de intersección
        let t = inv_determinant * edge2.dot(&origin_cross_edge1);

        if t > 0.001 {
            Some(t)
        } else {
            None
        }
    }
}

// ===== MALLA 3D =====

/// Representa una malla 3D compuesta por múltiples triángulos
pub struct Mesh {
    /// Lista de triángulos que forman la malla
    pub triangles: Vec<Triangle>,
    /// Posición de la malla en el espacio mundial
    pub position: Vec3,
    /// Factor de escala de la malla
    pub scale: f32,
    /// Material aplicado a toda la malla
    pub material: Material,
}

impl Mesh {
    // ===== CONSTRUCTORES =====
    
    /// Crea una nueva malla vacía en la posición especificada
    pub fn new(position: Vec3, material: Material) -> Self {
        Self {
            triangles: Vec::new(),
            position,
            scale: 1.0,
            material,
        }
    }

    /// Carga una malla desde archivo OBJ con escala y posición especificadas
    pub fn load_obj(path: &str, position: Vec3, scale: f32, material: Material) -> Self {
        println!("Cargando modelo OBJ: {} (escala: {})", path, scale);

        let config_carga = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        };

        match tobj::load_obj(path, &config_carga) {
            Ok((modelos, _materiales)) => {
                let mut triangulos = Vec::new();

                for modelo in modelos {
                    let malla = &modelo.mesh;
                    let posiciones = &malla.positions;
                    let indices = &malla.indices;

                    println!("  Modelo '{}': {} vértices, {} triángulos",
                        modelo.name,
                        posiciones.len() / 3,
                        indices.len() / 3
                    );

                    // Generar triángulos a partir de los índices
                    for i in (0..indices.len()).step_by(3) {
                        let idx0 = indices[i] as usize;
                        let idx1 = indices[i + 1] as usize;
                        let idx2 = indices[i + 2] as usize;

                        let vertice0 = Vec3::new(
                            posiciones[idx0 * 3] * scale,
                            posiciones[idx0 * 3 + 1] * scale,
                            posiciones[idx0 * 3 + 2] * scale,
                        );

                        let vertice1 = Vec3::new(
                            posiciones[idx1 * 3] * scale,
                            posiciones[idx1 * 3 + 1] * scale,
                            posiciones[idx1 * 3 + 2] * scale,
                        );

                        let vertice2 = Vec3::new(
                            posiciones[idx2 * 3] * scale,
                            posiciones[idx2 * 3 + 1] * scale,
                            posiciones[idx2 * 3 + 2] * scale,
                        );

                        triangulos.push(Triangle::new(vertice0, vertice1, vertice2));
                    }
                }

                println!("Carga exitosa: {} triángulos", triangulos.len());

                Self {
                    triangles: triangulos,
                    position,
                    scale,
                    material,
                }
            }
            Err(error) => {
                eprintln!("Error cargando archivo OBJ '{}': {}", path, error);
                eprintln!("Creando malla de respaldo (pirámide)");

                // Crear pirámide simple como respaldo
                let triangulos_respaldo = Self::crear_piramide_respaldo(scale);

                Self {
                    triangles: triangulos_respaldo,
                    position,
                    scale,
                    material,
                }
            }
        }
    }

    // ===== OPERACIONES DE TRANSFORMACIÓN =====
    
    /// Rota toda la malla alrededor del eje Y por el ángulo especificado (radianes)
    pub fn rotate_y(&mut self, angulo: f32) {
        let coseno = angulo.cos();
        let seno = angulo.sin();

        for triangulo in &mut self.triangles {
            // Rotar cada vértice del triángulo
            Self::rotar_vertice(&mut triangulo.v0, coseno, seno);
            Self::rotar_vertice(&mut triangulo.v1, coseno, seno);
            Self::rotar_vertice(&mut triangulo.v2, coseno, seno);

            // Recalcular normal después de la rotación
            let arista1 = triangulo.v1 - triangulo.v0;
            let arista2 = triangulo.v2 - triangulo.v0;
            triangulo.normal = arista1.cross(&arista2).normalize();
        }
    }

    // ===== MÉTODOS DE INTERSECCIÓN =====
    
    /// Calcula la intersección entre un rayo y la malla
    /// Retorna la intersección más cercana si existe
    pub fn intersect(&self, rayo: &Ray) -> Option<Intersection> {
        let mut distancia_minima = f32::INFINITY;
        let mut triangulo_mas_cercano: Option<&Triangle> = None;

        // Transformar rayo al espacio local de la malla
        let rayo_local = Ray::new(rayo.origin - self.position, rayo.direction);

        for triangulo in &self.triangles {
            if let Some(distancia) = triangulo.intersect(&rayo_local) {
                if distancia < distancia_minima {
                    distancia_minima = distancia;
                    triangulo_mas_cercano = Some(triangulo);
                }
            }
        }

        triangulo_mas_cercano.map(|triangulo| {
            let punto_impacto = rayo.at(distancia_minima);
            Intersection::new(
                distancia_minima,
                punto_impacto,
                triangulo.normal,
                self.material.clone(),
                0.0,  // UV no implementado
                0.0,
            )
        })
    }

    // ===== MÉTODOS PRIVADOS DE APOYO =====
    
    /// Rota un vértice individual alrededor del eje Y
    fn rotar_vertice(vertice: &mut Vec3, coseno: f32, seno: f32) {
        let x_original = vertice.x;
        let z_original = vertice.z;
        
        vertice.x = x_original * coseno - z_original * seno;
        vertice.z = x_original * seno + z_original * coseno;
    }

    /// Crea una pirámide simple como malla de respaldo
    fn crear_piramide_respaldo(escala: f32) -> Vec<Triangle> {
        vec![
            // Cara frontal
            Triangle::new(
                Vec3::new(-0.5 * escala, 0.0, -0.5 * escala),
                Vec3::new(0.5 * escala, 0.0, -0.5 * escala),
                Vec3::new(0.0, 1.0 * escala, 0.0),
            ),
            // Cara derecha
            Triangle::new(
                Vec3::new(0.5 * escala, 0.0, -0.5 * escala),
                Vec3::new(0.5 * escala, 0.0, 0.5 * escala),
                Vec3::new(0.0, 1.0 * escala, 0.0),
            ),
            // Cara trasera
            Triangle::new(
                Vec3::new(0.5 * escala, 0.0, 0.5 * escala),
                Vec3::new(-0.5 * escala, 0.0, 0.5 * escala),
                Vec3::new(0.0, 1.0 * escala, 0.0),
            ),
            // Cara izquierda
            Triangle::new(
                Vec3::new(-0.5 * escala, 0.0, 0.5 * escala),
                Vec3::new(-0.5 * escala, 0.0, -0.5 * escala),
                Vec3::new(0.0, 1.0 * escala, 0.0),
            ),
        ]
    }
}
