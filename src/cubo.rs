use crate::mate::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::intersection::Intersection;

/// Representa un cubo en el espacio 3D con soporte para materiales múltiples
pub struct Cube {
    /// Centro del cubo en coordenadas mundiales
    pub position: Vec3,
    /// Longitud de cada arista del cubo
    pub size: f32,
    /// Material por defecto para todas las caras
    pub material: Material,
    /// Material específico para la cara superior (opcional)
    pub top_material: Option<Material>,
    /// Material específico para las caras laterales (opcional)
    pub side_material: Option<Material>,
    /// Material específico para la cara inferior (opcional)
    pub bottom_material: Option<Material>,
}

impl Cube {
    // ===== CONSTRUCTORES =====
    
    /// Crea un nuevo cubo con un material único para todas las caras
    pub fn new(position: Vec3, size: f32, material: Material) -> Self {
        Self {
            position,
            size,
            material,
            top_material: None,
            side_material: None,
            bottom_material: None,
        }
    }

    /// Crea un cubo con materiales diferentes para la parte superior, laterales e inferior
    pub fn new_multi_texture(
        position: Vec3,
        size: f32,
        top: Material,
        sides: Material,
        bottom: Material,
    ) -> Self {
        Self {
            position,
            size,
            material: sides.clone(),
            top_material: Some(top),
            side_material: Some(sides),
            bottom_material: Some(bottom),
        }
    }

    // ===== MÉTODOS DE INTERSECCIÓN =====

    /// Calcula la intersección entre un rayo y el cubo usando el método slab
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let half_size = self.size / 2.0;
        let min_bound = self.position - Vec3::new(half_size, half_size, half_size);
        let max_bound = self.position + Vec3::new(half_size, half_size, half_size);

        // Pre-calcular la dirección inversa para optimización
        let inv_direction = Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        // Calcular distancias de intersección para cada par de planos
        let tx1 = (min_bound.x - ray.origin.x) * inv_direction.x;
        let tx2 = (max_bound.x - ray.origin.x) * inv_direction.x;
        let ty1 = (min_bound.y - ray.origin.y) * inv_direction.y;
        let ty2 = (max_bound.y - ray.origin.y) * inv_direction.y;
        let tz1 = (min_bound.z - ray.origin.z) * inv_direction.z;
        let tz2 = (max_bound.z - ray.origin.z) * inv_direction.z;

        // Encontrar los valores t mínimos y máximos válidos
        let t_near = tx1.min(tx2).max(ty1.min(ty2)).max(tz1.min(tz2));
        let t_far = tx1.max(tx2).min(ty1.max(ty2)).min(tz1.max(tz2));

        // Verificar si hay intersección válida
        if t_far < 0.0 || t_near > t_far {
            return None;
        }

        // Seleccionar el punto de intersección más cercano
        let t_value = if t_near > 0.001 { t_near } else { t_far };
        if t_value < 0.001 {
            return None;
        }

        // Calcular información de la intersección
        let intersection_point = ray.at(t_value);
        let surface_normal = self.compute_surface_normal(intersection_point, &min_bound, &max_bound);
        let (texture_u, texture_v) = self.compute_texture_coordinates(intersection_point, &surface_normal);

        // Seleccionar material apropiado según la cara impactada
        let face_material = self.select_face_material(&surface_normal);

        Some(Intersection::new(
            t_value,
            intersection_point,
            surface_normal,
            face_material,
            texture_u,
            texture_v,
        ))
    }

    // ===== MÉTODOS PRIVADOS DE APOYO =====

    /// Determina qué material usar basado en la normal de la superficie impactada
    fn select_face_material(&self, normal: &Vec3) -> Material {
        // Cara superior (normal apuntando hacia arriba)
        if normal.y > 0.5 {
            if let Some(ref material) = self.top_material {
                return material.clone();
            }
        }
        // Cara inferior (normal apuntando hacia abajo)
        else if normal.y < -0.5 {
            if let Some(ref material) = self.bottom_material {
                return material.clone();
            }
        }
        // Caras laterales
        else {
            if let Some(ref material) = self.side_material {
                return material.clone();
            }
        }

        // Material por defecto si no hay material específico
        self.material.clone()
    }

    /// Calcula el vector normal en el punto de intersección
    fn compute_surface_normal(&self, point: Vec3, min_bound: &Vec3, max_bound: &Vec3) -> Vec3 {
        let tolerance = 0.001;

        if (point.x - min_bound.x).abs() < tolerance { 
            Vec3::new(-1.0, 0.0, 0.0) 
        }
        else if (point.x - max_bound.x).abs() < tolerance { 
            Vec3::new(1.0, 0.0, 0.0) 
        }
        else if (point.y - min_bound.y).abs() < tolerance { 
            Vec3::new(0.0, -1.0, 0.0) 
        }
        else if (point.y - max_bound.y).abs() < tolerance { 
            Vec3::new(0.0, 1.0, 0.0) 
        }
        else if (point.z - min_bound.z).abs() < tolerance { 
            Vec3::new(0.0, 0.0, -1.0) 
        }
        else { 
            Vec3::new(0.0, 0.0, 1.0) 
        }
    }

    /// Calcula las coordenadas de textura (UV) para el punto de intersección
    fn compute_texture_coordinates(&self, point: Vec3, normal: &Vec3) -> (f32, f32) {
        let local_coords = point - self.position;
        let half_size = self.size / 2.0;

        let u_coord: f32;
        let v_coord: f32;

        if normal.x.abs() > 0.5 {
            // Caras orientadas en X - invertir V para corregir textura al revés
            u_coord = (local_coords.z + half_size) / self.size;
            v_coord = 1.0 - (local_coords.y + half_size) / self.size;
        } else if normal.y.abs() > 0.5 {
            // Caras superiores/inferiores (orientadas en Y) - mapeo UV normal
            u_coord = (local_coords.x + half_size) / self.size;
            v_coord = (local_coords.z + half_size) / self.size;
        } else {
            // Caras orientadas en Z - invertir V para corregir textura al revés
            u_coord = (local_coords.x + half_size) / self.size;
            v_coord = 1.0 - (local_coords.y + half_size) / self.size;
        }

        (u_coord, v_coord)
    }
}
