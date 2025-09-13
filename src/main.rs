#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod grafo;
mod dijkstra;

use eframe::{egui, App};
use grafo::{gen_labels, GrafoManual, GrafoRandom};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef; 

// La interfaz no sabia hacerla asi que tuve que investigar, podra ser mejor pero a las justas entiendo como funciona.
// El proximo trabajo lo haré en python xd
// main.rs solo maneja la interfaz y su inicialización, la logica del grafo y dijkstra estan en sus respectivos modulos

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
}

#[derive(Clone, Copy, PartialEq)]
enum Modo { Aleatorio, Manual }

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
        if self.origen >= self.n { self.origen = 0; }
        if self.destino >= self.n { self.destino = (self.n - 1).max(0); }
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
            self.log.push("Destino no alcanzable desde el origen.".into());
            return;
        }

        let todas = dijkstra::reconstruir_todos_caminos(&preds, self.origen, self.destino);
        self.caminos = todas;
    }
}

impl App for DijkstraApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // panel de titulo
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.heading("Digrafo y Dijkstra Aplicación para trabajo de matematica computacional");
        });
        // panel lateral de controles
        egui::SidePanel::left("controls").resizable(true).show(ctx, |ui| {
            ui.label("Parámetros");
            ui.add(egui::Slider::new(&mut self.n, 8..=16).text("n (nodos)"));

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.modo, Modo::Aleatorio, "Aleatorio");
                ui.radio_value(&mut self.modo, Modo::Manual, "Manual");
            });

            if self.modo == Modo::Aleatorio {
                ui.add(egui::Slider::new(&mut self.prob_extra, 0.0..=1.0).text("Prob. extra de arista"));
            } else {
                ui.label("Aristas (una por línea): `U V peso`");
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
                ui.add(egui::DragValue::new(&mut self.origen).range(0..=self.n.saturating_sub(1)));
                ui.add(egui::DragValue::new(&mut self.destino).range(0..=self.n.saturating_sub(1)));
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
                .auto_shrink([false, false]) // no encojas si hay más ancho/alto
                .show(ui, |ui| {
                    // === Gráfico ===
                    egui::CollapsingHeader::new("Gráfico")
                        .default_open(true)
                        .show(ui, |ui| {
                            // reservar un ALTO fijo (o calculado) para el lienzo
                            let width = ui.available_width();
                            let height = (width * 0.75).clamp(320.0, 900.0); // ajustable

                            let desired = egui::vec2(width, height);
                            let (rect, _resp) = ui.allocate_at_least(desired, egui::Sense::hover());
                            let painter = ui.painter_at(rect);

                            if let Some(g) = &self.grafo {
                                let labels_now = self.labels[..self.n].to_vec();
                                draw_graph(
                                    ui,
                                    &painter,
                                    rect,
                                    g,
                                    &labels_now,
                                    self.zoom,
                                    self.mostrar_pesos,
                                );
                            } else {
                                ui.centered_and_justified(|ui| ui.label("Construye el grafo para visualizarlo."));
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
                            } else {
                                for (i, c) in self.caminos.iter().enumerate() {
                                    let texto = c
                                        .iter()
                                        .map(|&idx| format!("{}({})", self.labels[idx], idx))
                                        .collect::<Vec<_>>()
                                        .join(" -> ");
                                    ui.monospace(format!("{}: {}", i + 1, texto));
                                }
                            }
                        });
                });
        });

    }
}

impl DijkstraApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self { Self::default() }
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

// Dibujar grafo 

fn draw_graph(
    _ui: &egui::Ui,
    painter: &egui::Painter,
    rect: egui::Rect,
    g: &DiGraph<String, i32>,
    labels: &Vec<String>,
    zoom: f32,
    mostrar_pesos: bool,
) {
    use egui::{Color32, Pos2, Vec2, Stroke, Align2, Rounding};

    let n = g.node_count();
    if n == 0 { return; }

    // Layout circular
    let center = rect.center();
    let r = (rect.width().min(rect.height()) * 0.45 * zoom as f32).max(40.0);
    let radio_nodo = (18.0 * zoom as f32).clamp(10.0, 30.0);

    // Precomputar posiciones
    let mut pos: Vec<Pos2> = Vec::with_capacity(n);
    for i in 0..n {
        let ang = (i as f32) / (n as f32) * std::f32::consts::TAU;
        let p = Pos2 {
            x: center.x + r * ang.cos(),
            y: center.y + r * ang.sin(),
        };
        pos.push(p);
    }

    // Dibujar aristas
    let stroke_edge = Stroke { width: 1.5, color: Color32::from_gray(90) };
    let stroke_arrow = Stroke { width: 1.5, color: Color32::from_gray(90) };
    let color_peso = Color32::from_gray(40);

    // la mayoria de lo que hay aqui no tiene que ver con la logica del grafo, es solo logica de la interfaz
    for e in g.edge_references() {
        let u = e.source().index();
        let v = e.target().index();
        let w = *e.weight();

        let pu = pos[u];
        let pv = pos[v];

        // Vector dirección
        let dir = (pv - pu).normalized();
        let margen = radio_nodo + 4.0;

        // Puntos con margen para que la línea no entre al círculo
        let a = pu + dir * margen;
        let b = pv - dir * margen;

        // La linea principal
        painter.line_segment([a, b], stroke_edge);

        // La flechita
        let arrow_len = 12.0;
        let arrow_w = 6.0;
        let tip = b;
        let base = tip - dir * arrow_len;
        let perp = Vec2::new(-dir.y, dir.x) * arrow_w;

        let p1 = base + perp;
        let p2 = base - perp;
        painter.add(egui::Shape::convex_polygon(
            vec![p1, tip, p2],
            stroke_arrow.color,               // color de relleno
            egui::Stroke::new(1.0, stroke_arrow.color), // borde opcional
        ));


        // Peso en el punto medio
        if mostrar_pesos {
            let mid = Pos2 {
                x: (a.x + b.x) * 0.5,
                y: (a.y + b.y) * 0.5,
            } + perp * 0.1;
            painter.text(
                mid,
                Align2::CENTER_CENTER,
                w.to_string(),
                egui::FontId::proportional((12.0 * zoom).clamp(10.0, 18.0)),
                color_peso,
            );
        }
    }

    // Dibujar nodos
    let stroke_node = Stroke { width: 2.0, color: Color32::from_rgb(40, 90, 60) };
    let fill_node = Color32::from_rgb(178, 220, 180);
    let color_text = Color32::from_rgb(20, 40, 25);

    for i in 0..n {
        let p = pos[i];
        painter.circle(p, radio_nodo, fill_node, stroke_node);
        painter.text(
            p,
            Align2::CENTER_CENTER,
            format!("{}\n({})", labels[i], i),
            egui::FontId::proportional((12.0 * zoom).clamp(10.0, 18.0)),
            color_text,
        );
    }

    // Bordecito
    painter.rect(
        rect,
        Rounding::same(8.0),
        Color32::TRANSPARENT,
        Stroke { width: 1.0, color: Color32::from_gray(180) },
    );
}
