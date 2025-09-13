use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::{BinaryHeap, HashMap}; // Binary heap sirve para extraer siempre el mayor o el menor con más facilidad :pulgararriba:
use std::cmp::Ordering;


// EN dijkstra todos los nodos tienen distancia infinita :P
/* En principio se usa el mismo concepto q aprendi en matematica discreta, buscar el menor más cercano a un vecino y 
   si ambos caminos son iguales, 
   se elije el que la suma de sus pesos de aristas sea menor. Hasta que se recorra el camino. */
   
/// Representa un nodo en la cola de prioridad (distancia acumulada + índice)
#[derive(Copy, Clone, Eq, PartialEq)]
struct NodoEnCola {
    distancia: i32,
    indice: petgraph::prelude::NodeIndex,
}

impl Ord for NodoEnCola {  //inicializador de orden
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap es un max-heap, así que invertimos para que sea min-heap, osea pasa de mayor distancia a la menor distancia 
        other.distancia.cmp(&self.distancia)
    }
}

impl PartialOrd for NodoEnCola { //inicializador de orden parcial, 5 y 5 no pueden ser comparados, entonces se usa esto
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implementación del algoritmo de Dijkstra, retorna un HashMap (Coleccion [python], como le quieras decir) con las distancias mínimas desde el nodo origen hacia cada nodo.
fn dijkstra(grafo: &DiGraph<String, i32>, etiqueta_origen: &str) -> HashMap<String, i32> {
    let mut distancias: HashMap<String, i32> = HashMap::new();
    let mut cola = BinaryHeap::new();

    // Buscar el índice del nodo origen
    let indice_origen = grafo
        .node_indices()
        .find(|i| grafo[*i] == etiqueta_origen) // buscar el nodo con la etiqueta 
        .expect("El nodo origen no existe en el grafo");

    // Inicializar distancias
    for indice in grafo.node_indices() {
        distancias.insert(grafo[indice].clone(), i32::MAX);
    }
    distancias.insert(etiqueta_origen.to_string(), 0);

    // Insertar el origen en la cola
    cola.push(NodoEnCola {
        distancia: 0,
        indice: indice_origen,
    });

    // Algoritmo principal
    while let Some(NodoEnCola { distancia, indice }) = cola.pop() { // Some sirve para desempaquetar, mientras tenga algo seguira el bucle
        if distancia > distancias[&grafo[indice]] {
            continue; // ya tenemos una mejor distancia
        }

        // Revisar vecinos
        for arista in grafo.edges(indice) {
            let vecino = arista.target();
            let peso = *arista.weight();
            let nueva_distancia = distancia + peso;

            let etiqueta_vecino = grafo[vecino].clone();
            if nueva_distancia < distancias[&etiqueta_vecino] {
                distancias.insert(etiqueta_vecino.clone(), nueva_distancia);
                cola.push(NodoEnCola {
                    distancia: nueva_distancia,
                    indice: vecino,
                });
            }
        }
    }

    distancias // retornar las distancias minimas
}



// Esto no es para presentar, solo es añadir al grafico, puro mecanismo y no logica del dijkstra
pub fn aplicar_dijkstra_y_etiquetar(grafo: &mut DiGraph<String, i32>, etiqueta_origen: &str) -> HashMap<String, i32> {
    // 1) Algoritmo para obtener las distancias
    let distancias = dijkstra(grafo, etiqueta_origen);

    // 2) Modificar el dígrafo "in-place": renombrar cada nodo con su distancia
    //    Ejemplo: "A" -> "A (d=0)" o "B" -> "B (d=7)" o "C" -> "C (d=inf)"
    let indices: Vec<_> = grafo.node_indices().collect();
    for idx in indices {
        let etiqueta_original = grafo[idx].clone();
        // OJO: buscamos la distancia por la etiqueta original (antes de modificarla)
        let d = distancias.get(&etiqueta_original).copied().unwrap_or(i32::MAX);
        let sufijo = if d == i32::MAX { "inf".to_string() } else { d.to_string() };
        grafo[idx] = format!("{} (d={})", etiqueta_original, sufijo);
    }

    distancias
}

pub fn dijkstra_con_predecesores(
    grafo: &DiGraph<String, i32>,
    etiqueta_origen: &str,
) -> (HashMap<String, i32>, HashMap<String, String>) {
    // Mapear etiqueta -> índice
    let mut indice_por_etiqueta = HashMap::new();
    for i in grafo.node_indices() {
        indice_por_etiqueta.insert(grafo[i].clone(), i);
    }
    let Some(origen_idx) = indice_por_etiqueta.get(etiqueta_origen) else {
        return (HashMap::new(), HashMap::new());
    };

    // Inicialización
    let mut distancia_minima: HashMap<String, i32> = HashMap::new();
    let mut predecesor: HashMap<String, String> = HashMap::new();
    for i in grafo.node_indices() {
        distancia_minima.insert(grafo[i].clone(), i32::MAX);
    }
    *distancia_minima.get_mut(etiqueta_origen).unwrap() = 0;

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct NodoEnCola {
        distancia: i32,
        indice: petgraph::prelude::NodeIndex,
    }
    impl Ord for NodoEnCola {
        fn cmp(&self, other: &Self) -> Ordering {
            other.distancia.cmp(&self.distancia)
        }
    }
    impl PartialOrd for NodoEnCola {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(NodoEnCola { distancia: 0, indice: *origen_idx });

    while let Some(NodoEnCola { distancia, indice }) = heap.pop() {
        let etiqueta_actual = grafo[indice].clone();
        if distancia > *distancia_minima.get(&etiqueta_actual).unwrap() {
            continue;
        }
        for arista in grafo.edges(indice) {
            let vecino = arista.target();
            let peso = *arista.weight();
            let etiqueta_vecino = grafo[vecino].clone();
            let nueva_dist = distancia.saturating_add(peso);
            if nueva_dist < *distancia_minima.get(&etiqueta_vecino).unwrap() {
                *distancia_minima.get_mut(&etiqueta_vecino).unwrap() = nueva_dist;
                predecesor.insert(etiqueta_vecino.clone(), etiqueta_actual.clone());
                heap.push(NodoEnCola { distancia: nueva_dist, indice: vecino });
            }
        }
    }

    (distancia_minima, predecesor)
}

/// Reconstruye la secuencia de etiquetas del camino mínimo [origen..destino].
pub fn reconstruir_camino(
    predecesor: &HashMap<String, String>,
    origen: &str,
    destino: &str,
) -> Vec<String> {
    let mut camino = Vec::new();
    let mut actual = destino.to_string();
    camino.push(actual.clone());

    while let Some(prev) = predecesor.get(&actual) {
        actual = prev.clone();
        camino.push(actual.clone());
        if actual == origen {
            break;
        }
    }

    if actual != origen {
        // No alcanzable
        return Vec::new();
    }

    camino.reverse();
    camino
}

