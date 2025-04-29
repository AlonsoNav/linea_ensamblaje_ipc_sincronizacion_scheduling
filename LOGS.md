# Logs de Ejecución Completa

## FCFS (First Come First Serve)

```
Seleccione el algoritmo de planificación:
1. FCFS (First Come First Serve)
2. Round Robin
Ingrese su elección (1 o 2):
1

--- Resumen Final ---

Producto 4:
  ➤ Tiempo de llegada: 0.311s
  Estación: Corte, Entrada: 0.312s, Salida: 1.317s
  Estación: Ensamblaje, Entrada: 1.317s, Salida: 2.822s
  Estación: Empaque, Entrada: 2.822s, Salida: 3.627s
  ➤ Tiempo de espera total: 0.000s
  ➤ Turnaround: 3.315s

Producto 2:
  ➤ Tiempo de llegada: 1.450s
  Estación: Corte, Entrada: 1.450s, Salida: 2.455s
  Estación: Ensamblaje, Entrada: 2.822s, Salida: 4.325s
  Estación: Empaque, Entrada: 4.325s, Salida: 5.128s
  ➤ Tiempo de espera total: 0.367s
  ➤ Turnaround: 3.679s

Producto 10:
  ➤ Tiempo de llegada: 1.973s
  Estación: Corte, Entrada: 2.455s, Salida: 3.460s
  Estación: Ensamblaje, Entrada: 4.325s, Salida: 5.830s
  Estación: Empaque, Entrada: 5.830s, Salida: 6.631s
  ➤ Tiempo de espera total: 1.347s
  ➤ Turnaround: 4.659s

Producto 3:
  ➤ Tiempo de llegada: 2.078s
  Estación: Corte, Entrada: 3.460s, Salida: 4.465s
  Estación: Ensamblaje, Entrada: 5.830s, Salida: 7.332s
  Estación: Empaque, Entrada: 7.332s, Salida: 8.137s
  ➤ Tiempo de espera total: 2.747s
  ➤ Turnaround: 6.060s

Producto 5:
  ➤ Tiempo de llegada: 3.122s
  Estación: Corte, Entrada: 4.465s, Salida: 5.470s
  Estación: Ensamblaje, Entrada: 7.332s, Salida: 8.835s
  Estación: Empaque, Entrada: 8.835s, Salida: 9.640s
  ➤ Tiempo de espera total: 3.206s
  ➤ Turnaround: 6.518s

Producto 6:
  ➤ Tiempo de llegada: 3.122s
  Estación: Corte, Entrada: 5.470s, Salida: 6.474s
  Estación: Ensamblaje, Entrada: 8.835s, Salida: 10.337s
  Estación: Empaque, Entrada: 10.337s, Salida: 11.141s
  ➤ Tiempo de espera total: 4.709s
  ➤ Turnaround: 8.020s

Producto 9:
  ➤ Tiempo de llegada: 3.226s
  Estación: Corte, Entrada: 6.474s, Salida: 7.476s
  Estación: Ensamblaje, Entrada: 10.337s, Salida: 11.838s
  Estación: Empaque, Entrada: 11.839s, Salida: 12.644s
  ➤ Tiempo de espera total: 6.110s
  ➤ Turnaround: 9.418s

Producto 1:
  ➤ Tiempo de llegada: 3.646s
  Estación: Corte, Entrada: 7.476s, Salida: 8.481s
  Estación: Ensamblaje, Entrada: 11.839s, Salida: 13.340s
  Estación: Empaque, Entrada: 13.340s, Salida: 14.145s
  ➤ Tiempo de espera total: 7.188s
  ➤ Turnaround: 10.499s

Producto 7:
  ➤ Tiempo de llegada: 3.751s
  Estación: Corte, Entrada: 8.481s, Salida: 9.486s
  Estación: Ensamblaje, Entrada: 13.340s, Salida: 14.845s
  Estación: Empaque, Entrada: 14.845s, Salida: 15.648s
  ➤ Tiempo de espera total: 8.584s
  ➤ Turnaround: 11.898s

Producto 8:
  ➤ Tiempo de llegada: 3.961s
  Estación: Corte, Entrada: 9.486s, Salida: 10.487s
  Estación: Ensamblaje, Entrada: 14.845s, Salida: 16.350s
  Estación: Empaque, Entrada: 16.350s, Salida: 17.151s
  ➤ Tiempo de espera total: 9.883s
  ➤ Turnaround: 13.190s

Promedio de espera: 4.414123300s
Promedio de turnaround: 7.725474900s

Orden final de procesamiento:
Producto 4
Producto 2
Producto 10
Producto 3
Producto 5
Producto 6
Producto 9
Producto 1
Producto 7
Producto 8
```

