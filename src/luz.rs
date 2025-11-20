use crate::mate::Vec3;
use crate::color::Color;

// ===== LUZ DIRECCIONAL =====

/// Representa una fuente de luz direccional (como el sol) que ilumina desde una dirección específica
/// La luz es uniforme en toda la escena y no tiene posición definida
pub struct DirectionalLight {
    /// Vector de dirección de la luz (normalizado)
    pub direction: Vec3,
    /// Color base de la luz emitida
    pub color: Color,
    /// Intensidad de la luz (factor multiplicativo)
    pub intensity: f32,
}

impl DirectionalLight {
    /// Crea una nueva luz direccional con los parámetros especificados
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }

    // Crea una luz direccional con características similares a la luz solar
    // Utiliza un color amarillo-blanco característico de la luz del sol
    pub fn sun(direction: Vec3, intensity: f32) -> Self {
        Self::new(
            direction, 
            Color::new(1.0, 0.95, 0.9), 
            intensity
        )
    }
}

// ===== LUZ PUNTUAL =====

/// Representa una fuente de luz puntual que emite luz en todas direcciones desde una posición específica
/// Similar a una bombilla o fuente de luz localizada
pub struct PointLight {
    /// Posición en el espacio 3D de donde emana la luz
    pub position: Vec3,
    /// Color de la luz emitida
    pub color: Color,
    /// Intensidad de la luz (afecta el brillo y alcance)
    pub intensity: f32,
}

impl PointLight {
    /// Construye una nueva luz puntual en la posición especificada
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
}
