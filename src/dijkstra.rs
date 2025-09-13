use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Estructura para la cola de prioridad
#[derive(Clone, Eq, PartialEq)]
struct Entrada {
    dist: i32,
    node: usize,
}
// inicialzadores de orden para BinaryHeap (min-heap), por defecto es max-heap osea
// encontrar el mayor pero aqui se cambia al menor
impl Ord for Entrada {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist).then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for Entrada {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}


// Algoritmo, se busca el menor, si es igual, se agrega como otro predecesor y al final se compara para encontrar el de menor peso
// TOdo eso devuelve??? Si xd, devuelve distancias, predecesores y un log detallado, el log es solo para la interfaz y algunas pruebas.
pub fn dijkstra_detallado(g: &DiGraph<String, i32>, origen: usize)-> (Vec<Option<i32>>, Vec<Vec<usize>>, Vec<String>)
{
    let n = g.node_count();
    let mut dist: Vec<Option<i32>> = vec![None; n];
    let mut preds: Vec<Vec<usize>> = vec![vec![]; n];
    let mut visitado = vec![false; n];
    let mut heap: BinaryHeap<Entrada> = BinaryHeap::new();
    let mut log: Vec<String> = vec![];

    dist[origen] = Some(0);
    heap.push(Entrada { dist: 0, node: origen });
    log.push(format!("Inicializo dist[{}]=0, resto = ∞", origen));

    while let Some(Entrada { dist: d_u, node: u }) = heap.pop() {
        if visitado[u] { continue; }
        visitado[u] = true;
        log.push(format!("Selecciono u={} con dist={}", u, d_u));

        for e in g.edges(petgraph::prelude::NodeIndex::new(u)) {
            let v = e.target().index();
            let w = *e.weight();
            if visitado[v] { continue; }

            let alt = d_u.saturating_add(w);
            match dist[v] {
                None => {
                    dist[v] = Some(alt);
                    preds[v].clear();
                    preds[v].push(u);
                    heap.push(Entrada { dist: alt, node: v });
                    log.push(format!("  Relajo ({} -> {}, w={}): dist[{}]={}", u, v, w, v, alt));
                }
                Some(curr) if alt < curr => {
                    dist[v] = Some(alt);
                    preds[v].clear();
                    preds[v].push(u);
                    heap.push(Entrada { dist: alt, node: v });
                    log.push(format!("  Mejora ({} -> {}, w={}): dist[{}] {}→{}", u, v, w, v, curr, alt));
                }
                Some(curr) if alt == curr => {
                    if !preds[v].contains(&u) {
                        preds[v].push(u);
                        log.push(format!("  Empate óptimo hacia {}: también via {}", v, u));
                    }
                }
                _ => {}
            }
        }
    }

    (dist, preds, log)
}

// Generar el output de los caminos minimos que se muestra en la interfaz :good:
pub fn reconstruir_todos_caminos(preds: &Vec<Vec<usize>>, origen: usize, destino: usize) -> Vec<Vec<usize>> {
    let mut actual = vec![destino];
    let mut res: Vec<Vec<usize>> = vec![];
    fn dfs(preds: &Vec<Vec<usize>>, u: usize, origen: usize, camino: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if u == origen {
            let mut c = camino.clone();
            c.reverse();
            out.push(c);
            return;
        }
        for &p in &preds[u] {
            camino.push(p);
            dfs(preds, p, origen, camino, out);
            camino.pop();
        }
    }
    dfs(preds, destino, origen, &mut actual, &mut res);
    res
}