## Round Robin con quantum de 500

```
Seleccione el algoritmo de planificación:
1. FCFS (First Come First Serve)
2. Round Robin
Ingrese su elección (1 o 2):
2
Ingrese el quantum para Round Robin (milliseconds) o presione Enter para usar el valor predeterminado 500):
500


--- Resumen Final ---

Producto 9:
  ➤ Tiempo de llegada: 0.732s
  Estación: Corte, Entrada: 0.732s, Salida: 1.237s
  Estación: Corte, Entrada: 5.762s, Salida: 6.267s
  Estación: Ensamblaje, Entrada: 6.267s, Salida: 6.772s
  Estación: Ensamblaje, Entrada: 6.772s, Salida: 7.277s
  Estación: Ensamblaje, Entrada: 11.802s, Salida: 12.307s
  Estación: Empaque, Entrada: 12.307s, Salida: 12.811s
  Estación: Empaque, Entrada: 12.811s, Salida: 13.112s
  ➤ Tiempo de espera total: 9.050s
  ➤ Turnaround: 12.380s

Producto 4:
  ➤ Tiempo de llegada: 0.942s
  Estación: Corte, Entrada: 1.237s, Salida: 1.739s
  Estación: Corte, Entrada: 6.267s, Salida: 6.772s
  Estación: Ensamblaje, Entrada: 7.277s, Salida: 7.777s
  Estación: Ensamblaje, Entrada: 12.307s, Salida: 12.812s
  Estación: Ensamblaje, Entrada: 16.843s, Salida: 17.348s
  Estación: Empaque, Entrada: 17.348s, Salida: 17.853s
  Estación: Empaque, Entrada: 18.357s, Salida: 18.659s
  ➤ Tiempo de espera total: 14.392s
  ➤ Turnaround: 17.716s

Producto 6:
  ➤ Tiempo de llegada: 0.942s
  Estación: Corte, Entrada: 1.739s, Salida: 2.240s
  Estación: Corte, Entrada: 6.772s, Salida: 7.277s
  Estación: Ensamblaje, Entrada: 7.777s, Salida: 8.278s
  Estación: Ensamblaje, Entrada: 12.812s, Salida: 13.315s
  Estación: Ensamblaje, Entrada: 17.348s, Salida: 17.853s
  Estación: Empaque, Entrada: 17.853s, Salida: 18.357s
  Estación: Empaque, Entrada: 18.659s, Salida: 18.961s
  ➤ Tiempo de espera total: 14.699s
  ➤ Turnaround: 18.018s

Producto 3:
  ➤ Tiempo de llegada: 1.670s
  Estación: Corte, Entrada: 2.240s, Salida: 2.744s
  Estación: Corte, Entrada: 7.277s, Salida: 7.782s
  Estación: Ensamblaje, Entrada: 8.278s, Salida: 8.783s
  Estación: Ensamblaje, Entrada: 13.315s, Salida: 13.820s
  Estación: Ensamblaje, Entrada: 17.853s, Salida: 18.358s
  Estación: Empaque, Entrada: 18.961s, Salida: 19.463s
  Estación: Empaque, Entrada: 22.489s, Salida: 22.794s
  ➤ Tiempo de espera total: 17.792s
  ➤ Turnaround: 21.124s

Producto 8:
  ➤ Tiempo de llegada: 2.180s
  Estación: Corte, Entrada: 2.744s, Salida: 3.249s
  Estación: Corte, Entrada: 7.782s, Salida: 8.285s
  Estación: Ensamblaje, Entrada: 8.783s, Salida: 9.286s
  Estación: Ensamblaje, Entrada: 13.820s, Salida: 14.325s
  Estación: Ensamblaje, Entrada: 18.358s, Salida: 18.860s
  Estación: Empaque, Entrada: 19.463s, Salida: 19.968s
  Estación: Empaque, Entrada: 22.794s, Salida: 23.096s
  ➤ Tiempo de espera total: 17.590s
  ➤ Turnaround: 20.916s

Producto 7:
  ➤ Tiempo de llegada: 2.386s
  Estación: Corte, Entrada: 3.249s, Salida: 3.751s
  Estación: Corte, Entrada: 8.285s, Salida: 8.786s
  Estación: Ensamblaje, Entrada: 9.286s, Salida: 9.791s
  Estación: Ensamblaje, Entrada: 14.325s, Salida: 14.826s
  Estación: Ensamblaje, Entrada: 18.860s, Salida: 19.361s
  Estación: Empaque, Entrada: 19.968s, Salida: 20.473s
  Estación: Empaque, Entrada: 23.096s, Salida: 23.401s
  ➤ Tiempo de espera total: 17.695s
  ➤ Turnaround: 21.015s

Producto 10:
  ➤ Tiempo de llegada: 2.490s
  Estación: Corte, Entrada: 3.751s, Salida: 4.251s
  Estación: Corte, Entrada: 8.786s, Salida: 9.286s
  Estación: Ensamblaje, Entrada: 9.791s, Salida: 10.293s
  Estación: Ensamblaje, Entrada: 14.826s, Salida: 15.330s
  Estación: Ensamblaje, Entrada: 19.361s, Salida: 19.866s
  Estación: Empaque, Entrada: 20.473s, Salida: 20.975s
  Estación: Empaque, Entrada: 23.401s, Salida: 23.706s
  ➤ Tiempo de espera total: 17.898s
  ➤ Turnaround: 21.216s

Producto 1:
  ➤ Tiempo de llegada: 2.899s
  Estación: Corte, Entrada: 4.251s, Salida: 4.756s
  Estación: Corte, Entrada: 9.286s, Salida: 9.790s
  Estación: Ensamblaje, Entrada: 10.293s, Salida: 10.796s
  Estación: Ensamblaje, Entrada: 15.330s, Salida: 15.833s
  Estación: Ensamblaje, Entrada: 19.866s, Salida: 20.371s
  Estación: Empaque, Entrada: 20.975s, Salida: 21.479s
  Estación: Empaque, Entrada: 23.706s, Salida: 24.011s
  ➤ Tiempo de espera total: 17.782s
  ➤ Turnaround: 21.112s

Producto 5:
  ➤ Tiempo de llegada: 3.927s
  Estación: Corte, Entrada: 4.756s, Salida: 5.257s
  Estación: Corte, Entrada: 9.790s, Salida: 10.293s
  Estación: Ensamblaje, Entrada: 10.796s, Salida: 11.299s
  Estación: Ensamblaje, Entrada: 15.833s, Salida: 16.338s
  Estación: Ensamblaje, Entrada: 20.371s, Salida: 20.872s
  Estación: Empaque, Entrada: 21.479s, Salida: 21.984s
  Estación: Empaque, Entrada: 24.011s, Salida: 24.313s
  ➤ Tiempo de espera total: 17.068s
  ➤ Turnaround: 20.386s

Producto 2:
  ➤ Tiempo de llegada: 4.132s
  Estación: Corte, Entrada: 5.257s, Salida: 5.762s
  Estación: Corte, Entrada: 10.293s, Salida: 10.795s
  Estación: Ensamblaje, Entrada: 11.299s, Salida: 11.802s
  Estación: Ensamblaje, Entrada: 16.338s, Salida: 16.843s
  Estación: Ensamblaje, Entrada: 20.872s, Salida: 21.376s
  Estación: Empaque, Entrada: 21.984s, Salida: 22.489s
  Estación: Empaque, Entrada: 24.313s, Salida: 24.613s
  ➤ Tiempo de espera total: 17.157s
  ➤ Turnaround: 20.481s

Promedio de espera: 16.112436700s
Promedio de turnaround: 19.436410400s

Orden final de procesamiento:
Producto 9
Producto 4
Producto 6
Producto 3
Producto 8
Producto 7
Producto 10
Producto 1
Producto 5
Producto 2
```

