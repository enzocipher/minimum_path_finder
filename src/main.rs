/* 
El programa debe solicitar al usuario un número entero 𝑛 con 𝑛 ∈ [8, 16].
𝑛 representa la cantidad de nodos del grafo, y permitirle elegir entre
generarlo de forma aleatoria o manual. Una vez construido, debe mostrar el
grafo dirigido etiquetado y solicitar al usuario un vértice origen y un vértice
destino.

Con estos datos, el programa debe determinar el camino mínimo
entre ambos utilizando el algoritmo de Dijkstra, detallando paso a paso el
proceso (incluyendo el etiquetado de vértices, la actualización de distancias
y los nodos visitados en cada iteración) y presentando todos los caminos
mínimos posibles en caso de que exista más de uno
*/

 // importa el archivos como módulos
mod grafo; 
mod dijkstra;
// Traer funciones de otros archivos, deben ser publicas (pub fn) sino no funciona :P, adoro la seguridad de rust como todo buen estudiante de ciberseguridad :v
use grafo::*; 
use dijkstra::aplicar_dijkstra_y_etiquetar; 
use input_py::input;
use petgraph::graph::DiGraph;

fn main() {
    // aquí llamas las funciones que movimos
    let n_nodos = validator();
    println!("Cantidad de nodos: {}", n_nodos);

    let nodos: Vec<String>;
    let mut grafo: DiGraph<String, i32>;
    loop {
        let eleccion = input("Elija entre generar el grafo de forma aleatoria (A) o manual (M): ")
            .expect("Error al leer el input");

        match eleccion.trim().to_uppercase().as_str() {
            "A" => {
                println!("Generando etiquetas de forma aleatoria...");
                nodos = fill_vector_auto(n_nodos);
                grafo = generar_grafo_dirigido(&nodos,1);
                break
            }
            "M" => {
                println!("Generando etiquetas de forma manual...");
                nodos = fill_vector_manual(n_nodos);
                grafo = generar_grafo_dirigido(&nodos,2);
                break 
            }
            _ => println!("Opción inválida. Por favor, elija 'A' o 'M'."),
        }
    }

    println!("Nodos o vertices: {:?}", nodos);
    exportar_y_mostrar_grafo(&grafo, "grafo_antes_de_dijkstra");
    loop {
        let etiqueta_origen = input("Etiqueta del vértice ORIGEN: ").unwrap().trim().to_string();
        let etiqueta_destino = input("Etiqueta del vértice DESTINO: ").unwrap().trim().to_string();
        let (distancias, predecesor) = dijkstra::dijkstra_con_predecesores(&grafo, &etiqueta_origen);
        let camino = dijkstra::reconstruir_camino(&predecesor, &etiqueta_origen, &etiqueta_destino);
        if camino.is_empty() {
            println!("No existe camino desde '{}' hasta '{}'.", etiqueta_origen, etiqueta_destino);
        } else {
            println!("Camino mínimo: {:?}", camino);
            println!("Distancia total: {:?}", distancias.get(&etiqueta_destino));
            // >>> AQUÍ está el cambio clave: exportar SOLO el camino <<<
            grafo::exportar_camino_minimo_png(&grafo, &camino, "camino_minimo");
            println!("Presione 'X' para salir: ");
            let continuar = input("").unwrap().trim().to_uppercase();
            if continuar == "X" {
                break;
            } else {
                continue;
            }
        }
    }
}

// Pururu~ Oh in a blink gone 
