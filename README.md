# Visualizador de Grafos con Dijkstra 🪢

Proyecto en **Rust** que permite:

- Crear grafos manualmente o de manera automática.  
- Visualizar los nodos y aristas en una interfaz gráfica con **egui**.  
- Ejecutar el algoritmo de **Dijkstra** para calcular la ruta más corta.  
- Mostrar los resultados de manera interactiva con scroll y zoom.  

---

## 📚 Índice

1. [🚀 Cómo ejecutar el proyecto](#-cómo-ejecutar-el-proyecto)  
2. [🖼️ Funcionalidades principales](#️-funcionalidades-principales)  
3. [📦 Ejemplo de uso](#-ejemplo-de-uso)  
4. [🛠️ Requisitos](#️-requisitos)  
5. [📄 Licencia](#-licencia)  

---

## 🚀 Cómo ejecutar el proyecto

Clona el repositorio y compílalo con **cargo**:

git clone <repo>
cd <repo>
cargo run
Para compilarlo como ejecutable sin abrir consola (en Windows):

Generar ejecutable:
cargo build --release 

# 🖼️ Funcionalidades principales
✨ Crear y editar grafos en la interfaz.

📝 Agregar nodos y aristas con pesos personalizados.

🔍 Calcular la ruta más corta entre dos nodos con Dijkstra.

🎨 Visualización clara e interactiva gracias a egui.

# 📦 Ejemplo de uso
Abre la aplicación.

Crea un grafo manualmente (agregando nodos/aristas).

Ejecuta el algoritmo de Dijkstra seleccionando nodo de inicio y fin.

Visualiza en pantalla el camino más corto resaltado.

# 🛠️ Requisitos
Rust (versión estable recomendada).

Librerías utilizadas:

- eframe
- egui
- petgraph
