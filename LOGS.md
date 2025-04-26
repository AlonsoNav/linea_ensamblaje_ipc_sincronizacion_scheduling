# Logs de Ejecución Completa

## FCFS (First Come First Serve)

```
Seleccione el algoritmo de planificación:
1. FCFS (First Come First Serve)
2. Round Robin
Ingrese su elección (1 o 2):
1

--- Resumen Final ---

Producto 8:
  Estación: Corte, Entrada: 0.602s, Salida: 1.603s
  Estación: Ensamblaje, Entrada: 1.603s, Salida: 3.103s
  Estación: Empaque, Entrada: 3.103s, Salida: 3.903s
  ➤ Tiempo de llegada: 0.602s
  ➤ Tiempo de espera total: 0.000s
  ➤ Turnaround: 3.301s

Producto 2:
  Estación: Corte, Entrada: 1.603s, Salida: 2.603s
  Estación: Ensamblaje, Entrada: 3.103s, Salida: 4.603s
  Estación: Empaque, Entrada: 4.603s, Salida: 5.404s
  ➤ Tiempo de llegada: 0.703s
  ➤ Tiempo de espera total: 0.500s
  ➤ Turnaround: 4.701s

Producto 10:
  Estación: Corte, Entrada: 2.603s, Salida: 3.604s
  Estación: Ensamblaje, Entrada: 4.603s, Salida: 6.104s
  Estación: Empaque, Entrada: 6.104s, Salida: 6.904s
  ➤ Tiempo de llegada: 1.103s
  ➤ Tiempo de espera total: 1.000s
  ➤ Turnaround: 5.801s

Producto 3:
  Estación: Corte, Entrada: 3.604s, Salida: 4.604s
  Estación: Ensamblaje, Entrada: 6.104s, Salida: 7.604s
  Estación: Empaque, Entrada: 7.604s, Salida: 8.405s
  ➤ Tiempo de llegada: 1.304s
  ➤ Tiempo de espera total: 1.500s
  ➤ Turnaround: 7.101s

Producto 1:
  Estación: Corte, Entrada: 4.604s, Salida: 5.604s
  Estación: Ensamblaje, Entrada: 7.604s, Salida: 9.105s
  Estación: Empaque, Entrada: 9.105s, Salida: 9.905s
  ➤ Tiempo de llegada: 1.304s
  ➤ Tiempo de espera total: 2.000s
  ➤ Turnaround: 8.601s

Producto 9:
  Estación: Corte, Entrada: 5.604s, Salida: 6.605s
  Estación: Ensamblaje, Entrada: 9.105s, Salida: 10.605s
  Estación: Empaque, Entrada: 10.605s, Salida: 11.406s
  ➤ Tiempo de llegada: 1.504s
  ➤ Tiempo de espera total: 2.500s
  ➤ Turnaround: 9.902s

Producto 5:
  Estación: Corte, Entrada: 6.605s, Salida: 7.605s
  Estación: Ensamblaje, Entrada: 10.605s, Salida: 12.106s
  Estación: Empaque, Entrada: 12.106s, Salida: 12.906s
  ➤ Tiempo de llegada: 2.105s
  ➤ Tiempo de espera total: 3.000s
  ➤ Turnaround: 10.800s

Producto 6:
  Estación: Corte, Entrada: 7.605s, Salida: 8.606s
  Estación: Ensamblaje, Entrada: 12.106s, Salida: 13.606s
  Estación: Empaque, Entrada: 13.606s, Salida: 14.406s
  ➤ Tiempo de llegada: 4.110s
  ➤ Tiempo de espera total: 3.500s
  ➤ Turnaround: 10.296s

Producto 4:
  Estación: Corte, Entrada: 8.606s, Salida: 9.606s
  Estación: Ensamblaje, Entrada: 13.606s, Salida: 15.106s
  Estación: Empaque, Entrada: 15.106s, Salida: 15.906s
  ➤ Tiempo de llegada: 5.013s
  ➤ Tiempo de espera total: 4.000s
  ➤ Turnaround: 10.894s

Producto 7:
  Estación: Corte, Entrada: 9.606s, Salida: 10.606s
  Estación: Ensamblaje, Entrada: 15.106s, Salida: 16.607s
  Estación: Empaque, Entrada: 16.607s, Salida: 17.407s
  ➤ Tiempo de llegada: 5.013s
  ➤ Tiempo de espera total: 4.500s
  ➤ Turnaround: 12.395s

Promedio de espera: 2.250011740s
Promedio de turnaround: 8.379172840s

Orden final de procesamiento:
Producto 8
Producto 2
Producto 10
Producto 3
Producto 1
Producto 9
Producto 5
Producto 6
Producto 4
Producto 7
```

