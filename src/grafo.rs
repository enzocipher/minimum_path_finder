use petgraph::graph::{DiGraph, NodeIndex};
use rand::Rng;

// Funciones para generar digrafos, tanto aleatorios como manuales

pub fn gen_labels(n: usize) -> Vec<String> {
    (0..n).map(|i| {
        let c = (b'A' + (i as u8)) as char;
        c.to_string()
    }).collect()
}

pub struct GrafoRandom {
    labels: Vec<String>,
    p_extra: f64,
}

// Lo de abajo pero con rng, leer documentación de GrafoManual porfavor, osea los comentarios....
impl GrafoRandom {
    pub fn new(labels: Vec<String>, p_extra: f64) -> Self {
        Self { labels, p_extra }
    }

    pub fn generar(&self) -> DiGraph<String, i32> {
        let mut grafo = DiGraph::<String, i32>::new();
        let mut idx: Vec<NodeIndex> = Vec::with_capacity(self.labels.len());
        for l in &self.labels {
            idx.push(grafo.add_node(l.clone()));
        }

        let mut rng = rand::rng();

        // Asegurar conectividad básica con un anillo
        for i in 0..idx.len() {
            let j = (i + 1) % idx.len();
            let w = rng.random_range(1..=9);
            grafo.add_edge(idx[i], idx[j], w);
        }

        // Aristas extra con probabilidad p_extra
        for i in 0..idx.len() {
            for j in 0..idx.len() {
                if i == j { continue; }
                if rng.random::<f64>() < self.p_extra {
                    let w = rng.random_range(1..=9);
                    grafo.add_edge(idx[i], idx[j], w);
                }
            }
        }
        grafo
    }
}

pub struct GrafoManual {
    labels: Vec<String>,
    raw: String,
}

impl GrafoManual {
    pub fn new(labels: Vec<String>, raw: String) -> Self {
        Self { labels, raw }
    }

    pub fn generar(&self) -> Result<DiGraph<String, i32>, String> {
        let mut grafo = DiGraph::<String, i32>::new();
        let mut idx = Vec::with_capacity(self.labels.len());
        for l in &self.labels {
            idx.push(grafo.add_node(l.clone()));
        }
        // que demonios es u? v? w? 
        // Son los nodos y el peso de la arista :P ejemplo: A B 5 es una arista de A a B con peso 5
        for (lineno, line) in self.raw.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() { continue; }
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 3 {
                return Err(format!("Línea {}: formato inválido. Usa: U V peso", lineno + 1));
            }
            let u_lbl = parts[0].to_uppercase();
            let v_lbl = parts[1].to_uppercase();
            let w: i32 = parts[2].parse().map_err(|_| format!("Línea {}: peso inválido", lineno + 1))?;

            let u = self.labels.iter().position(|s| s == &u_lbl)
                .ok_or_else(|| format!("Línea {}: nodo '{}' no existe", lineno + 1, u_lbl))?;
            let v = self.labels.iter().position(|s| s == &v_lbl)
                .ok_or_else(|| format!("Línea {}: nodo '{}' no existe", lineno + 1, v_lbl))?;

            if u == v { continue; }
            grafo.add_edge(
                petgraph::prelude::NodeIndex::new(u),
                petgraph::prelude::NodeIndex::new(v),
                w.max(1),
            );
        }
        Ok(grafo)
    }
}
