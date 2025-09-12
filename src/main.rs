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


mod grafo; // importa el archivo grafo.rs como módulo

use grafo::*; // trae las funciones públicas de grafo.rs
use input_py::input;

fn main() {
    // aquí llamas las funciones que movimos
    let n_nodos = validator();
    println!("Cantidad de nodos: {}", n_nodos);

    let nodos: Vec<String>;
    loop {
        let eleccion = input("Elija entre generar el grafo de forma aleatoria (A) o manual (M): ")
            .expect("Error al leer el input");

        match eleccion.trim().to_uppercase().as_str() {
            "A" => {
                println!("Generando etiquetas de forma aleatoria...");
                nodos = fill_vector_auto(n_nodos);
                break;
            }
            "M" => {
                println!("Generando etiquetas de forma manual...");
                nodos = fill_vector_manual(n_nodos);
                break;
            }
            _ => println!("Opción inválida. Por favor, elija 'A' o 'M'."),
        }
    }

    println!("Vector de nodos: {:?}", nodos);

    let grafo = generar_grafo_dirigido(&nodos);
    exportar_y_mostrar_grafo(&grafo, "grafo");
}
