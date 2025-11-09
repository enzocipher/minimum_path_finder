//Welcome to hell
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod dibujar;
mod dijkstra;
mod grafo;
use eframe::{App, egui};
use grafo::{GrafoManual, GrafoRandom, gen_labels};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
// La interfaz no sabia hacerla asi que tuve que investigar, podra ser mejor pero a las justas entiendo como funciona.
// El proximo trabajo lo haré en python xd
// main.rs solo maneja la interfaz y su inicialización, la logica del grafo y dijkstra estan en sus respectivos modulos
// hecho con easygui (egui + eframe), no era tan
struct DijkstraApp {
    n: usize,
    modo: Modo,
    prob_extra: f32,
    manual_input: String,
    origen: usize,
    destino: usize,
    grafo: Option<DiGraph<String, i32>>,
    log: Vec<String>,
    caminos: Vec<Vec<usize>>,
    error: Option<String>,
    labels: Vec<String>,

    // Controles del gráfico
    zoom: f32,
    mostrar_pesos: bool,
    offset: egui::Vec2, // desplazamiento del gráfico
    arrastrando: bool,
    ultimo_mouse: Option<egui::Pos2>,
}

#[derive(Clone, Copy, PartialEq)]
enum Modo {
    Aleatorio,
    Manual,
}

impl Default for DijkstraApp {
    fn default() -> Self {
        Self {
            n: 8,
            modo: Modo::Aleatorio,
            prob_extra: 0.25,
            manual_input: "A B 4\nA C 2\nB D 3\nC D 1\nC B 2\nD E 5".into(),
            origen: 0,
            destino: 1,
            grafo: None,
            log: vec![],
            caminos: vec![],
            error: None,
            labels: gen_labels(16),

            zoom: 1.0,
            mostrar_pesos: true,
            offset: egui::Vec2::ZERO,
            arrastrando: false,
            ultimo_mouse: None,
        }
    }
}

impl DijkstraApp {
    fn construir(&mut self) {
        self.error = None;
        self.log.clear();
        self.caminos.clear();

        if self.n < 8 || self.n > 16 {
            self.error = Some("n debe estar entre 8 y 16".into());
            return;
        }

        let labels = self.labels[..self.n].to_vec();

        let g = match self.modo {
            Modo::Aleatorio => {
                GrafoRandom::new(labels.clone(), self.prob_extra.clamp(0.0, 1.0) as f64).generar()
            }
            Modo::Manual => {
                match GrafoManual::new(labels.clone(), self.manual_input.clone()).generar() {
                    Ok(g) => g,
                    Err(e) => {
                        self.error = Some(e);
                        return;
                    }
                }
            }
        };

        self.grafo = Some(g);
        if self.origen >= self.n {
            self.origen = 0;
        }
        if self.destino >= self.n {
            self.destino = (self.n - 1).max(0);
        }
    }

