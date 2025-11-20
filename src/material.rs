use crate::color::Color;
use crate::texture::Texture;

/// Define las propiedades ópticas y superficiales de un objeto en la escena
/// Controla cómo interactúa la luz con la superficie para renderizado
#[derive(Clone)]
pub struct Material {
    /// Color base de la superficie (difuso)
    pub albedo: Color,
    /// Textura opcional para mapeo de superficie
    pub texture: Option<Texture>,
    /// Coeficiente de reflexión (0.0 = no reflexión, 1.0 = espejo perfecto)
    pub reflectivity: f32,
    /// Intensidad de componente especular (0.0 = sin brillo, 1.0 = máximo brillo)
    pub specular: f32,
    /// Exponente de brillo especular (valores altos = reflejos más concentrados)
    pub shininess: f32,
    /// Color y intensidad de emisión de luz propia
    pub emissive: Color,
    /// Índice de refracción para materiales transparentes
    pub refractive_index: f32,
    /// Grado de transparencia (0.0 = opaco, 1.0 = totalmente transparente)
    pub transparency: f32,
}

impl Material {
    // ===== CONSTRUCTOR PRINCIPAL Y VALORES POR DEFECTO =====
    
    /// Crea un nuevo material con color base especificado
    /// y valores por defecto para el resto de propiedades
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            texture: None,
            reflectivity: 0.0,
            specular: 0.0,
            shininess: 32.0,
            emissive: Color::black(),
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }

    // ===== MÉTODOS DE CONFIGURACIÓN CON PATRÓN BUILDER =====
    
    /// Asigna una textura al material para mapeo superficial
    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Define el coeficiente de reflexión del material
    pub fn with_reflectivity(mut self, reflectivity: f32) -> Self {
        self.reflectivity = reflectivity;
        self
    }

    /// Configura las propiedades de brillo especular
    pub fn with_specular(mut self, specular: f32, shininess: f32) -> Self {
        self.specular = specular;
        self.shininess = shininess;
        self
    }

    /// Establece propiedades de emisión de luz (materiales luminosos)
    pub fn with_emissive(mut self, emissive: Color) -> Self {
        self.emissive = emissive;
        self
    }

    /// Configura propiedades de transparencia y refracción
    pub fn with_transparency(mut self, transparency: f32, refractive_index: f32) -> Self {
        self.transparency = transparency;
        self.refractive_index = refractive_index;
        self
    }

    // ===== MÉTODOS DE CONSULTA Y CÁLCULO =====
    
    /// Obtiene el color en coordenadas UV específicas, considerando textura si existe
    pub fn get_color(&self, u: f32, v: f32) -> Color {
        if let Some(ref texture) = self.texture {
            texture.sample(u, v)
        } else {
            self.albedo
        }
    }
}

// ===== IMPLEMENTACIÓN DE TRAIT DEFAULT =====

impl Default for Material {
    /// Proporciona un material por defecto con color blanco
    /// y propiedades básicas no reflectivas
    fn default() -> Self {
        Self::new(Color::white())
    }
}
