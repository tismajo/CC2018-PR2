use std::ops::{Add, Sub, Mul, Div, Neg};

/// Representa un vector tridimensional con operaciones matemáticas básicas
/// para gráficos por computadora y simulaciones físicas.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    /// Componente en el eje X
    pub x: f32,
    /// Componente en el eje Y
    pub y: f32,
    /// Componente en el eje Z
    pub z: f32,
}

// ===== IMPLEMENTACIÓN DE OPERACIONES VECTORIALES =====

impl Vec3 {
    // === CONSTRUCTORES BÁSICOS ===
    
    /// Crea un nuevo vector con los componentes especificados
    pub fn new(x: f32, y: f32, z: f32) -> Self { 
        Self { x, y, z } 
    }
    
    /// Retorna el vector cero (0, 0, 0)
    pub fn zero() -> Self { 
        Self::new(0.0, 0.0, 0.0) 
    }
    
    /// Retorna el vector unitario (1, 1, 1)
    pub fn one() -> Self { 
        Self::new(1.0, 1.0, 1.0) 
    }

    // === OPERACIONES VECTORIALES FUNDAMENTALES ===
    
    /// Calcula el producto punto entre dos vectores
    pub fn dot(&self, other: &Vec3) -> f32 { 
        self.x * other.x + self.y * other.y + self.z * other.z 
    }
    
    /// Calcula el producto cruz entre dos vectores
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // === PROPIEDADES Y NORMALIZACIÓN ===
    
    /// Calcula la longitud (magnitud) del vector
    pub fn length(&self) -> f32 { 
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt() 
    }

    /// Retorna una versión normalizada del vector (longitud = 1)
    pub fn normalize(&self) -> Vec3 {
        let magnitude = self.length();
        if magnitude > 0.0 { 
            *self / magnitude 
        } else { 
            *self 
        }
    }

    // === OPERACIONES DE ÓPTICA PARA RAY TRACING ===
    
    /// Calcula el vector reflejado respecto a una normal
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * 2.0 * self.dot(normal)
    }

    /// Calcula el vector refractado usando la ley de Snell
    /// Retorna None si ocurre reflexión interna total
    pub fn refract(&self, normal: &Vec3, eta: f32) -> Option<Vec3> {
        let cos_incident = -self.dot(normal).max(-1.0).min(1.0);
        let sin_transmitted_sq = eta * eta * (1.0 - cos_incident * cos_incident);
        
        if sin_transmitted_sq > 1.0 { 
            None  // Reflexión interna total
        } else {
            let cos_transmitted = (1.0 - sin_transmitted_sq).sqrt();
            Some(*self * eta + *normal * (eta * cos_incident - cos_transmitted))
        }
    }
}

// ===== FUNCIONES UTILITARIAS GLOBALES =====

/// Restringe un valor al rango [min, max]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min { 
        min 
    } else if value > max { 
        max 
    } else { 
        value 
    }
}

// ===== IMPLEMENTACIONES DE OPERADORES =====

/// Suma componente a componente de dos vectores
impl Add for Vec3 { 
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x + other.x, 
            self.y + other.y, 
            self.z + other.z
        ) 
    } 
}

/// Resta componente a componente de dos vectores
impl Sub for Vec3 { 
    type Output = Vec3;
    
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x - other.x, 
            self.y - other.y, 
            self.z - other.z
        ) 
    } 
}

/// Multiplicación por escalar
impl Mul<f32> for Vec3 { 
    type Output = Vec3;
    
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3::new(
            self.x * scalar, 
            self.y * scalar, 
            self.z * scalar
        )
    }
}

/// División por escalar
impl Div<f32> for Vec3 {
    type Output = Vec3;
    
    fn div(self, scalar: f32) -> Vec3 {
        Vec3::new(
            self.x / scalar, 
            self.y / scalar, 
            self.z / scalar
        ) 
    } 
}

/// Negación del vector (inversión de dirección)
impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        Vec3::new(
            -self.x, 
            -self.y, 
            -self.z
        ) 
    } 
}
