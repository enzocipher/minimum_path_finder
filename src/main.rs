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
// inicializador de la interfaz
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

                                    ui.monospace(format!("{}: {}   |   suma de pesos = {}", i + 1, texto, total_peso));
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

    // escenario circular :v
    let center = rect.center();
    let r = (rect.width().min(rect.height()) * 0.45 * zoom as f32).max(40.0);
    let radio_nodo = (18.0 * zoom as f32).clamp(10.0, 30.0);

    // posiciones de los nodos
    let mut pos: Vec<Pos2> = Vec::with_capacity(n);
    for i in 0..n {
        let ang = (i as f32) / (n as f32) * std::f32::consts::TAU;
        let p = Pos2 {
            x: center.x + r * ang.cos(),
            y: center.y + r * ang.sin(),
        };
        pos.push(p);
    }

    // las aristas y sus flechas (colores y longitudes)
    let stroke_edge = Stroke { width: 1.5, color: Color32::from_gray(90) };
    let stroke_arrow = Stroke { width: 1.5, color: Color32::from_gray(90) };
    //color blanco pa los pesos
    let color_peso = Color32::from_rgb(240, 240, 240);

    // la mayoria de lo que hay aqui no tiene que ver con la logica del grafo, es solo logica de la interfaz
    use std::collections::HashMap;

    // === Helpers Bezier (punto y tangente final) ===
    let _bezier_point = |p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32| -> Pos2 {
        let u = 1.0 - t;
        let uu = u * u;
        let tt = t * t;
        let uuu = uu * u;
        let ttt = tt * t;
        Pos2 {
            x: uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
            y: uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
        }
    };

    // === Preconteo de cuántas aristas paralelas hay por (u,v) ===
    let mut multi_count: HashMap<(usize, usize), usize> = HashMap::new();
    for e in g.edge_references() {
        let key = (e.source().index(), e.target().index());
        *multi_count.entry(key).or_insert(0) += 1;
    }
    // Llevamos cuántas ya hemos dibujado por (u,v) para alternar lados/espaciado
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();

    // === Dibujo de aristas ===
    for e in g.edge_references() {
        let u = e.source().index();
        let v = e.target().index();
        let w = *e.weight();

        let pu = pos[u];
        let pv = pos[v];

        // Dirección y normal
        let dir = (pv - pu).normalized();
        let normal = Vec2::new(-dir.y, dir.x);

        // Margen para no meterse al círculo del nodo
        let margen = radio_nodo + 4.0;
        let a = pu + dir * margen;
        let b = pv - dir * margen;

        // ¿cuántas aristas (u,v)?
        let key = (u, v);
        let total = *multi_count.get(&key).unwrap_or(&1);

        if total == 1 {
            // === ÚNICA ARISTA -> RECTO ===
            painter.line_segment([a, b], stroke_edge);

            // Flecha recta (igual que tu lógica original)
            let arrow_len = 12.0;
            let arrow_w = 6.0;
            let tip = b;
            let base = tip - dir * arrow_len;
            let peso_perp = Vec2::new(-dir.y, dir.x) * arrow_w;
            let p1 = base + peso_perp;
            let p2 = base - peso_perp;

            painter.add(egui::Shape::convex_polygon(
                vec![p1, tip, p2],
                stroke_arrow.color,
                egui::Stroke::new(1.0, stroke_arrow.color),
            ));

            if mostrar_pesos {
                let mid = Pos2 { x: (a.x + b.x) * 0.5, y: (a.y + b.y) * 0.5 };
                let font = egui::FontId::proportional((12.0 * zoom).clamp(10.0, 18.0));
                painter.text(mid, Align2::CENTER_CENTER, format!("{}", w), font, color_peso);
            }
        } else {
            // ======= múltiples aristas: curvas =======
            let idx = *seen.entry(key).and_modify(|i| *i += 1).or_insert(0);

            // alterna lado (+/-) y escala separación
            let side = if idx % 2 == 0 { 1.0 } else { -1.0 };
            let tier = (idx / 2) as f32 + 1.0;

            // curvatura base proporcional a distancia
            let dist = (b - a).length();
            let base_curva = (dist * 0.20).clamp(16.0, 80.0);
            let offset = side * tier * base_curva;

            // *** NUEVO: empuje longitudinal para control points ***
            let push = (dist * 0.10).clamp(8.0, 32.0);

            // control points: no simétricos para mejorar la curvatura visual
            let ctrl1 = a + dir * push + normal * offset;
            let ctrl2 = b - dir * push + normal * offset;

            // polilínea de la Bezier
            let samples = 24;
            let mut pts: Vec<Pos2> = Vec::with_capacity(samples + 1);
            let _bezier_point = |p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32| -> Pos2 {
                let u = 1.0 - t;
                let uu = u * u;
                let tt = t * t;
                let uuu = uu * u;
                let ttt = tt * t;
                Pos2 {
                    x: uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
                    y: uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
                }
            };
            for i in 0..=samples {
                let t = i as f32 / samples as f32;
                pts.push(_bezier_point(a, ctrl1, ctrl2, b, t));
            }
            painter.add(egui::Shape::line(pts.clone(), stroke_edge));

            // *** NUEVO: tangente = último tramo de la polilínea ***
            let tip = pts[samples];                    // final real de la curva
            let prev = pts[samples.saturating_sub(1)]; // punto anterior
            let tan_dir = (tip - prev).normalized();

            // flecha alineada a la tangente de la curva
            let arrow_len = 12.0;
            let arrow_w = 6.0;
            // retrocede un pelo para que la base no entre al nodo
            let tip_adj = tip - tan_dir * 0.5;
            let base = tip_adj - tan_dir * arrow_len;
            let perp = Vec2::new(-tan_dir.y, tan_dir.x) * arrow_w;

            let p1 = base + perp;
            let p2 = base - perp;

            painter.add(egui::Shape::convex_polygon(
                vec![p1, tip_adj, p2],
                stroke_arrow.color,
                egui::Stroke::new(1.0, stroke_arrow.color),
            ));

            // peso en el medio de la curva, levemente hacia el lado de la curva
            if mostrar_pesos {
                let mid = _bezier_point(a, ctrl1, ctrl2, b, 0.5);
                let font = egui::FontId::proportional((12.0 * zoom).clamp(10.0, 18.0));
                painter.text(
                    mid + normal * (offset.signum() * 0.10 * arrow_w),
                    Align2::CENTER_CENTER,
                    format!("{}", w),
                    font,
                    color_peso,
                );
            }

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
