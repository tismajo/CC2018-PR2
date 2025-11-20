// Fuente de luz

use crate::color::Color;
use crate::mate::Vec3;

/// Representa una fuente de luz puntual que emite iluminación en todas direcciones
/// desde una posición específica en el espacio, con atenuación por distancia.
#[derive(Clone)]
pub struct PointLight {
    /// Ubicación espacial de la fuente luminosa
    pub position: Vec3,
    /// Tono base de la luz emitida
    pub color: Color,
    /// Intensidad base de la fuente luminosa
    pub intensity: f32,
    /// Distancia máxima de alcance de la iluminación
    pub radius: f32,
}

impl PointLight {
    // ===== CONSTRUCTOR PRINCIPAL =====
    
    /// Construye una nueva fuente de luz puntual con los parámetros especificados
    pub fn new(position: Vec3, color: Color, intensity: f32, radius: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            radius,
        }
    }

    // ===== CÁLCULOS DE ILUMINACIÓN =====
    
    /// Calcula la contribución lumínica en un punto específico del espacio
    /// 
    /// # Argumentos
    /// 
    /// * `point` - Punto en el espacio donde se evalúa la iluminación
    /// 
    /// # Retorna
    /// 
    /// Una tupla que contiene:
    /// - Dirección del vector de luz (normalizado)
    /// - Color de la luz con atenuación aplicada
    /// 
    /// # Notas
    /// 
    /// La iluminación se atenúa cuadráticamente con la distancia y
    /// se anula completamente más allá del radio especificado
    pub fn illuminate(&self, point: &Vec3) -> (Vec3, Color) {
        let vector_hacia_luz = self.position - *point;
        let distancia = vector_hacia_luz.length();

        // Verificar si el punto está fuera del alcance de la luz
        if distancia > self.radius {
            return (Vec3::new(0.0, 0.0, 0.0), Color::black());
        }

        let direccion_luz = vector_hacia_luz.normalize();

        // Calcular atenuación usando modelo cuadrático: 1 / (1 + d² * factor)
        // Esto produce una caída realista de la intensidad lumínica
        let factor_atenuacion = 1.0 / (1.0 + distancia * distancia * 0.5);

        let color_atenuado = self.color * (self.intensity * factor_atenuacion);

        (direccion_luz, color_atenuado)
    }
}
