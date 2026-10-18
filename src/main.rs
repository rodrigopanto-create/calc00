use eframe::egui;

// ============================================================
// VOCHO CORE: EVALUADOR MATEMATICO
// ============================================================

fn evaluar(expresion: &str) -> Result<f64, String> {
    let mut parser = Parser {
        chars: expresion.chars().collect(),
        pos: 0,
    };

    let resultado = parser.parse_expresion()?;
    parser.ignorar_espacios();

    if parser.pos != parser.chars.len() {
        return Err("Token inesperado".to_string());
    }

    if !resultado.is_finite() {
        return Err("Resultado fuera de rango".to_string());
    }

    Ok(resultado)
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn ignorar_espacios(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn actual(&mut self) -> Option<char> {
        self.ignorar_espacios();
        self.chars.get(self.pos).copied()
    }

    fn consumir(&mut self, esperado: char) -> bool {
        if self.actual() == Some(esperado) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn parse_expresion(&mut self) -> Result<f64, String> {
        let mut izquierda = self.parse_termino()?;

        loop {
            match self.actual() {
                Some('+') => {
                    self.pos += 1;
                    izquierda += self.parse_termino()?;
                }
                Some('-') => {
                    self.pos += 1;
                    izquierda -= self.parse_termino()?;
                }
                _ => break,
            }
        }

        Ok(izquierda)
    }

    fn parse_termino(&mut self) -> Result<f64, String> {
        let mut izquierda = self.parse_unario()?;

        loop {
            match self.actual() {
                Some('*') => {
                    self.pos += 1;
                    izquierda *= self.parse_unario()?;
                }
                Some('/') => {
                    self.pos += 1;
                    let derecha = self.parse_unario()?;

                    if derecha == 0.0 {
                        return Err("División entre cero".to_string());
                    }

                    izquierda /= derecha;
                }
                _ => break,
            }
        }

        Ok(izquierda)
    }

    fn parse_unario(&mut self) -> Result<f64, String> {
        if self.consumir('+') {
            return self.parse_unario();
        }

        if self.consumir('-') {
            return Ok(-self.parse_unario()?);
        }

        self.parse_primario()
    }

    fn parse_primario(&mut self) -> Result<f64, String> {
        if self.consumir('(') {
            let valor = self.parse_expresion()?;

            if !self.consumir(')') {
                return Err("Falta cerrar paréntesis".to_string());
            }

            return Ok(valor);
        }

        self.parse_numero()
    }

    fn parse_numero(&mut self) -> Result<f64, String> {
        self.ignorar_espacios();

        let inicio = self.pos;
        let mut puntos = 0;

        while let Some(&c) = self.chars.get(self.pos) {
            if c.is_ascii_digit() {
                self.pos += 1;
            } else if c == '.' {
                puntos += 1;

                if puntos > 1 {
                    return Err("Número decimal inválido".to_string());
                }

                self.pos += 1;
            } else {
                break;
            }
        }

        if inicio == self.pos {
            return Err("Se esperaba un número".to_string());
        }

        let texto: String = self.chars[inicio..self.pos].iter().collect();

        if !texto.chars().any(|c| c.is_ascii_digit()) {
            return Err("Número decimal inválido".to_string());
        }

        texto
            .parse::<f64>()
            .map_err(|_| "Número inválido".to_string())
    }
}

fn formatear(valor: f64) -> String {
    if valor == 0.0 {
        return "0".to_string();
    }

    let texto = format!("{:.10}", valor);
    let texto = texto.trim_end_matches('0');
    let texto = texto.trim_end_matches('.');

    texto.to_string()
}

// ============================================================
// ESTADO DE LA CALCULADORA
// ============================================================

struct CalcApp {
    display: String,
    expresion_previa: Option<String>,
    mensaje: Option<String>,
    reemplazar_display: bool,
}

impl Default for CalcApp {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            expresion_previa: None,
            mensaje: None,
            reemplazar_display: false,
        }
    }
}

