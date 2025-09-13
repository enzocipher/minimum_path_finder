use input_py::input;
use rand::Rng;
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use petgraph::dot::{Dot, Config};
use std::process::Command;
use std::collections::HashMap;
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


pub fn generar_grafo_dirigido(nodos: &[String], eleccion: i8) -> DiGraph<String, i32> {
    match eleccion {
        1 => generar_grafo_dirigido_con_prob(nodos, 0.30), // Probabilidad de arista adicional del 30%, deberia bajarlo, casi siempre hay muchas...
        2 => generar_grafo_dirigido_manual(nodos), // PASS, luego sera modificado para entrada manual
        _ => panic!("Como demonios elegiste si no te dejo XD?."),
    }
}


pub fn generar_grafo_dirigido_con_prob(nodos: &[String], p_extra: f64) -> DiGraph<String, i32> {
    let mut grafo = DiGraph::<String, i32>::new();
    // 1) Añadir nodos al grafo
    let mut idxs = Vec::with_capacity(nodos.len());
    for etiqueta in nodos {
        idxs.push(grafo.add_node(etiqueta.clone()));
    }

    // Para ver si el nodo no requiere aristas adicionales :v
    if idxs.len() <= 1 {
        return grafo;
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
        if grafo.find_edge(src, dst).is_none() {
            let peso = rng.random_range(1..=10);
            grafo.add_edge(src, dst, peso);
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
                if grafo.find_edge(u, v).is_none() {
                    let peso = rng.random_range(1..=10);
                    grafo.add_edge(u, v, peso);
                }
            }
        }
    }

    grafo // retornar el grafo auto
}


/// Genera un grafo dirigido de forma manual:
/// - Primero agrega todos los nodos (según `nodos`).
/// - Luego te permite ingresar aristas (origen, destino, peso) una por una.
/// - Finaliza cuando respondes 'N' al prompt de continuar.
///
/// Se ve similar a la función anterior :P.
pub fn generar_grafo_dirigido_manual(nodos: &[String]) -> DiGraph<String, i32> {
    let mut grafo = DiGraph::<String, i32>::new();

    // 1) Agregar nodos y guardar índices por etiqueta para validar entradas
    let mut indice_por_etiqueta: HashMap<String, petgraph::prelude::NodeIndex> =
        HashMap::with_capacity(nodos.len());

    for etiqueta in nodos {
        let idx = grafo.add_node(etiqueta.clone());
        indice_por_etiqueta.insert(etiqueta.clone(), idx);
    }

    println!("\n--- Construcción manual de aristas ---");
    println!("Etiquetas disponibles: {:?}", nodos);
    println!("Para cada arista te pediré: ORIGEN, DESTINO y PESO (entero).");
    println!("Escribe las etiquetas exactamente como aparecen arriba.\n");

    loop {
        // Preguntar si desea continuar
        let continuar = input("¿Deseas agregar una arista? (S/N): ")
            .expect("Error al leer el input");
        let continuar = continuar.trim().to_uppercase();
        if continuar == "N" {
            break;
        } else if continuar != "S" {
            println!("Respuesta inválida. Escribe 'S' para sí o 'N' para no.");
            continue;
        }

        // Pedir ORIGEN
        let etiqueta_origen = loop {
            let valor = input("Etiqueta del nodo ORIGEN: ")
                .expect("Error al leer el input")
                .trim()
                .to_string();
            if let Some(_) = indice_por_etiqueta.get(&valor) {
                break valor;
            } else {
                println!("No existe un nodo con esa etiqueta. Intenta de nuevo.");
            }
        };

        // Pedir DESTINO
        let etiqueta_destino = loop {
            let valor = input("Etiqueta del nodo DESTINO: ")
                .expect("Error al leer el input")
                .trim()
                .to_string();
            if let Some(_) = indice_por_etiqueta.get(&valor) {
                if valor == etiqueta_origen {
                    println!("ORIGEN y DESTINO no pueden ser el mismo. Intenta de nuevo.");
                    continue;
                }
                break valor;
            } else {
                println!("No existe un nodo con esa etiqueta. Intenta de nuevo.");
            }
        };

        // Pedir PESO
        let peso = loop {
            let valor = input("Peso de la arista (entero >= 1): ")
                .expect("Error al leer el input")
                .trim()
                .to_string();
            match valor.parse::<i32>() {
                Ok(n) if n >= 1 => break n,
                _ => println!("Peso inválido. Debe ser un entero mayor o igual a 1."),
            }
        };

        // Agregar arista validada
        let idx_origen = *indice_por_etiqueta.get(&etiqueta_origen).unwrap();
        let idx_destino = *indice_por_etiqueta.get(&etiqueta_destino).unwrap();
        grafo.add_edge(idx_origen, idx_destino, peso); // añade las aristas ps, que tonteria tu arista

        println!(
            "Arista agregada: {} -> {} (peso: {})",
            etiqueta_origen, etiqueta_destino, peso
        );
    }

    println!("\nConstrucción manual finalizada.");
    grafo //retornar el grafo manual, EL GRAFO AUTO ES MAS DIFICIL QUE EL MANUAL  
}

pub fn exportar_camino_minimo_png(
    grafo_original: &DiGraph<String, i32>,
    camino_por_etiquetas: &[String],
    nombre_png: &str,
) {
    // 1) Mapear etiqueta -> índice (del grafo original)
    let mut indice_por_etiqueta = HashMap::new();
    for i in grafo_original.node_indices() {
        indice_por_etiqueta.insert(grafo_original[i].clone(), i);
    }

    // 2) Crear un grafo nuevo
    let mut grafo_camino = DiGraph::<String, i32>::new();

    // 3) Añadir todos los nodos (misma etiqueta visual que el original)
    let mut indice_nuevo_por_etiqueta = HashMap::new();
    for i in grafo_original.node_indices() {
        let etiqueta = grafo_original[i].clone();
        let nuevo_idx = grafo_camino.add_node(etiqueta.clone());
        indice_nuevo_por_etiqueta.insert(etiqueta, nuevo_idx);
    }

    // 4) Añadir solo las aristas que pertenecen al camino
    for ventana in camino_por_etiquetas.windows(2) {
        let a = &ventana[0];
        let b = &ventana[1];

        let Some(&src_old) = indice_por_etiqueta.get(a) else { continue; };
        let Some(&dst_old) = indice_por_etiqueta.get(b) else { continue; };

        if let Some(edge_id) = grafo_original.find_edge(src_old, dst_old) {
            let peso = *grafo_original.edge_weight(edge_id).unwrap();
            let src_new = *indice_nuevo_por_etiqueta.get(a).unwrap();
            let dst_new = *indice_nuevo_por_etiqueta.get(b).unwrap();
            grafo_camino.add_edge(src_new, dst_new, peso);
        }
    }

    // 5) Exportar a DOT y PNG
    let dot = Dot::with_config(&grafo_camino, &[Config::EdgeNoLabel]); // o muestra pesos si prefieres
    let dot_path = format!("{nombre_png}.dot");
    let png_path = format!("{nombre_png}.png");
    fs::write(&dot_path, format!("{dot:?}")).expect("No se pudo escribir .dot");

    // Layout circular como tu export original
    Command::new("dot")
        .args(&["-Kcirco", "-Tpng", &dot_path, "-o", &png_path])
        .status()
        .expect("Error al ejecutar Graphviz (dot)");

    println!("Imagen generada: {png_path}");
    // Reutiliza tu función abrir_imagen si la tienes
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