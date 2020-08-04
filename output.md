# Output from running with different algorithms

We includede 3 test files:

- test1.txt: The first file the teacher provided us
- test2.txt: A file with random numbers that fills the whole memory
- test3.txt: The last file the teacher provided us and that we had to check our program against

## Final F instruction

The whole output is below, but if you only want to see the final F instruction here is it:

| Procesos | Turnaround FIFO | Swap-ins FIFO | Swap-outs FIFO | Turnaround LRU | Swap-ins LRU | Swap-outs LRU |
| -------- | --------------- | ------------- | -------------- | -------------- | ------------ | ------------- |
| 3        | 4.3s            | 0             | 0              | 4.3s           | 0            | 0             |
| 2        | 137.8s          | 0             | 2              | 137.8s         | 0            | 1             |
| 5        | 127.2s          | 0             | 0              | 126.2s         | 0            | 0             |
| 4        | 132.9s          | 1             | 3              | 131.9s         | 1            | 3             |
| 6        | 13.2s           | 0             | 0              | 12.2s          | 0            | 0             |
| 61       | 3.5s            | 0             | 0              | 2.5s           | 0            | 0             |
| 109      | 30.7s           | 0             | 0              | 29.7s          | 0            | 0             |

## FIFO algorithm (ran as `cargo run fifo test3.txt`)

```
C archivo de prueba para FIFO, LRU
La instrucción tomó 0s

P 32 2
Asignar 32 bytes al proceso 2
Se asignaron 32 bytes (2 páginas) al proceso 2
La instrucción tomó 2s

P 48 3
Asignar 48 bytes al proceso 3
Se asignaron 48 bytes (3 páginas) al proceso 3
La instrucción tomó 3s

P 63 4
Asignar 63 bytes al proceso 4
Se asignaron 63 bytes (4 páginas) al proceso 4
La instrucción tomó 4s

L 3
Liberar los marcos de página ocupados por el proceso 3
Se liberan de la memoria real: 2 a 4
La instrucción tomó 0.3s

P 80 5
Asignar 80 bytes al proceso 5
Se asignaron 80 bytes (5 páginas) al proceso 5
La instrucción tomó 5s

P 1744 109
Asignar 1744 bytes al proceso 109
Se asignaron 1744 bytes (109 páginas) al proceso 109
La instrucción tomó 109s

P 96 6
Asignar 96 bytes al proceso 6
Se asignaron 96 bytes (6 páginas) al proceso 6
La instrucción tomó 6s

A 16 2 0
Obtener la dirección real correspondiente a la dirección virtual 16 del proceso 2
Se accedió a la dirección 16 del proceso 2 (página 1)
Esta dirección corresponde a la dirección 16 en la memoria real (marco de página 1)
La instrucción tomó 0.1s

A 63 4 0
Obtener la dirección real correspondiente a la dirección virtual 63 del proceso 4
Error: El proceso 4 no contiene la dirección virtual 63

A 62 4 1
Obtener la dirección real correspondiente a la dirección virtual 62 del proceso 4 y modificar dicha dirección
Se modificó la dirección 62 del proceso 4 (página 3)
Esta dirección corresponde a la dirección 142 en la memoria real (marco de página 8)
La instrucción tomó 0.1s

A 1 109 1
Obtener la dirección real correspondiente a la dirección virtual 1 del proceso 109 y modificar dicha dirección
Se modificó la dirección 1 del proceso 109 (página 0)
Esta dirección corresponde a la dirección 177 en la memoria real (marco de página 11)
La instrucción tomó 0.1s

C
La instrucción tomó 0s

P 96 61
Asignar 96 bytes al proceso 61
Se asignaron 96 bytes (6 páginas) al proceso 61
Swap out de páginas del proceso 2: 0 a 1
Swap out de páginas del proceso 4: 0 a 1
La instrucción tomó 10s

L 2
Liberar los marcos de página ocupados por el proceso 2
Se liberan del espacio swap: 0 a 1
La instrucción tomó 0.2s

A 8 5 0
Obtener la dirección real correspondiente a la dirección virtual 8 del proceso 5
Se accedió a la dirección 8 del proceso 5 (página 0)
Esta dirección corresponde a la dirección 40 en la memoria real (marco de página 2)
La instrucción tomó 0.1s

A 8 4 0
Obtener la dirección real correspondiente a la dirección virtual 8 del proceso 4
Swap out de la página 2 del proceso 4
Swap in de la página 0 del proceso 4
Se accedió a la dirección 8 del proceso 4 (página 0)
Esta dirección corresponde a la dirección 120 en la memoria real (marco de página 7)
La instrucción tomó 1.1s

L 5
Liberar los marcos de página ocupados por el proceso 5
Se liberan de la memoria real: 2 a 4, 9 a 10
La instrucción tomó 0.5s

L 2
Liberar los marcos de página ocupados por el proceso 2
Error: No existe un proceso ejecutándose con el pid 2

L 4
Liberar los marcos de página ocupados por el proceso 4
Se liberan de la memoria real: 7 a 8
Se liberan del espacio swap: 2 a 3
La instrucción tomó 0.4s

L 6
Liberar los marcos de página ocupados por el proceso 6
Se liberan de la memoria real: 120 a 125
La instrucción tomó 0.6s

L 61
Liberar los marcos de página ocupados por el proceso 61
Se liberan de la memoria real: 0 a 1, 5 a 6, 126 a 127
La instrucción tomó 0.6s

L 109
Liberar los marcos de página ocupados por el proceso 109
Se liberan de la memoria real: 11 a 119
La instrucción tomó 10.9s

F
Fin. Reporte de salida:
Turnaround de cada proceso:
	Proceso 3:	5s - 9.3s       	4.3s de turnaround
	Proceso 2:	2s - 139.8s     	137.8s de turnaround
	Proceso 5:	14.3s - 141.5s  	127.2s de turnaround
	Proceso 4:	9s - 141.9s     	132.9s de turnaround
	Proceso 6:	129.3s - 142.5s 	13.2s de turnaround
	Proceso 61:	139.6s - 143.1s 	3.5s de turnaround
	Proceso 109:	123.3s - 154s   	30.7s de turnaround
Turnaround promedio: 64.22857142857143 segundos
Swaps por proceso:
	Proceso 3:	0 swap-ins,	0 swap-outs
	Proceso 2:	0 swap-ins,	2 swap-outs
	Proceso 5:	0 swap-ins,	0 swap-outs
	Proceso 4:	1 swap-ins,	3 swap-outs
	Proceso 6:	0 swap-ins,	0 swap-outs
	Proceso 61:	0 swap-ins,	0 swap-outs
	Proceso 109:	0 swap-ins,	0 swap-outs
La instrucción tomó 0s

A 2 2 0
Obtener la dirección real correspondiente a la dirección virtual 2 del proceso 2
Error: No existe un proceso ejecutándose con el pid 2

P 2049 2049
Asignar 2049 bytes al proceso 2049
Error: El tamaño del proceso (2049 bytes) es mayor al de la memoria real (2048 bytes)

F
Fin. Reporte de salida:
Turnaround de cada proceso:
	Proceso 3:	5s - 9.3s       	4.3s de turnaround
	Proceso 2:	2s - 139.8s     	137.8s de turnaround
	Proceso 5:	14.3s - 141.5s  	127.2s de turnaround
	Proceso 4:	9s - 141.9s     	132.9s de turnaround
	Proceso 6:	129.3s - 142.5s 	13.2s de turnaround
	Proceso 61:	139.6s - 143.1s 	3.5s de turnaround
	Proceso 109:	123.3s - 154s   	30.7s de turnaround
Turnaround promedio: 64.22857142857143 segundos
Swaps por proceso:
	Proceso 3:	0 swap-ins,	0 swap-outs
	Proceso 2:	0 swap-ins,	2 swap-outs
	Proceso 5:	0 swap-ins,	0 swap-outs
	Proceso 4:	1 swap-ins,	3 swap-outs
	Proceso 6:	0 swap-ins,	0 swap-outs
	Proceso 61:	0 swap-ins,	0 swap-outs
	Proceso 109:	0 swap-ins,	0 swap-outs
La instrucción tomó 0s

E
La instrucción tomó 0s
```

