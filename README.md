# Casa del Leñador - Raytracer

Un raytracer 3D en tiempo real escrito en Rust que muestra una casa de leñador (con farmeador de experiencia) con efectos avanzados de iluminación, reflexión y transparencia.

## 🎮 Características

- **Renderizado por Raytracing** en tiempo real
- **Sistema día/noche** dinámico con transiciones suaves
- **Skybox procedural** con cielos azules diurnos y púrpura nocturno
- **Iluminación avanzada**:
  - Luz direccional (sol/luna)
  - Luces puntuales con atenuación
  - Sombras suaves
  - Reflexiones y refracciones
- **Materiales realistas**:
  - Texturas difusas
  - Reflexiones especulares
  - Transparencia y refracción (vidrio, agua)
  - Materiales emisivos
- **Optimizaciones**:
  - Renderizado multihilo
  - Sistema de calidad adaptable
  - Downscaling dinámico

## 🚀 Controles

### Movimiento de Cámara
- **W/S**: Mirar arriba/abajo
- **A/D**: Mirar izquierda/derecha  
- **Q/E**: Mover cámara arriba/abajo
- **Flechas ↑/↓**: Zoom in/out
- **Flechas ←/→**: Rotar cámara

### Configuración de Renderizado
- **1/2/3**: Cambiar calidad (Baja/Media/Alta)
- **P**: Activar/desactivar calidad automática
- **T**: Activar/desactivar multihilo
- **N**: Avanzar tiempo (día/noche)

---
## Video funcionamiento
[<video controls src="Screen Recording 2025-11-18 215049.mp4" title="Title"></video>
](https://github.com/tismajo/CC2018-PR2/blob/main/Screen%20Recording%202025-11-18%20215049.mp4)

## 📊 Rendimiento

- Resolución: 800x600 por defecto
- Rayos por píxel: 1 (path tracing básico)
- Profundidad máxima: 8 rebotes
- Threads: 4 por defecto

---

## 🚀 Instalación

### 1️⃣ Clonar el proyecto
```bash
git clone https://github.com/tismajo/CC2018-PR2.git
cd off-3d-version
```

### 2️⃣ Instalar dependencias
Asegúrate de tener instalado **Rust** (v1.70 o superior) y **cargo**.  
Luego instala las librerías necesarias:

```bash
cargo build
```

### 3️⃣ Ejecutar
```bash
cargo run
```

---

## 🧠 Créditos

- 💻 **Programación:** María José Girón Isidro, 23559 (Rust + Raylib)
- 🎨 **Inspiración visual:** *Minecraft* de MIcrosoft
- 🧱 **Engine base:** [raylib-rs](https://github.com/deltaphc/raylib-rs)

---

## ⚙️ Dependencias (Cargo.toml)

Asegúrate de incluir esto en tu `Cargo.toml`:

```toml
[dependencies]
raylib = "5.5.1"
rodio = "0.17"
minifb = "0.28" 
nalgebra = "0.34.1"
tobj = "4.0.2"
image = "0.25"
```
---

## 📜 Licencia

Este proyecto es un **fan game sin fines comerciales**.  
Usa este código libremente para fines educativos o recreativos.

---
