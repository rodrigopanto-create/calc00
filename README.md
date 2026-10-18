# 🚗 calc00 [Vocho Core]

Una calculadora nativa, ultraligera y de alto rendimiento escrita 100% en **Rust**, diseñada bajo una filosofía de **soberanía digital**: cero dependencias pesadas, cero telemetría, cero Electron y eficiencia absoluta al nivel del metal.

Interfaz gráfica impulsada por **`egui`** / **`eframe`**, con un motor matemático basado en un **parser de descenso recursivo** hecho desde cero.

---

## ⚙️ Características Técnicas

* **Motor Matemático Autodidacta:** Parser recursivo formal que respeta estrictamente la jerarquía de operadores y soporte para paréntesis anidados.
* **Seguridad de Memoria y Tipos:** Garantizado por el sistema de tipos y el *borrow checker* de Rust.
* **Manejo de Unarios y Errores:** Soporta números negativos (`-5`, `--5`, `-(2+3)`), validación estricta de división entre cero y desbordamientos flotantes (`is_finite()`).
* **UI Minimalista y Responsiva:** Diseño limpio de doble línea (historial de operación + resultado) con centrado dinámico y tipografía monoespaciada.

---

## 🛠️ Tecnologías

* **Lenguaje:** Rust (Edición 2021)
* **GUI:** `egui` / `eframe` (Rendering inmediato basado en GPU, rápido y sin bloatware)

---

## 🚀 Compilación y Ejecución

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/).

```bash
# Clonar el repositorio
git clone [https://github.com/rodrigopanto-create/calc00.git](https://github.com/rodrigopanto-create/calc00.git)
cd calc00

# Compilar y ejecutar en modo optimizado (--release)
cargo run --release