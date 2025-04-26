# Informe de Línea de Ensamblaje con IPC, Sincronización y Algoritmos de Scheduling

## Descripción de la solución

El proyecto implementa una simulación de una línea de ensamblajes de productos utilizando el lenguaje de programación **Rust**. En esta línea, los productos pasan por varias estaciones (`Corte`, `Ensamblaje` y `Empaque`), donde cada estación tiene un tiempo de procesamiento definido.

La comunicación entre estaciones se realiza mediante **canales (`mpsc`)** y cada estación se ejecuta en un **hilo (`thread`)** separado, permitiendo procesamiento concurrente.

El sistema utiliza dos algoritmos de planificación para el manejo de los productos en cada estación:

- **First Come First Serve (FCFS)**: Los productos se procesan estrictamente en el orden en que llegan.
- **Round Robin (RR)**: Los productos se procesan en rebanadas de tiempo (`quantum`) definidas. Si no terminan en el quantum asignado, se vuelven a poner en la cola para continuar su procesamiento.

El usuario puede seleccionar el algoritmo de planificación al iniciar el programa y, en caso de elegir Round Robin, también puede configurar el valor del quantum.

Al final de la simulación, se calcula y muestra un resumen con
- El tiempo de llegada de cada producto.
- El tiempo de entrada y salida de cada producto en cada estación.
- El tiempo de espera total de cada producto.
- El tiempo de turnaround (tiempo total en el sistema) de cada producto.
- El promedio de tiempos de espera y turnaround para todos los productos.
- El orden final en que los productos terminaron su procesamiento.


## Justificación Técnica

- **Rust y concurrencia segura**: Se eligió Rust por su capacidad para manejar concurrencia de manera segura y eficiente. El uso de **Arc** y **Mutex** garantiza que el acceso compartido entre hilos sea seguro sin riesgos de condiciones de carrera.
  
- **Canales (`mpsc`)**: Se utilizaron canales para transmitir los productos entre las estaciones de forma asíncrona y segura, desacoplando así la lógica de procesamiento de cada estación.

- **Modelado limpio**: El diseño orientado a objetos mediante estructuras (`struct`) para `Station`, `Product`, y `ProcessingStep` facilita el mantenimiento y extensión futura del proyecto.

- **Algoritmos de planificación**:
  - **FCFS**: Es simple de implementar y sirve como baseline para evaluar el rendimiento del sistema.
  - **Round Robin**: Permite simular situaciones donde se necesita repartir el tiempo de procesamiento de manera más equitativa entre varios productos, siendo especialmente útil cuando hay productos que requieren tiempos largos de procesamiento.

- **Flexibilidad**: El sistema permite fácilmente cambiar o agregar nuevos algoritmos de planificación en el futuro, gracias a la separación de lógica según el tipo de algoritmo (`SchedulingAlgorithm`).

## Comparación entre algoritmos

| Criterio                    | FCFS                                               | Round Robin                                          |
|-------------------------------|----------------------------------------------------|------------------------------------------------------|
| **Orden de procesamiento**    | Estrictamente por orden de llegada.                | Basado en rebanadas de tiempo (`quantum`).           |
| **Tiempo de espera**          | Puede ser alto para productos que llegan tarde.    | Más equilibrado; productos largos no bloquean tanto. |
| **Equidad**                   | Baja: los primeros acaparan la estación.           | Alta: todos reciben atención periódicamente.         |
| **Complejidad de implementación** | Más sencillo.                                    | Más complejo (manejo de tiempo restante, reencolado). |
| **Uso recomendado**           | Cargas de trabajo homogéneas y simples.            | Cargas variadas, con productos de tiempos muy distintos. |

### Resultados esperados

- **FCFS** tiende a favorecer a los primeros productos y puede provocar largos tiempos de espera para los siguientes, especialmente si los primeros productos son muy grandes.
- **Round Robin** mejora la equidad, evitando que productos pequeños esperen demasiado, pero puede incrementar el overhead si el quantum no es bien elegido.