## Round Robin con quantum de 1000

```
Seleccione el algoritmo de planificación:
1. FCFS (First Come First Serve)
2. Round Robin
Ingrese su elección (1 o 2):
2
Ingrese el quantum para Round Robin (milliseconds) o presione Enter para usar el valor predeterminado 500):
1000

--- Resumen Final ---
Producto 3:
  ➤ Tiempo de llegada: 0.410s
  Estación: Corte, Entrada: 0.410s, Salida: 1.413s
  Estación: Ensamblaje, Entrada: 1.413s, Salida: 2.416s
  Estación: Ensamblaje, Entrada: 2.416s, Salida: 2.921s
  Estación: Empaque, Entrada: 2.921s, Salida: 3.726s
  ➤ Tiempo de espera total: 0.000s
  ➤ Turnaround: 3.316s
Producto 10:
  ➤ Tiempo de llegada: 0.511s
  Estación: Corte, Entrada: 1.413s, Salida: 2.418s
  Estación: Ensamblaje, Entrada: 2.921s, Salida: 3.922s
  Estación: Ensamblaje, Entrada: 11.940s, Salida: 12.444s
  Estación: Empaque, Entrada: 12.444s, Salida: 13.247s
  ➤ Tiempo de espera total: 9.423s
  ➤ Turnaround: 12.736s
Producto 2:
  ➤ Tiempo de llegada: 0.616s
  Estación: Corte, Entrada: 2.418s, Salida: 3.419s
  Estación: Ensamblaje, Entrada: 3.922s, Salida: 4.926s
  Estación: Ensamblaje, Entrada: 12.444s, Salida: 12.949s
  Estación: Empaque, Entrada: 13.247s, Salida: 14.050s
  ➤ Tiempo de espera total: 10.120s
  ➤ Turnaround: 13.434s
Producto 1:
  ➤ Tiempo de llegada: 0.825s
  Estación: Corte, Entrada: 3.419s, Salida: 4.424s
  Estación: Ensamblaje, Entrada: 4.926s, Salida: 5.931s
  Estación: Ensamblaje, Entrada: 12.949s, Salida: 13.451s
  Estación: Empaque, Entrada: 14.050s, Salida: 14.855s
  ➤ Tiempo de espera total: 10.714s
  ➤ Turnaround: 14.030s
Producto 5:
  ➤ Tiempo de llegada: 1.954s
  Estación: Corte, Entrada: 4.424s, Salida: 5.428s
  Estación: Ensamblaje, Entrada: 5.931s, Salida: 6.932s
  Estación: Ensamblaje, Entrada: 13.451s, Salida: 13.952s
  Estación: Empaque, Entrada: 14.855s, Salida: 15.660s
  ➤ Tiempo de espera total: 10.395s
  ➤ Turnaround: 13.706s
Producto 7:
  ➤ Tiempo de llegada: 2.982s
  Estación: Corte, Entrada: 5.428s, Salida: 6.433s
  Estación: Ensamblaje, Entrada: 6.932s, Salida: 7.933s
  Estación: Ensamblaje, Entrada: 13.952s, Salida: 14.453s
  Estación: Empaque, Entrada: 15.660s, Salida: 16.465s
  ➤ Tiempo de espera total: 10.171s
  ➤ Turnaround: 13.483s
Producto 6:
  ➤ Tiempo de llegada: 3.393s
  Estación: Corte, Entrada: 6.433s, Salida: 7.434s
  Estación: Ensamblaje, Entrada: 7.933s, Salida: 8.935s
  Estación: Ensamblaje, Entrada: 14.453s, Salida: 14.958s
  Estación: Empaque, Entrada: 16.465s, Salida: 17.270s
  ➤ Tiempo de espera total: 10.564s
  ➤ Turnaround: 13.877s
Producto 4:
  ➤ Tiempo de llegada: 3.916s
  Estación: Corte, Entrada: 7.434s, Salida: 8.439s
  Estación: Ensamblaje, Entrada: 8.935s, Salida: 9.935s
  Estación: Ensamblaje, Entrada: 14.958s, Salida: 15.459s
  Estación: Empaque, Entrada: 17.270s, Salida: 18.075s
  ➤ Tiempo de espera total: 10.848s
  ➤ Turnaround: 14.159s
Producto 9:
  ➤ Tiempo de llegada: 4.020s
  Estación: Corte, Entrada: 8.439s, Salida: 9.441s
  Estación: Ensamblaje, Entrada: 9.935s, Salida: 10.935s
  Estación: Ensamblaje, Entrada: 15.459s, Salida: 15.964s
  Estación: Empaque, Entrada: 18.075s, Salida: 18.880s
  ➤ Tiempo de espera total: 11.548s
  ➤ Turnaround: 14.860s
Producto 8:
  ➤ Tiempo de llegada: 4.229s
  Estación: Corte, Entrada: 9.441s, Salida: 10.446s
  Estación: Ensamblaje, Entrada: 10.935s, Salida: 11.940s
  Estación: Ensamblaje, Entrada: 15.964s, Salida: 16.465s
  Estación: Empaque, Entrada: 18.880s, Salida: 19.681s
  ➤ Tiempo de espera total: 12.142s
  ➤ Turnaround: 15.452s
Promedio de espera: 9.592476900s
Promedio de turnaround: 12.905262000s
Orden final de procesamiento:
Producto 3
Producto 10
Producto 2
Producto 1
Producto 5
Producto 7
Producto 6
Producto 4
Producto 9
Producto 8
```


