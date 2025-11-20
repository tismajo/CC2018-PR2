use crate::mate::Vec3;
use crate::material::Material;

/// Representa el punto de intersección entre un rayo y una superficie geométrica
/// Contiene toda la información necesaria para el cálculo de iluminación y texturas
#[derive(Clone)]
pub struct Intersection {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersection {
    /// Construye una nueva instancia de Intersection con todos los parámetros necesarios
    /// Una nueva instancia de Intersection con los valores proporcionados
    pub fn new(
        t: f32, 
        position: Vec3, 
        normal: Vec3, 
        material: Material, 
        u: f32, 
        v: f32
    ) -> Self {
        // Crear y retornar la estructura con todos los campos
        Self {
            t,
            position,
            normal,
            material,
            u,
            v,
        }
    }
}
