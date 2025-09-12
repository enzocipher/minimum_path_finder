use input_py::input;
use rand::Rng;
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use petgraph::dot::{Dot, Config};
use std::process::Command;
use std::fs;

pub fn validator() -> i32 {
    loop {
        let input = input("Ingrese un número entre 8 y 16: ").expect("Error al leer el input");
        let numero: i32 = input.trim().parse().unwrap_or(-1);

        if !(8..=16).contains(&numero) {
            println!("El input debe ser un número entre 8 y 16");
        } else {
            return numero;
        }
    }
}

pub fn fill_vector_auto(n: i32) -> Vec<String> {
    (1..=n).map(|i| format!("Nodo{}", i)).collect()
}

pub fn fill_vector_manual(n: i32) -> Vec<String> {
    let mut nodos = Vec::with_capacity(n as usize);
    for i in 0..n {
        let etiqueta = loop {
            let s = input(&format!("Ingrese la etiqueta del nodo {}: ", i + 1))
                .expect("Error al leer el input");
            let s = s.trim().to_string();
            if s.is_empty() {
                println!("La etiqueta no puede estar vacía.");
            } else {
                break s;
            }
        };
        nodos.push(etiqueta);
    }
    nodos
}


pub fn generar_grafo_dirigido(nodos: &[String]) -> DiGraph<String, i32> {
    generar_grafo_dirigido_con_prob(nodos, 0.3)
}


pub fn generar_grafo_dirigido_con_prob(nodos: &[String], p_extra: f64) -> DiGraph<String, i32> {
    let mut g = DiGraph::<String, i32>::new();
    // 1) Añadir nodos al grafo
    let mut idxs = Vec::with_capacity(nodos.len());
    for etiqueta in nodos {
        idxs.push(g.add_node(etiqueta.clone()));
    }

    // Para ver si el nodo no requiere aristas adicionales :v
    if idxs.len() <= 1 {
        return g;
    }

    let mut rng = rand::rng();

    // 2) Crear el orden para que no se vea como fideo
    //    Esto es un shuffle pero si no lo quieres ps lo borras papito
    let mut orden = idxs.clone();
    // entreverar el orden
    for i in (1..orden.len()).rev() {
        let j = rng.random_range(0..=i);
        orden.swap(i, j);
    }

    // 3) Conectar los nodos en cadena ToT
    //    Esto garantiza conectividad si ignoramos la dirección.
    for w in orden.windows(2) {
        let a = w[0];
        let b = w[1];

        // dirección aleatoria para que no sea fuertemente conexo por obligación
        let (src, dst) = if rng.random_bool(0.5) { (a, b) } else { (b, a) };

        // nao paralelas exactas pero si ciclos
        if g.find_edge(src, dst).is_none() {
            let peso = rng.random_range(1..=10);
            g.add_edge(src, dst, peso);
        }
    }

    // 4) Aristas adicionales aleatorias pa q se vea como en un camino real
    //    Prob = p_extra por par (i,j), i != j
    let n = idxs.len();
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if rng.random_bool(p_extra) {
                let (u, v) = (idxs[i], idxs[j]);
                if g.find_edge(u, v).is_none() {
                    let peso = rng.random_range(1..=10);
                    g.add_edge(u, v, peso);
                }
            }
        }
    }

    g
}

// Mostrar el grafo en formato texto para debug, no se usara en el trabajo final pero queria ver como se ven las conexiones :v
pub fn mostrar_grafo(g: &DiGraph<String, i32>) {
    println!("\nNodos:");
    for idx in g.node_indices() {
        println!("  {:?}: {}", idx, g[idx]);
    }

    println!("\nAristas dirigidas (origen -> destino) [peso]:");
    for edge in g.edge_references() {
        let (src, dst) = (edge.source(), edge.target());
        println!("  {} -> {} [{}]", g[src], g[dst], edge.weight());
    }

    println!("\n--- Grafo en formato DOT ---\n");
    let dot = Dot::with_config(g, &[Config::EdgeNoLabel]);
    println!("{dot:?}");
}

// Convertir el grafo a imagen, uso: choco install graphviz, no tienes chocolate instalado? Yo tambien pense lo mismo XD
pub fn exportar_y_mostrar_grafo(g: &DiGraph<String, i32>, nombre: &str) {
    let dot = Dot::new(g);
    let dot_path = format!("{nombre}.dot");
    let png_path = format!("{nombre}.png");
    fs::write(&dot_path, format!("{dot:?}")).expect("No se pudo escribir el .dot"); //expect manejo de errores, por alguna razón explota si no pongo esto

    // -Kcirco usa el motor 'circo' (circular), osea los nodos se acomodan en circulo, no quieres ver como se ve por defecto :calaverita:
    Command::new("dot")
        .args(&["-Kcirco", "-Tpng", &dot_path, "-o", &png_path])
        .status()
        .expect("Error al ejecutar Graphviz (dot -Kcirco)");

    println!("Imagen generada: {png_path}");
    abrir_imagen(&png_path);
}
fn abrir_imagen(path: &str) {
    let _ = Command::new("cmd").args(&["/C", "start", "", path]).status(); //abrir imagen en windows, en linux es xdg-open y en mac es open
}