## Round Robin con quantum de 1500

```
Seleccione el algoritmo de planificación:
1. FCFS (First Come First Serve)
2. Round Robin
Ingrese su elección (1 o 2):
2
Ingrese el quantum para Round Robin (milliseconds) o presione Enter para usar el valor predeterminado 500):
1500

--- Resumen Final ---

Producto 8:
  ➤ Tiempo de llegada: 2.060s
  Estación: Corte, Entrada: 2.060s, Salida: 3.065s
  Estación: Ensamblaje, Entrada: 3.128s, Salida: 4.633s
  Estación: Empaque, Entrada: 4.633s, Salida: 5.434s
  ➤ Tiempo de espera total: 0.063s
  ➤ Turnaround: 3.374s

Producto 6:
  ➤ Tiempo de llegada: 2.060s
  Estación: Corte, Entrada: 3.065s, Salida: 4.066s
  Estación: Ensamblaje, Entrada: 4.633s, Salida: 6.138s
  Estación: Empaque, Entrada: 6.138s, Salida: 6.943s
  ➤ Tiempo de espera total: 1.572s
  ➤ Turnaround: 4.883s

Producto 2:
  ➤ Tiempo de llegada: 2.468s
  Estación: Corte, Entrada: 4.066s, Salida: 5.071s
  Estación: Ensamblaje, Entrada: 6.138s, Salida: 7.643s
  Estación: Empaque, Entrada: 7.653s, Salida: 8.454s
  ➤ Tiempo de espera total: 2.675s
  ➤ Turnaround: 5.986s

Producto 7:
  ➤ Tiempo de llegada: 2.573s
  Estación: Corte, Entrada: 5.071s, Salida: 6.073s
  Estación: Ensamblaje, Entrada: 7.643s, Salida: 9.145s
  Estación: Empaque, Entrada: 9.165s, Salida: 9.970s
  ➤ Tiempo de espera total: 4.088s
  ➤ Turnaround: 7.397s

Producto 5:
  ➤ Tiempo de llegada: 2.573s
  Estación: Corte, Entrada: 6.074s, Salida: 7.075s
  Estación: Ensamblaje, Entrada: 9.145s, Salida: 10.646s
  Estación: Empaque, Entrada: 10.682s, Salida: 11.486s
  ➤ Tiempo de espera total: 5.606s
  ➤ Turnaround: 8.913s

Producto 4:
  ➤ Tiempo de llegada: 3.195s
  Estación: Corte, Entrada: 7.075s, Salida: 8.080s
  Estación: Ensamblaje, Entrada: 10.646s, Salida: 12.148s
  Estación: Empaque, Entrada: 12.196s, Salida: 12.999s
  ➤ Tiempo de espera total: 6.495s
  ➤ Turnaround: 9.805s

Producto 9:
  ➤ Tiempo de llegada: 3.505s
  Estación: Corte, Entrada: 8.080s, Salida: 9.081s
  Estación: Ensamblaje, Entrada: 12.148s, Salida: 13.653s
  Estación: Empaque, Entrada: 13.704s, Salida: 14.509s
  ➤ Tiempo de espera total: 7.693s
  ➤ Turnaround: 11.004s

Producto 10:
  ➤ Tiempo de llegada: 4.861s
  Estación: Corte, Entrada: 9.081s, Salida: 10.086s
  Estación: Ensamblaje, Entrada: 13.653s, Salida: 15.155s
  Estación: Empaque, Entrada: 15.224s, Salida: 16.029s
  ➤ Tiempo de espera total: 7.855s
  ➤ Turnaround: 11.168s

Producto 1:
  ➤ Tiempo de llegada: 4.965s
  Estación: Corte, Entrada: 10.086s, Salida: 11.087s
  Estación: Ensamblaje, Entrada: 15.155s, Salida: 16.657s
  Estación: Empaque, Entrada: 16.743s, Salida: 17.548s
  ➤ Tiempo de espera total: 9.275s
  ➤ Turnaround: 12.582s

Producto 3:
  ➤ Tiempo de llegada: 4.965s
  Estación: Corte, Entrada: 11.087s, Salida: 12.087s
  Estación: Ensamblaje, Entrada: 16.657s, Salida: 18.161s
  Estación: Empaque, Entrada: 18.255s, Salida: 19.057s
  ➤ Tiempo de espera total: 10.786s
  ➤ Turnaround: 14.092s

Promedio de espera: 5.610891700s
Promedio de turnaround: 8.920495200s

Orden final de procesamiento:
Producto 8
Producto 6
Producto 2
Producto 7
Producto 5
Producto 4
Producto 9
Producto 10
Producto 1
Producto 3
```