## LRU algorithm (ran as `cargo run lru test3.txt`)

```
C archivo de prueba para FIFO, LRU
La instrucción tomó 0s

P 32 2
Asignar 32 bytes al proceso 2
Se asignaron 32 bytes (2 páginas) al proceso 2
La instrucción tomó 2s

P 48 3
Asignar 48 bytes al proceso 3
Se asignaron 48 bytes (3 páginas) al proceso 3
La instrucción tomó 3s

P 63 4
Asignar 63 bytes al proceso 4
Se asignaron 63 bytes (4 páginas) al proceso 4
La instrucción tomó 4s

L 3
Liberar los marcos de página ocupados por el proceso 3
Se liberan de la memoria real: 2 a 4
La instrucción tomó 0.3s

P 80 5
Asignar 80 bytes al proceso 5
Se asignaron 80 bytes (5 páginas) al proceso 5
La instrucción tomó 5s

P 1744 109
Asignar 1744 bytes al proceso 109
Se asignaron 1744 bytes (109 páginas) al proceso 109
La instrucción tomó 109s

P 96 6
Asignar 96 bytes al proceso 6
Se asignaron 96 bytes (6 páginas) al proceso 6
La instrucción tomó 6s

A 16 2 0
Obtener la dirección real correspondiente a la dirección virtual 16 del proceso 2
Se accedió a la dirección 16 del proceso 2 (página 1)
Esta dirección corresponde a la dirección 16 en la memoria real (marco de página 1)
La instrucción tomó 0.1s

A 63 4 0
Obtener la dirección real correspondiente a la dirección virtual 63 del proceso 4
Error: El proceso 4 no contiene la dirección virtual 63

A 62 4 1
Obtener la dirección real correspondiente a la dirección virtual 62 del proceso 4 y modificar dicha dirección
Se modificó la dirección 62 del proceso 4 (página 3)
Esta dirección corresponde a la dirección 142 en la memoria real (marco de página 8)
La instrucción tomó 0.1s

A 1 109 1
Obtener la dirección real correspondiente a la dirección virtual 1 del proceso 109 y modificar dicha dirección
Se modificó la dirección 1 del proceso 109 (página 0)
Esta dirección corresponde a la dirección 177 en la memoria real (marco de página 11)
La instrucción tomó 0.1s

C
La instrucción tomó 0s

P 96 61
Asignar 96 bytes al proceso 61
Se asignaron 96 bytes (6 páginas) al proceso 61
Swap out de páginas del proceso 2: 0
Swap out de páginas del proceso 4: 0 a 2
La instrucción tomó 10s

L 2
Liberar los marcos de página ocupados por el proceso 2
Se liberan de la memoria real: 1
Se liberan del espacio swap: 0
La instrucción tomó 0.2s

A 8 5 0
Obtener la dirección real correspondiente a la dirección virtual 8 del proceso 5
Se accedió a la dirección 8 del proceso 5 (página 0)
Esta dirección corresponde a la dirección 40 en la memoria real (marco de página 2)
La instrucción tomó 0.1s

A 8 4 0
Obtener la dirección real correspondiente a la dirección virtual 8 del proceso 4
Swap in de la página 0 del proceso 4
Se accedió a la dirección 8 del proceso 4 (página 0)
Esta dirección corresponde a la dirección 24 en la memoria real (marco de página 1)
La instrucción tomó 0.1s

L 5
Liberar los marcos de página ocupados por el proceso 5
Se liberan de la memoria real: 2 a 4, 9 a 10
La instrucción tomó 0.5s

L 2
Liberar los marcos de página ocupados por el proceso 2
Error: No existe un proceso ejecutándose con el pid 2

L 4
Liberar los marcos de página ocupados por el proceso 4
Se liberan de la memoria real: 1, 8
Se liberan del espacio swap: 2 a 3
La instrucción tomó 0.4s

L 6
Liberar los marcos de página ocupados por el proceso 6
Se liberan de la memoria real: 120 a 125
La instrucción tomó 0.6s

L 61
Liberar los marcos de página ocupados por el proceso 61
Se liberan de la memoria real: 0, 5 a 7, 126 a 127
La instrucción tomó 0.6s

L 109
Liberar los marcos de página ocupados por el proceso 109
Se liberan de la memoria real: 11 a 119
La instrucción tomó 10.9s

F
Fin. Reporte de salida:
Turnaround de cada proceso:
	Proceso 3:	5s - 9.3s       	4.3s de turnaround
	Proceso 2:	2s - 139.8s     	137.8s de turnaround
	Proceso 5:	14.3s - 140.5s  	126.2s de turnaround
	Proceso 4:	9s - 140.9s     	131.9s de turnaround
	Proceso 6:	129.3s - 141.5s 	12.2s de turnaround
	Proceso 61:	139.6s - 142.1s 	2.5s de turnaround
	Proceso 109:	123.3s - 153s   	29.7s de turnaround
Turnaround promedio: 63.51428571428572 segundos
Swaps por proceso:
	Proceso 3:	0 swap-ins,	0 swap-outs
	Proceso 2:	0 swap-ins,	1 swap-outs
	Proceso 5:	0 swap-ins,	0 swap-outs
	Proceso 4:	1 swap-ins,	3 swap-outs
	Proceso 6:	0 swap-ins,	0 swap-outs
	Proceso 61:	0 swap-ins,	0 swap-outs
	Proceso 109:	0 swap-ins,	0 swap-outs
La instrucción tomó 0s

A 2 2 0
Obtener la dirección real correspondiente a la dirección virtual 2 del proceso 2
Error: No existe un proceso ejecutándose con el pid 2

P 2049 2049
Asignar 2049 bytes al proceso 2049
Error: El tamaño del proceso (2049 bytes) es mayor al de la memoria real (2048 bytes)

F
Fin. Reporte de salida:
Turnaround de cada proceso:
	Proceso 3:	5s - 9.3s       	4.3s de turnaround
	Proceso 2:	2s - 139.8s     	137.8s de turnaround
	Proceso 5:	14.3s - 140.5s  	126.2s de turnaround
	Proceso 4:	9s - 140.9s     	131.9s de turnaround
	Proceso 6:	129.3s - 141.5s 	12.2s de turnaround
	Proceso 61:	139.6s - 142.1s 	2.5s de turnaround
	Proceso 109:	123.3s - 153s   	29.7s de turnaround
Turnaround promedio: 63.51428571428572 segundos
Swaps por proceso:
	Proceso 3:	0 swap-ins,	0 swap-outs
	Proceso 2:	0 swap-ins,	1 swap-outs
	Proceso 5:	0 swap-ins,	0 swap-outs
	Proceso 4:	1 swap-ins,	3 swap-outs
	Proceso 6:	0 swap-ins,	0 swap-outs
	Proceso 61:	0 swap-ins,	0 swap-outs
	Proceso 109:	0 swap-ins,	0 swap-outs
La instrucción tomó 0s

E
La instrucción tomó 0s
```
