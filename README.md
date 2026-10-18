# 🚗 calc00 [Vocho Core]

Una calculadora escrita en **Rust**, con interfaz nativa de escritorio y soporte web mediante **WebAssembly**, sin Electron.

Interfaz gráfica impulsada por **`egui`** / **`eframe`**, con un motor matemático basado en un **parser de descenso recursivo** hecho desde cero.

---

## ⚙️ Características Técnicas

* **Motor Matemático:** Parser de descenso recursivo que respeta la jerarquía de operadores y admite paréntesis anidados.
* **Seguridad de Memoria y Tipos:** Garantizado por el sistema de tipos y el *borrow checker* de Rust.
* **Manejo de Unarios y Errores:** Soporta números negativos (`-5`, `--5`, `-(2+3)`), validación estricta de división entre cero y desbordamientos flotantes (`is_finite()`).
* **UI Minimalista y Responsiva:** Diseño limpio de doble línea (historial de operación + resultado) con centrado dinámico y tipografía monoespaciada.

---

## 🛠️ Tecnologías

* **Lenguaje:** Rust (Edición 2021)
* **GUI:** `egui` / `eframe` 0.29.1 (interfaz de modo inmediato).
* **Web:** `wasm-bindgen`, `web-sys` y un canvas HTML para renderizar la interfaz.

---

## 🚀 Compilación y Ejecución

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/). Ejecuta los comandos desde la carpeta que contiene `Cargo.toml`.

```sh
# Clonar el repositorio
git clone https://github.com/rodrigopanto-create/calc00.git
cd calc00

# Compilar y ejecutar en modo optimizado (--release)
cargo run --release
```

## 🌐 Compilación web

Instala el destino WebAssembly y [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/):

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
```

Genera el módulo para el navegador:

```sh
wasm-pack build --target web --out-dir dist --out-name calc00 --release
```

Este comando genera `dist/calc00.js` y `dist/calc00_bg.wasm`. La página
`dist/index.html` carga el módulo y llama a `start("calc00_canvas")`.
Los archivos generados no están versionados: deben compilarse antes de publicar
la carpeta `dist/`.

Para probar localmente, si tienes Python 3:

```sh
python3 -m http.server 8080 --directory dist
```

Abre <http://localhost:8080>. No abras el HTML directamente con `file://`.
Para publicar, sirve el contenido de `dist/` mediante HTTPS; `localhost` también
permite probar el service worker.

### Caché y manifiesto web

* `dist/sw.js` almacena la página, el cargador JavaScript y el módulo WebAssembly
  para usarlos sin conexión después de una primera carga correcta.
* La caché actual usa una estrategia *cache-first*. Al publicar una nueva
  versión, cambia `CACHE_NAME` en `dist/sw.js` y comprueba la activación del
  nuevo service worker; durante el desarrollo puedes eliminar el registro y la
  caché desde las herramientas del navegador.
* `dist/manifest.json` define la presentación de la aplicación y referencia
  `icon.png` de 192 × 192 píxeles. Ese icono todavía no está incluido: añádelo
  antes de validar la instalación como PWA. El manifiesto por sí solo no
  garantiza que el navegador ofrezca instalarla.

## 📁 Estructura del proyecto

* [src/lib.rs](src/lib.rs): evaluador matemático, estado `CalcApp`, interfaz,
  pruebas y puntos de entrada `run()` (escritorio) y `start(canvas_id)` (web).
* [src/main.rs](src/main.rs): ejecutable que inicia la aplicación nativa mediante
  `calc00::run()`.
* [Cargo.toml](Cargo.toml): dependencias y configuración de la biblioteca
  `cdylib` / `rlib`.
* [dist/index.html](dist/index.html): página con el canvas y el arranque web.
* [dist/manifest.json](dist/manifest.json): metadatos de la aplicación web.
* [dist/sw.js](dist/sw.js): service worker para la caché sin conexión.

## ✅ Validación

```sh
cargo check --locked --all-targets
cargo test --locked --lib
cargo check --locked --all-targets --target wasm32-unknown-unknown
```

Las pruebas cubren precedencia, paréntesis, asociatividad, signos unarios,
decimales, expresiones inválidas, división entre cero y formato de resultados.
La comprobación de compilación web no sustituye una prueba en el navegador.