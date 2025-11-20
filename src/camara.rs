use crate::ray::Ray;
use crate::mate::Vec3;

/// Sistema de cámara que soporta movimiento orbital y navegación libre
pub struct Camera {
    // Parámetros públicos de configuración
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub aspect: f32,
    
    // Estado interno para control orbital
    orbital_distance: f32,
    rotation_horizontal: f32,
    rotation_vertical: f32,
}

impl Camera {
    /// Construye una nueva cámara con parámetros iniciales
    pub fn new(position: Vec3, target: Vec3, fov: f32, aspect: f32) -> Self {
        let orbital_distance = (position - target).length();
        let direction_normalized = (position - target).normalize();
        
        // Calcular ángulos iniciales basados en la posición
        let rotation_horizontal = direction_normalized.z.atan2(direction_normalized.x);
        let rotation_vertical = direction_normalized.y.asin();
        
        Camera {
            position,
            target,
            fov,
            aspect,
            orbital_distance,
            rotation_horizontal,
            rotation_vertical,
        }
    }
    
    // ===== MÉTODOS DE MOVIMIENTO Y NAVEGACIÓN =====
    
    /// Desplazamiento lateral hacia la izquierda
    pub fn strafe_left(&mut self, distance: f32) {
        let right_vector = self.calculate_right_vector();
        self.apply_translation(-right_vector * distance);
    }
    
    /// Desplazamiento lateral hacia la derecha
    pub fn strafe_right(&mut self, distance: f32) {
        let right_vector = self.calculate_right_vector();
        self.apply_translation(right_vector * distance);
    }
    
    /// Movimiento hacia adelante en la dirección actual
    pub fn move_forward(&mut self, distance: f32) {
        let forward_vector = self.calculate_forward_vector();
        self.apply_translation(forward_vector * distance);
    }
    
    /// Movimiento hacia atrás en la dirección actual
    pub fn move_backward(&mut self, distance: f32) {
        let forward_vector = self.calculate_forward_vector();
        self.apply_translation(-forward_vector * distance);
    }
    
    /// Movimiento vertical ascendente
    pub fn move_up(&mut self, distance: f32) {
        let vertical_offset = Vec3::new(0.0, distance, 0.0);
        self.apply_translation(vertical_offset);
    }
    
    /// Movimiento vertical descendente
    pub fn move_down(&mut self, distance: f32) {
        let vertical_offset = Vec3::new(0.0, -distance, 0.0);
        self.apply_translation(vertical_offset);
    }
    
    // ===== CONTROLES DE ROTACIÓN ORBITAL =====
    
    /// Rotación horizontal alrededor del objetivo
    pub fn rotate_around_target(&mut self, angle_change: f32) {
        self.rotation_horizontal += angle_change.to_radians();
        self.refresh_camera_transform();
    }
    
    /// Rotación vertical con límites de ángulo
    pub fn rotate_vertical(&mut self, angle_change: f32) {
        self.rotation_vertical += angle_change.to_radians();
        // Restringir ángulo vertical para evitar volteos
        self.rotation_vertical = self.rotation_vertical.clamp(-1.5, 1.5);
        self.refresh_camera_transform();
    }
    
    // ===== CONTROL DE ZOOM =====
    
    /// Ajusta la distancia de la cámara al objetivo
    pub fn zoom(&mut self, zoom_delta: f32) {
        self.orbital_distance -= zoom_delta;
        // Mantener distancia dentro de límites razonables
        self.orbital_distance = self.orbital_distance.clamp(1.0, 50.0);
        self.refresh_camera_transform();
    }
    
    // ===== GENERACIÓN DE RAYOS =====
    
    /// Genera un rayo desde la cámara a través de coordenadas normalizadas del viewport
    pub fn get_ray(&self, viewport_u: f32, viewport_v: f32) -> Ray {
        let forward_dir = (self.target - self.position).normalize();
        let right_dir = forward_dir.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up_dir = right_dir.cross(&forward_dir).normalize();
        
        let fov_radians = self.fov.to_radians();
        let viewport_half_height = (fov_radians / 2.0).tan();
        let viewport_half_width = self.aspect * viewport_half_height;
        
        // Calcular dirección del rayo en el espacio de la cámara
        let ray_direction = forward_dir
            + right_dir * (2.0 * viewport_u - 1.0) * viewport_half_width
            + up_dir * (1.0 - 2.0 * viewport_v) * viewport_half_height;
        
        Ray::new(self.position, ray_direction.normalize())
    }
    
    // ===== MÉTODOS PRIVADOS DE APOYO =====
    
    /// Calcula vector de dirección frontal normalizado
    fn calculate_forward_vector(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }
    
    /// Calcula vector de dirección derecha normalizado
    fn calculate_right_vector(&self) -> Vec3 {
        let forward = self.calculate_forward_vector();
        forward.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize()
    }
    
    /// Calcula vector de dirección superior normalizado
    fn calculate_up_vector(&self) -> Vec3 {
        let forward = self.calculate_forward_vector();
        let right = self.calculate_right_vector();
        right.cross(&forward).normalize()
    }
    
    /// Aplica desplazamiento tanto a posición como a objetivo
    fn apply_translation(&mut self, translation: Vec3) {
        self.position = self.position + translation;
        self.target = self.target + translation;
    }
    
    /// Actualiza posición de la cámara basada en parámetros orbitales
    fn refresh_camera_transform(&mut self) {
        let offset_x = self.orbital_distance * 
                      self.rotation_vertical.cos() * 
                      self.rotation_horizontal.cos();
        
        let offset_y = self.orbital_distance * 
                      self.rotation_vertical.sin();
        
        let offset_z = self.orbital_distance * 
                      self.rotation_vertical.cos() * 
                      self.rotation_horizontal.sin();
        
        self.position = self.target + Vec3::new(offset_x, offset_y, offset_z);
    }
}