impl CalcApp {
    fn agregar(&mut self, token: &str) {
        let es_inicio = token.chars().all(|c| c.is_ascii_digit())
            || token == "."
            || token == "("
            || token == "-";

        if self.mensaje.is_some() {
            if !es_inicio {
                return;
            }
            self.display.clear();
            self.mensaje = None;
            self.expresion_previa = None;
        } else if self.reemplazar_display && es_inicio {
            self.display.clear();
            self.expresion_previa = None;
        }

        self.reemplazar_display = false;

        // Sustituir el "0" inicial al presionar números, '(' o '-'
        if self.display == "0" {
            if token.chars().all(|c| c.is_ascii_digit()) || token == "(" || token == "-" {
                self.display.clear();
            } else if token == "." {
                self.display = "0".to_string();
            }
        }

        if self.display.len() + token.len() > 100 {
            return;
        }

        if token == "." {
            let ultimo_numero: String = self.display
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();

            if ultimo_numero.contains('.') {
                return;
            }
        }

        self.display.push_str(token);
    }

    fn borrar(&mut self) {
        if self.mensaje.is_some() || self.reemplazar_display {
            self.limpiar();
            return;
        }

        self.display.pop();

        if self.display.is_empty() {
            self.display = "0".to_string();
        }

        self.mensaje = None;
    }

    fn limpiar(&mut self) {
        self.display = "0".to_string();
        self.expresion_previa = None;
        self.mensaje = None;
        self.reemplazar_display = false;
    }

    fn calcular(&mut self) {
        let expr = self.display.clone();

        match evaluar(&expr) {
            Ok(resultado) => {
                self.expresion_previa = Some(format!("{expr} ="));
                self.display = formatear(resultado);
                self.mensaje = None;
                self.reemplazar_display = true;
            }
            Err(error) => {
                self.expresion_previa = Some(format!("{expr} ="));
                self.mensaje = Some(error);
                self.reemplazar_display = true;
            }
        }
    }
}

