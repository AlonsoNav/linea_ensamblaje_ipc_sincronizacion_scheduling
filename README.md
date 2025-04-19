# Línea de Ensamblaje con IPC, Sincronización y Scheduling

Este proyecto simula una línea de ensamblaje con múltiples estaciones de procesamiento, utilizando algoritmos de planificación como **FCFS** (First Come First Serve) y **Round Robin**. Se implementa comunicación entre estaciones mediante canales (IPC) y sincronización con `Mutex` y `Arc`.

## Requisitos

Antes de ejecutar el proyecto, asegúrate de tener instalado lo siguiente:

- [Rust](https://www.rust-lang.org/tools/install) (versión 1.65 o superior)
- Cargo (incluido con la instalación de Rust)

## Instrucciones para ejecutar

1. **Clona el repositorio**:
   ```bash
   git clone <URL_DEL_REPOSITORIO>
   cd linea_ensamblaje_ipc_sincronizacion_scheduling
   ```

2. **Compila el proyecto**:
    ```bash
    cargo build
    ```

3. **Ejecuta el proyecto**:
    ```bash
    cargo run
    ```

4. **Selecciona el algoritmo de planificación**:
Al ejecutar el programa, se te pedirá que selecciones un algoritmo de planificación que se aplicará a todas las estaciones:
- Ingresa 1 para FCFS
- Ingresa 2 para RR 
  
  Si seleccionas **Round Robin**, también se te pedirá que ingreses el valor del quantum (en milisegundos). Si presionas Enter sin ingresar un valor, se usará el valor predeterminado de 500ms.

5. **Observa los resultados**:
El programa mostrará un resumen final con:

- Informe individual de cada producto.
- Tiempo de espera total y promedio.
- Tiempo de turnaround total y promedio.
- Orden final de procesamiento de los productos.

## Estructura del proyecto
- **src/main.rs**:
Contiene la lógica principal del programa, incluyendo la inicialización de estaciones y productos.

- **src/models.rs**:
Define las estructuras principales como **Station**, **Product**, y **ProcessingStep**, así como los algoritmos de planificación (**FCFS** y **Round Robin**).
