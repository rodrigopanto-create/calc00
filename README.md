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

Para probar localmente, instala `miniserve` si todavía no lo tienes:

```sh
cargo install miniserve --locked
```

Desde la carpeta del proyecto, sirve los archivos web:

```sh
miniserve dist -p 8081 --index index.html
```

Si ya estás dentro de `dist/`, usa `miniserve -p 8081 --index index.html`.
Abre <http://localhost:8081>. No abras el HTML directamente con `file://`.
Para publicar, sirve el contenido de `dist/` mediante HTTPS; `localhost` también
permite probar el service worker. Acceder desde otro dispositivo a una dirección
como `http://192.168.1.67:8081` no habilita el service worker: cambiar de servidor
no elimina el requisito de HTTPS con un certificado confiable.

### Caché y manifiesto web

* `dist/sw.js` almacena la página, el cargador JavaScript y el módulo WebAssembly
  para usarlos sin conexión después de una primera carga correcta.
* La caché usa una estrategia *cache-first*. GitHub Actions asigna una versión
  distinta a la caché en cada despliegue. Para una publicación manual, cambia
  `CACHE_NAME` en `dist/sw.js`.
* Una actualización espera a que cierres las ventanas de la aplicación anterior.
  Al activarse, elimina las cachés antiguas de calc00 y toma control de las
  páginas dentro de su ámbito.
* El mensaje **Lista sin conexión** aparece después de activar el service worker,
  que solo se instala si puede almacenar todos los recursos requeridos.
  Los errores de registro o instalación se muestran en pantalla y en la consola.
* `dist/manifest.json` incluye ámbito relativo e iconos PNG de 192 y 512 píxeles.
  La caché incluye el manifiesto y los iconos.

## Publicación en GitHub Pages

URL de la aplicación: <https://rodrigopanto-create.github.io/calc00/>.

El flujo [.github/workflows/pages.yml](.github/workflows/pages.yml) se ejecuta
con cada push a `main` o manualmente desde GitHub Actions. Compila la biblioteca
WebAssembly, genera los enlaces JavaScript, prepara un directorio con solo los
recursos públicos y lo despliega mediante GitHub Pages.

En **Settings > Pages > Build and deployment**, el origen debe ser
**GitHub Actions**. El despliegue necesita permisos `pages: write` y
`id-token: write`, definidos en el flujo. No requiere guardar binarios generados
en Git.

### Instalar y probar sin conexión

1. Abre la URL HTTPS con conexión y espera el mensaje **Lista sin conexión**.
2. En Android/Chrome, usa **Instalar aplicación** o **Añadir a pantalla de inicio**
   según la versión. En iPhone/Safari, usa **Compartir > Añadir a pantalla de inicio**
   y la opción de abrir como aplicación web, si está disponible.
3. Abre la aplicación desde el icono instalado. Una pestaña normal conserva la
   barra de dirección aunque el manifiesto declare `standalone`.
4. Cierra la aplicación, activa el modo avión y vuelve a abrirla desde el icono.
   Comprueba que puedes calcular, por ejemplo, `2 + 3 * 4 = 14`.

El modo sin conexión conserva los archivos de la aplicación, no el historial
ni la operación en curso. El navegador puede eliminar el almacenamiento por falta
de espacio, limpieza manual o políticas de navegación privada; en ese caso es
necesaria otra carga con conexión.

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