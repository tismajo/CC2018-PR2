use std::ops::{Add, Mul};
use crate::mate::{Vec3, clamp};

/// Representa un color en el espacio RGB con componentes de punto flotante
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Componente rojo (0.0 - 1.0)
    pub r: f32,
    /// Componente verde (0.0 - 1.0)
    pub g: f32,
    /// Componente azul (0.0 - 1.0)
    pub b: f32,
}

impl Color {
    // ===== CONSTRUCTORES BÁSICOS =====
    
    /// Crea un nuevo color a partir de componentes RGB
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    
    /// Constructor desde vector 3D
    pub fn from_vec3(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
    
    /// Constructor desde valores RGB en formato u8 (0-255)
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        )
    }
    
    // ===== COLORES PREDEFINIDOS =====
    
    /// Color negro (0, 0, 0)
    pub fn black() -> Self { 
        Self::new(0.0, 0.0, 0.0) 
    }
    
    /// Color blanco (1, 1, 1)
    pub fn white() -> Self { 
        Self::new(1.0, 1.0, 1.0) 
    }
    
    /// Color rojo puro (1, 0, 0)
    pub fn red() -> Self { 
        Self::new(1.0, 0.0, 0.0) 
    }
    
    /// Color verde puro (0, 1, 0)
    pub fn green() -> Self { 
        Self::new(0.0, 1.0, 0.0) 
    }
    
    /// Color azul puro (0, 0, 1)
    pub fn blue() -> Self { 
        Self::new(0.0, 0.0, 1.0) 
    }
    
    // ===== CONVERSIONES Y OPERACIONES =====
    
    /// Convierte el color a un vector 3D
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
    
    /// Convierte el color al formato de color de Raylib
    pub fn to_raylib(&self) -> raylib::prelude::Color {
        raylib::prelude::Color::new(
            (clamp(self.r, 0.0, 1.0) * 255.0) as u8,
            (clamp(self.g, 0.0, 1.0) * 255.0) as u8,
            (clamp(self.b, 0.0, 1.0) * 255.0) as u8,
            255,
        )
    }
    
    /// Asegura que todos los componentes estén en el rango [0, 1]
    pub fn clamp(&self) -> Self {
        Self::new(
            clamp(self.r, 0.0, 1.0),
            clamp(self.g, 0.0, 1.0),
            clamp(self.b, 0.0, 1.0),
        )
    }
}

// ===== IMPLEMENTACIONES DE OPERADORES =====

/// Suma de colores componente a componente
impl Add for Color {
    type Output = Color;
    
    fn add(self, other: Color) -> Color {
        Color::new(
            self.r + other.r, 
            self.g + other.g, 
            self.b + other.b
        )
    }
}

/// Multiplicación por escalar (ajusta el brillo)
impl Mul<f32> for Color {
    type Output = Color;
    
    fn mul(self, scalar: f32) -> Color {
        Color::new(
            self.r * scalar, 
            self.g * scalar, 
            self.b * scalar
        )
    }
}

/// Multiplicación componente a componente (mezcla de colores)
impl Mul<Color> for Color {
    type Output = Color;
    
    fn mul(self, other: Color) -> Color {
        Color::new(
            self.r * other.r, 
            self.g * other.g, 
            self.b * other.b
        )
    }
}
