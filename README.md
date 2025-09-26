# Taller 1 HPC - Sudoku solver

## Descripción
Programa que completa y mide el tiempo de resolución de un sudoku mediante paralelización

## Instrucciones
Compilar en modo release para mayor rendimiento

Por defecto, el número de hilos está fijado en el código (num_threads en main2.rs).
Para calcular speedup y eficiencia, modifique este valor y ejecute varias veces:

k = 1..N (donde N = núm. de núcleos + 1 en el procesador).

Anote los tiempos de ejecución impresos por el programa.

Calcule:

* Speedup: S(k) = T(1) / T(k)

* Eficiencia: E(k) = S(k) / k