    fn correr_dijkstra(&mut self) {
        self.log.clear();
        self.caminos.clear();
        if self.grafo.is_none() {
            self.error = Some("Primero construye el grafo".into());
            return;
        }
        let g = self.grafo.as_ref().unwrap();
        let (dist, preds, pasos) = dijkstra::dijkstra_detallado(g, self.origen);
        self.log = pasos;

        if dist[self.destino].is_none() {
            self.log.push(
                "Destino no alcanzable desde el origen indicado, intente otro destino.".into(),
            );
            return;
        }

        let todas = dijkstra::reconstruir_todos_caminos(&preds, self.origen, self.destino);
        self.caminos = todas;
    }
}
// inicializador de la interfaz
impl App for DijkstraApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // panel de titulo

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(16.0, egui::FontFamily::Monospace),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(16.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Small,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        ctx.set_style(style);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading("Digrafo y Dijkstra Aplicación para trabajo de matematica computacional");
        });
        // panel lateral de controles
        egui::SidePanel::left("controls")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Parámetros");
                ui.add(egui::Slider::new(&mut self.n, 8..=16).text("n (nodos)"));

                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.modo, Modo::Aleatorio, "Aleatorio");
                    ui.radio_value(&mut self.modo, Modo::Manual, "Manual");
                });

                if self.modo == Modo::Aleatorio {
                    ui.add(
                        egui::Slider::new(&mut self.prob_extra, 0.0..=1.0)
                            .text("Prob. extra de arista"),
                    );
                } else {
                    ui.label("Aristas (una por línea): `(U)inicio (V)destino (W)peso`");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.manual_input)
                            .desired_rows(8)
                            .font(egui::TextStyle::Monospace),
                    );
                    ui.small("Ejemplo: A B 4  (A→B con peso 4)");
                }

                ui.separator();
                ui.label("Origen / Destino");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.origen).range(0..=self.n.saturating_sub(1)),
                    );
                    ui.add(
                        egui::DragValue::new(&mut self.destino).range(0..=self.n.saturating_sub(1)),
                    );
                });
                ui.small("Los índices comienzan en 0. A=0, B=1, ...");

                ui.separator();
                ui.label("Gráfico");
                ui.add(egui::Slider::new(&mut self.zoom, 0.5..=2.0).text("Zoom"));
                ui.checkbox(&mut self.mostrar_pesos, "Mostrar pesos");

                ui.separator();
                if ui.button("Construir grafo").clicked() {
                    self.construir();
                }
                if ui.button("Correr Dijkstra").clicked() {
                    self.correr_dijkstra();
                }

                if let Some(err) = &self.error {
                    ui.colored_label(egui::Color32::RED, err);
                }
            });
        // panel para ver resultado :like:
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // === Gráfico ===
                    egui::CollapsingHeader::new("Gráfico")
                        .default_open(true)
                        .show(ui, |ui| {
                            let width = ui.available_width();
                            let height = (width * 0.75).clamp(320.0, 900.0);
                            let desired = egui::vec2(width, height);
                            let (rect, resp) = ui.allocate_at_least(desired, egui::Sense::drag());
                            let painter = ui.painter_at(rect);

                            // Pan: arrastrar con mouse
                            if resp.dragged() {
                                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                                    if let Some(prev) = self.ultimo_mouse {
                                        let delta = pos - prev;
                                        self.offset += delta;
                                    }
                                    self.ultimo_mouse = Some(pos);
                                    self.arrastrando = true;
                                }
                            } else {
                                self.arrastrando = false;
                                self.ultimo_mouse = None;
                            }

                            // Zoom con Ctrl + rueda del ratón
                            let input = ctx.input(|i| i.clone());
                            if input.modifiers.ctrl && resp.hovered() {
                                let scroll = input.raw_scroll_delta.y;
                                if scroll != 0.0 {
                                    let factor = if scroll > 0.0 { 1.10 } else { 0.90 };
                                    self.zoom = (self.zoom * factor).clamp(0.5, 2.0);
                                }
                            }

                            if let Some(g) = &self.grafo {
                                let labels_now = self.labels[..self.n].to_vec();
                                dibujar::draw_graph_offset(
                                    ui,
                                    &painter,
                                    rect,
                                    g,
                                    &labels_now,
                                    self.zoom,
                                    self.mostrar_pesos,
                                    self.offset,
                                );
                            } else {
                                ui.centered_and_justified(|ui| {
                                    ui.label("Construye el grafo para visualizarlo.")
                                });
                            }
                        });

                    // === Aristas ===
                    egui::CollapsingHeader::new("Aristas del grafo")
                        .default_open(true)
                        .show(ui, |ui| {
                            if let Some(g) = &self.grafo {
                                for e in g.edge_references() {
                                    let u = e.source().index();
                                    let v = e.target().index();
                                    let w = *e.weight();
                                    ui.monospace(format!(
                                        "{}({}) -> {}({})  peso={}",
                                        self.labels[u], u, self.labels[v], v, w
                                    ));
                                }
                            } else {
                                ui.label("Aún no has construido el grafo.");
                            }
                        });

                    // === Paso a paso ===
                    egui::CollapsingHeader::new("Paso a paso (Dijkstra)")
                        .default_open(true)
                        .show(ui, |ui| {
                            if self.log.is_empty() {
                                ui.label("Sin ejecuciones aún.");
                            } else {
                                for l in &self.log {
                                    ui.monospace(l);
                                }
                            }
                        });

                    // === Caminos mínimos ===
                    egui::CollapsingHeader::new("Caminos mínimos")
                        .default_open(true)
                        .show(ui, |ui| {
                            if self.caminos.is_empty() {
                                ui.label("Sin caminos para mostrar.");
                            //else
                            } else {
                                // Necesitamos el grafo para sumar los pesos
                                let g = self.grafo.as_ref().unwrap();

                                for (i, c) in self.caminos.iter().enumerate() {
                                    // Texto del camino con etiquetas
                                    let texto = c
                                        .iter()
                                        .map(|&idx| format!("{}({})", self.labels[idx], idx))
                                        .collect::<Vec<_>>()
                                        .join(" -> ");

                                    // Suma de pesos del camino
                                    let mut total_peso: i32 = 0;
                                    for win in c.windows(2) {
                                        let u = win[0];
                                        let v = win[1];
                                        if let Some(eid) = g.find_edge(
                                            petgraph::prelude::NodeIndex::new(u),
                                            petgraph::prelude::NodeIndex::new(v),
                                        ) {
                                            if let Some(w) = g.edge_weight(eid) {
                                                total_peso += *w;
                                            }
                                        }
                                    }

                                    ui.monospace(format!(
                                        "{}: {}   |   suma de pesos = {}",
                                        i + 1,
                                        texto,
                                        total_peso
                                    ));
                                }
                            }
                        });
                });
        });
    }
}

impl DijkstraApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// El clasico app.run()
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Visiualizador Doronski",
        options,
        Box::new(|cc| Ok(Box::new(DijkstraApp::new(cc)))),
    )
}
