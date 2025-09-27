/// Igual que draw_graph pero permite desplazamiento (offset) para pan.
pub fn draw_graph_offset(
    _ui: &egui::Ui,
    painter: &egui::Painter,
    rect: egui::Rect,
    g: &DiGraph<String, i32>,
    labels: &Vec<String>,
    zoom: f32,
    mostrar_pesos: bool,
    offset: Vec2,
) {
    let n = g.node_count();
    if n == 0 { return; }

    // layout circular con offset
    let center = rect.center() + offset;
    let r = (rect.width().min(rect.height()) * 0.45 * zoom as f32).max(40.0);
    let radio_nodo = (18.0 * zoom as f32).clamp(10.0, 30.0);

    // posiciones de nodos
    let mut pos: Vec<Pos2> = Vec::with_capacity(n);
    for i in 0..n {
        let ang = (i as f32) / (n as f32) * std::f32::consts::TAU;
        pos.push(Pos2 { x: center.x + r * ang.cos(), y: center.y + r * ang.sin() });
    }

    // estilos
    let stroke_edge = Stroke { width: 1.5, color: Color32::from_gray(90) };
    let stroke_arrow = Stroke { width: 1.5, color: Color32::from_gray(90) };
    let color_peso = Color32::from_rgb(240, 240, 240);

    // conteo de paralelas
    let mut multi_count: HashMap<(usize, usize), usize> = HashMap::new();
    for e in g.edge_references() {
        let key = (e.source().index(), e.target().index());
        *multi_count.entry(key).or_insert(0) += 1;
    }
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();

    // dibujar aristas
    for e in g.edge_references() {
        let u = e.source().index();
        let v = e.target().index();
        let w = *e.weight();

        let pu = pos[u];
        let pv = pos[v];

        // dir/normal
        let dir = (pv - pu).normalized();
        let normal = Vec2::new(-dir.y, dir.x);

        // margenes
        let margen = radio_nodo + 4.0;
        let a = pu + dir * margen;
        let b = pv - dir * margen;

        let key = (u, v);
        let total = *multi_count.get(&key).unwrap_or(&1);

        if total == 1 {
            // ---- única arista: recta ----
            painter.line_segment([a, b], stroke_edge);

            // flecha recta
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
                // Separar pesos si existe arista opuesta
                let mut mid = Pos2 { x: (a.x + b.x) * 0.5, y: (a.y + b.y) * 0.5 };
                let font = egui::FontId::proportional((12.0 * zoom).clamp(10.0, 18.0));
                let opuesta = multi_count.get(&(v, u)).unwrap_or(&0) > &0;
                if opuesta {
                    let sep = 12.0 * zoom;
                    mid += normal * sep;
                }
                painter.text(mid, Align2::CENTER_CENTER, format!("{}", w), font, color_peso);
            }
        } else {
            // ---- múltiples aristas: curvas a lados opuestos ----
            let idx = *seen.entry(key).and_modify(|i| *i += 1).or_insert(0);

            let side = if idx % 2 == 0 { 1.0 } else { -1.0 };
            let tier = (idx / 2) as f32 + 1.0;

            let dist = (b - a).length();
            let base_curva = (dist * 0.20).clamp(16.0, 80.0);
            let offset = side * tier * base_curva;

            let push = (dist * 0.10).clamp(8.0, 32.0);
            let ctrl1 = a + dir * push + normal * offset;
            let ctrl2 = b - dir * push + normal * offset;

            // polilínea de la Bezier
            let samples = 24;
            let mut pts: Vec<Pos2> = Vec::with_capacity(samples + 1);
            for i in 0..=samples {
                let t = i as f32 / samples as f32;
                pts.push(bezier_point(a, ctrl1, ctrl2, b, t));
            }
            painter.add(egui::Shape::line(pts.clone(), stroke_edge));

            // tangente = último tramo dibujado
            let tip = pts[samples];
            let prev = pts[samples.saturating_sub(1)];
            let tan_dir = (tip - prev).normalized();

            // flecha alineada a la curva
            let arrow_len = 12.0;
            let arrow_w = 6.0;
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

            if mostrar_pesos {
                let mut mid = bezier_point(a, ctrl1, ctrl2, b, 0.5);
                let font = egui::FontId::proportional((12.0 * zoom).clamp(10.0, 18.0));
                let opuesta = multi_count.get(&(v, u)).unwrap_or(&0) > &0;
                if opuesta {
                    let sep = 12.0 * zoom;
                    mid += normal * sep;
                } else {
                    mid += normal * (offset.signum() * 0.10 * arrow_w);
                }
                painter.text(
                    mid,
                    Align2::CENTER_CENTER,
                    format!("{}", w),
                    font,
                    color_peso,
                );
            }
        }
    }

    // nodos
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

    // borde
    painter.rect(
        rect,
        Rounding::same(8.0),
        Color32::TRANSPARENT,
        Stroke { width: 1.0, color: Color32::from_gray(180) },
    );
}

use eframe::egui::{self, Align2, Color32, Pos2, Rounding, Stroke, Vec2};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

fn bezier_point(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
    let u = 1.0 - t;
    let uu = u * u;
    let tt = t * t;
    let uuu = uu * u;
    let ttt = tt * t;
    Pos2 {
        x: uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
        y: uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
    }
}