## Round Robin

```
Seleccione el algoritmo de planificación:
1. FCFS (First Come First Serve)
2. Round Robin
Ingrese su elección (1 o 2):
2
Ingrese el quantum para Round Robin (milliseconds) o presione Enter para usar el valor predeterminado 500):
500
Recibido nuevo producto 4 en cola RR (bloqueante)
Estación Corte procesando producto 4 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 4 a la cola, tiempo restante: 500ms
Estación Corte procesando producto 4 por 500ms (tiempo restante: 500ms)
Estación Corte terminó de procesar producto 4
Recibido nuevo producto 4 en cola RR (bloqueante)
Estación Ensamblaje procesando producto 4 por 500ms (tiempo restante: 1500ms)
Devolviendo producto 4 a la cola, tiempo restante: 1000ms
Estación Ensamblaje procesando producto 4 por 500ms (tiempo restante: 1000ms)
Recibido nuevo producto 1 en cola RR (bloqueante)
Estación Corte procesando producto 1 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 4 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 4 por 500ms (tiempo restante: 500ms)
Devolviendo producto 1 a la cola, tiempo restante: 500ms
Añadiendo producto 10 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 1 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 4
Recibido nuevo producto 4 en cola RR (bloqueante)
Estación Empaque procesando producto 4 por 500ms (tiempo restante: 800ms)
Estación Corte terminó de procesar producto 1
Añadiendo producto 5 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 10 por 500ms (tiempo restante: 1000ms)
Recibido nuevo producto 1 en cola RR (bloqueante)
Estación Ensamblaje procesando producto 1 por 500ms (tiempo restante: 1500ms)
Devolviendo producto 4 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 4 por 300ms (tiempo restante: 300ms)
Devolviendo producto 10 a la cola, tiempo restante: 500ms
Estación Corte procesando producto 5 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 1 a la cola, tiempo restante: 1000ms
Estación Ensamblaje procesando producto 1 por 500ms (tiempo restante: 1000ms)
Estación Empaque terminó de procesar producto 4
Devolviendo producto 5 a la cola, tiempo restante: 500ms
Devolviendo producto 1 a la cola, tiempo restante: 500ms
Añadiendo producto 6 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 10 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje procesando producto 1 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 1
Estación Corte terminó de procesar producto 10
Recibido nuevo producto 1 en cola RR (bloqueante)
Estación Empaque procesando producto 1 por 500ms (tiempo restante: 800ms)
Añadiendo producto 3 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 5 por 500ms (tiempo restante: 500ms)
Recibido nuevo producto 10 en cola RR (bloqueante)
Estación Ensamblaje procesando producto 10 por 500ms (tiempo restante: 1500ms)
Devolviendo producto 1 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 1 por 300ms (tiempo restante: 300ms)
Estación Corte terminó de procesar producto 5
Añadiendo producto 7 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 6 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 10 a la cola, tiempo restante: 1000ms
Añadiendo producto 5 a la cola RR, tiempo restante: 1500ms
Estación Ensamblaje procesando producto 10 por 500ms (tiempo restante: 1000ms)
Estación Empaque terminó de procesar producto 1
Devolviendo producto 6 a la cola, tiempo restante: 500ms
Añadiendo producto 9 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 3 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 10 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 5 por 500ms (tiempo restante: 1500ms)
Devolviendo producto 3 a la cola, tiempo restante: 500ms
Añadiendo producto 2 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 7 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 5 a la cola, tiempo restante: 1000ms
Estación Ensamblaje procesando producto 10 por 500ms (tiempo restante: 500ms)
Devolviendo producto 7 a la cola, tiempo restante: 500ms
Añadiendo producto 8 a la cola RR, tiempo restante: 1000ms
Estación Corte procesando producto 6 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 10
Estación Ensamblaje procesando producto 5 por 500ms (tiempo restante: 1000ms)
Recibido nuevo producto 10 en cola RR (bloqueante)
Estación Empaque procesando producto 10 por 500ms (tiempo restante: 800ms)
Estación Corte terminó de procesar producto 6
Estación Corte procesando producto 9 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 5 a la cola, tiempo restante: 500ms
Añadiendo producto 6 a la cola RR, tiempo restante: 1500ms
Estación Ensamblaje procesando producto 5 por 500ms (tiempo restante: 500ms)
Devolviendo producto 10 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 10 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 10
Devolviendo producto 9 a la cola, tiempo restante: 500ms
Estación Corte procesando producto 3 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 5
Estación Ensamblaje procesando producto 6 por 500ms (tiempo restante: 1500ms)
Recibido nuevo producto 5 en cola RR (bloqueante)
Estación Empaque procesando producto 5 por 500ms (tiempo restante: 800ms)
Estación Corte terminó de procesar producto 3
Estación Corte procesando producto 2 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 6 a la cola, tiempo restante: 1000ms
Añadiendo producto 3 a la cola RR, tiempo restante: 1500ms
Estación Ensamblaje procesando producto 6 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 5 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 5 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 5
Devolviendo producto 2 a la cola, tiempo restante: 500ms
Estación Corte procesando producto 7 por 500ms (tiempo restante: 500ms)
Devolviendo producto 6 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 3 por 500ms (tiempo restante: 1500ms)
Estación Corte terminó de procesar producto 7
Estación Corte procesando producto 8 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 3 a la cola, tiempo restante: 1000ms
Añadiendo producto 7 a la cola RR, tiempo restante: 1500ms
Estación Ensamblaje procesando producto 6 por 500ms (tiempo restante: 500ms)
Devolviendo producto 8 a la cola, tiempo restante: 500ms
Estación Corte procesando producto 9 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 6
Estación Ensamblaje procesando producto 3 por 500ms (tiempo restante: 1000ms)
Recibido nuevo producto 6 en cola RR (bloqueante)
Estación Empaque procesando producto 6 por 500ms (tiempo restante: 800ms)
Estación Corte terminó de procesar producto 9
Estación Corte procesando producto 2 por 500ms (tiempo restante: 500ms)
Devolviendo producto 3 a la cola, tiempo restante: 500ms
Añadiendo producto 9 a la cola RR, tiempo restante: 1500ms
Devolviendo producto 6 a la cola, tiempo restante: 300ms
Estación Ensamblaje procesando producto 7 por 500ms (tiempo restante: 1500ms)
Estación Empaque procesando producto 6 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 6
Estación Corte terminó de procesar producto 2
Estación Corte procesando producto 8 por 500ms (tiempo restante: 500ms)
Devolviendo producto 7 a la cola, tiempo restante: 1000ms
Añadiendo producto 2 a la cola RR, tiempo restante: 1500ms
Estación Ensamblaje procesando producto 3 por 500ms (tiempo restante: 500ms)
Estación Corte terminó de procesar producto 8
Estación Ensamblaje terminó de procesar producto 3
Añadiendo producto 8 a la cola RR, tiempo restante: 1500ms
Estación Ensamblaje procesando producto 9 por 500ms (tiempo restante: 1500ms)
Recibido nuevo producto 3 en cola RR (bloqueante)
Estación Empaque procesando producto 3 por 500ms (tiempo restante: 800ms)
Devolviendo producto 9 a la cola, tiempo restante: 1000ms
Estación Ensamblaje procesando producto 7 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 3 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 3 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 3
Devolviendo producto 7 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 2 por 500ms (tiempo restante: 1500ms)
Devolviendo producto 2 a la cola, tiempo restante: 1000ms
Estación Ensamblaje procesando producto 8 por 500ms (tiempo restante: 1500ms)
Devolviendo producto 8 a la cola, tiempo restante: 1000ms
Estación Ensamblaje procesando producto 9 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 9 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 7 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 7
Estación Ensamblaje procesando producto 2 por 500ms (tiempo restante: 1000ms)
Recibido nuevo producto 7 en cola RR (bloqueante)
Estación Empaque procesando producto 7 por 500ms (tiempo restante: 800ms)
Devolviendo producto 2 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 8 por 500ms (tiempo restante: 1000ms)
Devolviendo producto 7 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 7 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 7
Devolviendo producto 8 a la cola, tiempo restante: 500ms
Estación Ensamblaje procesando producto 9 por 500ms (tiempo restante: 500ms)
Estación Ensamblaje terminó de procesar producto 9
Estación Ensamblaje procesando producto 2 por 500ms (tiempo restante: 500ms)
Recibido nuevo producto 9 en cola RR (bloqueante)
Estación Empaque procesando producto 9 por 500ms (tiempo restante: 800ms)
Estación Ensamblaje terminó de procesar producto 2
Estación Ensamblaje procesando producto 8 por 500ms (tiempo restante: 500ms)
Devolviendo producto 9 a la cola, tiempo restante: 300ms
Añadiendo producto 2 a la cola RR, tiempo restante: 800ms
Estación Empaque procesando producto 9 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 9
Estación Empaque procesando producto 2 por 500ms (tiempo restante: 800ms)
Estación Ensamblaje terminó de procesar producto 8
Devolviendo producto 2 a la cola, tiempo restante: 300ms
Añadiendo producto 8 a la cola RR, tiempo restante: 800ms
Estación Empaque procesando producto 2 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 2
Estación Empaque procesando producto 8 por 500ms (tiempo restante: 800ms)
Devolviendo producto 8 a la cola, tiempo restante: 300ms
Estación Empaque procesando producto 8 por 300ms (tiempo restante: 300ms)
Estación Empaque terminó de procesar producto 8

--- Resumen Final ---

Producto 4:
  Estación: Corte, Entrada: 0.101s, Salida: 0.601s
  Estación: Corte, Entrada: 0.601s, Salida: 1.101s
  Estación: Ensamblaje, Entrada: 1.102s, Salida: 1.602s
  Estación: Ensamblaje, Entrada: 1.603s, Salida: 2.104s
  Estación: Ensamblaje, Entrada: 2.104s, Salida: 2.605s
  Estación: Empaque, Entrada: 2.605s, Salida: 3.106s
  Estación: Empaque, Entrada: 3.106s, Salida: 3.406s
  ➤ Tiempo de llegada: 0.100s
  ➤ Tiempo de espera total: 0.003s
  ➤ Turnaround: 3.306s

Producto 1:
  Estación: Corte, Entrada: 1.803s, Salida: 2.303s
  Estación: Corte, Entrada: 2.303s, Salida: 2.804s
  Estación: Ensamblaje, Entrada: 2.804s, Salida: 3.305s
  Estación: Ensamblaje, Entrada: 3.305s, Salida: 3.805s
  Estación: Ensamblaje, Entrada: 3.806s, Salida: 4.306s
  Estación: Empaque, Entrada: 4.307s, Salida: 4.807s
  Estación: Empaque, Entrada: 4.807s, Salida: 5.108s
  ➤ Tiempo de llegada: 1.705s
  ➤ Tiempo de espera total: 0.003s
  ➤ Turnaround: 3.403s

Producto 10:
  Estación: Corte, Entrada: 2.804s, Salida: 3.304s
  Estación: Corte, Entrada: 3.806s, Salida: 4.306s
  Estación: Ensamblaje, Entrada: 4.407s, Salida: 4.907s
  Estación: Ensamblaje, Entrada: 4.908s, Salida: 5.408s
  Estación: Ensamblaje, Entrada: 5.909s, Salida: 6.409s
  Estación: Empaque, Entrada: 6.411s, Salida: 6.911s
  Estación: Empaque, Entrada: 6.911s, Salida: 7.211s
  ➤ Tiempo de llegada: 2.206s
  ➤ Tiempo de espera total: 1.105s
  ➤ Turnaround: 5.005s

Producto 5:
  Estación: Corte, Entrada: 3.305s, Salida: 3.805s
  Estación: Corte, Entrada: 4.307s, Salida: 4.808s
  Estación: Ensamblaje, Entrada: 5.408s, Salida: 5.908s
  Estación: Ensamblaje, Entrada: 6.409s, Salida: 6.909s
  Estación: Ensamblaje, Entrada: 6.910s, Salida: 7.410s
  Estación: Empaque, Entrada: 7.410s, Salida: 7.911s
  Estación: Empaque, Entrada: 7.911s, Salida: 8.211s
  ➤ Tiempo de llegada: 2.406s
  ➤ Tiempo de espera total: 1.605s
  ➤ Turnaround: 5.805s

Producto 6:
  Estación: Corte, Entrada: 4.808s, Salida: 5.308s
  Estación: Corte, Entrada: 6.309s, Salida: 6.810s
  Estación: Ensamblaje, Entrada: 7.410s, Salida: 7.910s
  Estación: Ensamblaje, Entrada: 7.911s, Salida: 8.411s
  Estación: Ensamblaje, Entrada: 8.912s, Salida: 9.412s
  Estación: Empaque, Entrada: 9.412s, Salida: 9.912s
  Estación: Empaque, Entrada: 9.913s, Salida: 10.213s
  ➤ Tiempo de llegada: 3.509s
  ➤ Tiempo de espera total: 2.104s
  ➤ Turnaround: 6.704s

Producto 3:
  Estación: Corte, Entrada: 5.309s, Salida: 5.809s
  Estación: Corte, Entrada: 7.310s, Salida: 7.811s
  Estación: Ensamblaje, Entrada: 8.411s, Salida: 8.911s
  Estación: Ensamblaje, Entrada: 9.412s, Salida: 9.912s
  Estación: Ensamblaje, Entrada: 10.413s, Salida: 10.913s
  Estación: Empaque, Entrada: 10.915s, Salida: 11.415s
  Estación: Empaque, Entrada: 11.415s, Salida: 11.715s
  ➤ Tiempo de llegada: 3.709s
  ➤ Tiempo de espera total: 3.105s
  ➤ Turnaround: 8.006s

Producto 7:
  Estación: Corte, Entrada: 5.809s, Salida: 6.309s
  Estación: Corte, Entrada: 8.312s, Salida: 8.812s
  Estación: Ensamblaje, Entrada: 9.913s, Salida: 10.413s
  Estación: Ensamblaje, Entrada: 11.414s, Salida: 11.914s
  Estación: Ensamblaje, Entrada: 13.417s, Salida: 13.917s
  Estación: Empaque, Entrada: 13.918s, Salida: 14.418s
  Estación: Empaque, Entrada: 14.419s, Salida: 14.719s
  ➤ Tiempo de llegada: 3.910s
  ➤ Tiempo de espera total: 5.608s
  ➤ Turnaround: 10.809s

Producto 9:
  Estación: Corte, Entrada: 6.810s, Salida: 7.310s
  Estación: Corte, Entrada: 9.312s, Salida: 9.813s
  Estación: Ensamblaje, Entrada: 10.914s, Salida: 11.414s
  Estación: Ensamblaje, Entrada: 12.916s, Salida: 13.417s
  Estación: Ensamblaje, Entrada: 14.919s, Salida: 15.419s
  Estación: Empaque, Entrada: 15.423s, Salida: 15.923s
  Estación: Empaque, Entrada: 15.924s, Salida: 16.224s
  ➤ Tiempo de llegada: 4.211s
  ➤ Tiempo de espera total: 6.113s
  ➤ Turnaround: 12.013s

Producto 2:
  Estación: Corte, Entrada: 7.811s, Salida: 8.311s
  Estación: Corte, Entrada: 9.813s, Salida: 10.313s
  Estación: Ensamblaje, Entrada: 11.915s, Salida: 12.415s
  Estación: Ensamblaje, Entrada: 13.918s, Salida: 14.418s
  Estación: Ensamblaje, Entrada: 15.419s, Salida: 15.920s
  Estación: Empaque, Entrada: 16.224s, Salida: 16.725s
  Estación: Empaque, Entrada: 16.725s, Salida: 17.025s
  ➤ Tiempo de llegada: 5.013s
  ➤ Tiempo de espera total: 5.912s
  ➤ Turnaround: 12.012s

Producto 8:
  Estación: Corte, Entrada: 8.812s, Salida: 9.312s
  Estación: Corte, Entrada: 10.313s, Salida: 10.814s
  Estación: Ensamblaje, Entrada: 12.415s, Salida: 12.916s
  Estación: Ensamblaje, Entrada: 14.418s, Salida: 14.919s
  Estación: Ensamblaje, Entrada: 15.920s, Salida: 16.420s
  Estación: Empaque, Entrada: 17.026s, Salida: 17.526s
  Estación: Empaque, Entrada: 17.526s, Salida: 17.826s
  ➤ Tiempo de llegada: 5.013s
  ➤ Tiempo de espera total: 5.713s
  ➤ Turnaround: 12.814s

Promedio de espera: 3.127168470s
Promedio de turnaround: 7.987670220s

Orden final de procesamiento:
Producto 4
Producto 1
Producto 10
Producto 5
Producto 6
Producto 3
Producto 7
Producto 9
Producto 2
Producto 8
```