// ============================================================
// INTERFAZ GRAFICA EGUI
// ============================================================

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🚗 calc00 [Rust Core]");
            });

            ui.add_space(8.0);

            // Pantalla LCD estilo doble línea
            egui::Frame::group(ui.style()).show(ui, |ui| {
                let ancho = ui.available_width();

                ui.set_width(ancho);
                ui.set_min_height(85.0);
                ui.set_max_height(85.0);

                ui.vertical(|ui| {
                    // Línea superior: Operación previa
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let texto_previo = self
                                .expresion_previa
                                .as_deref()
                                .unwrap_or("");
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(texto_previo)
                                        .size(16.0)
                                        .monospace()
                                        .weak(),
                                )
                                .truncate(),
                            );
                        },
                    );

                    ui.add_space(4.0);

                    // Línea inferior: Entrada / Resultado principal
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&self.display)
                                        .size(28.0)
                                        .monospace()
                                        .strong(),
                                )
                                .truncate(),
                            );
                        },
                    );
                });
            });

            if let Some(error) = &self.mensaje {
                ui.vertical_centered(|ui| {
                    ui.colored_label(
                        egui::Color32::RED,
                        format!("Error: {error}"),
                    );
                });
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(10.0);

            // Botonera
            let botones = [
                ["C", "DEL", "(", ")"],
                ["7", "8", "9", "/"],
                ["4", "5", "6", "*"],
                ["1", "2", "3", "-"],
                ["0", ".", "=", "+"],
            ];

            let ancho_boton = 58.0;
            let alto_boton = 48.0;
            let espaciado = 8.0;

            // Ancho total del bloque de botones: (4 * 58) + (3 * 8) = 256.0
            let ancho_total_teclado = (4.0 * ancho_boton) + (3.0 * espaciado);
            let margen_izq = ((ui.available_width() - ancho_total_teclado) / 2.0).max(0.0);

            // Centrar horizontalmente la rejilla de botones
            ui.horizontal(|ui| {
                ui.add_space(margen_izq);

                egui::Grid::new("teclado_grid")
                    .spacing([espaciado, espaciado])
                    .min_col_width(ancho_boton)
                    .show(ui, |ui| {
                        for fila in botones {
                            for token in fila {
                                let respuesta = ui.add_sized(
                                    [ancho_boton, alto_boton],
                                    egui::Button::new(
                                        egui::RichText::new(token).size(20.0),
                                    ),
                                );

                                if respuesta.clicked() {
                                    match token {
                                        "C" => self.limpiar(),
                                        "DEL" => self.borrar(),
                                        "=" => self.calcular(),
                                        _ => self.agregar(token),
                                    }
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}

// ============================================================
// PRUEBAS UNITARIAS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn casi_igual(actual: f64, esperado: f64) {
        assert!(
            (actual - esperado).abs() < 1e-10,
            "Esperado {esperado}, obtenido {actual}"
        );
    }

    #[test]
    fn respeta_precedencia() {
        casi_igual(evaluar("2 + 3 * 4").unwrap(), 14.0);
        casi_igual(evaluar("10 - 2 * 3").unwrap(), 4.0);
        casi_igual(evaluar("8 / 2 + 3").unwrap(), 7.0);
    }

    #[test]
    fn respeta_parentesis() {
        casi_igual(evaluar("(2 + 3) * 4").unwrap(), 20.0);
        casi_igual(evaluar("2 * (3 + 4)").unwrap(), 14.0);
        casi_igual(evaluar("((2 + 3))").unwrap(), 5.0);
    }

    #[test]
    fn evalua_izquierda_a_derecha() {
        casi_igual(evaluar("10 - 3 - 2").unwrap(), 5.0);
        casi_igual(evaluar("24 / 4 / 2").unwrap(), 3.0);
    }

    #[test]
    fn admite_signos_unarios() {
        casi_igual(evaluar("-5").unwrap(), -5.0);
        casi_igual(evaluar("2 * -3").unwrap(), -6.0);
        casi_igual(evaluar("--5").unwrap(), 5.0);
        casi_igual(evaluar("-(2 + 3)").unwrap(), -5.0);
    }

    #[test]
    fn admite_decimales_y_espacios() {
        casi_igual(evaluar("0.1 + 0.2").unwrap(), 0.3);
        casi_igual(evaluar(" 3.5 * 2 ").unwrap(), 7.0);
        casi_igual(evaluar(".5 + .5").unwrap(), 1.0);
    }

    #[test]
    fn detecta_division_entre_cero() {
        assert!(evaluar("1 / 0").is_err());
        assert!(evaluar("1 / (3 - 3)").is_err());
    }

    #[test]
    fn rechaza_expresiones_invalidas() {
        assert!(evaluar("").is_err());
        assert!(evaluar("2 +").is_err());
        assert!(evaluar("(2 + 3").is_err());
        assert!(evaluar("2 + 3)").is_err());
        assert!(evaluar("2 ** 3").is_err());
        assert!(evaluar("1..2").is_err());
        assert!(evaluar("abc").is_err());
    }

    #[test]
    fn detecta_resultados_no_finitos() {
        assert!(evaluar("1e308 * 1e308").is_err());
    }

    #[test]
    fn formatea_resultados() {
        assert_eq!(formatear(0.0), "0");
        assert_eq!(formatear(5.0), "5");
        assert_eq!(formatear(0.3), "0.3");
        assert_eq!(formatear(-2.5), "-2.5");
    }
}

// ============================================================
// PUNTO DE ENTRADA
// ============================================================

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 500.0]) // <--- Ajustado a 500 para el margen inferior
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "calc00 - Vocho Core",
        options,
        Box::new(|_cc| Ok(Box::new(CalcApp::default()))),
    )
}