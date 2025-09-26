use std::time::Instant;
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() {
    let matrix: [[i32;9];9] = [
        [8,0,0,0,0,0,0,0,0],
        [0,0,3,6,0,0,0,0,0],
        [0,7,0,0,9,0,2,0,0],
        [0,0,5,0,0,7,0,0,0],
        [0,0,0,0,4,5,7,0,0],
        [0,0,0,1,0,0,0,3,0],
        [0,0,1,0,0,0,0,6,8],
        [0,0,8,5,0,0,0,1,0],
        [0,9,0,0,0,0,4,0,0]
    ];

    //Modificar para distintos valores de k
    let num_threads: usize = num_cpus::get();//num_cpus::get();
    println!("Resutados para {} hilos: ", num_threads);

    // divide las columas para cada hilo
    let column_assignments = assign_rows(matrix.len(), num_threads);

    let start_time = Instant::now();

    let found = Arc::new(AtomicBool::new(false));

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(num_threads);

    for columns in column_assignments {

        let matrix_copy = matrix;
        let found_clone = Arc::clone(&found);
        let start_clone = start_time.clone();

        let handle = thread::spawn(move || {
            for column in columns {
                solve_sdk(matrix_copy, column, &found_clone, start_clone);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn solve_sdk (mut matrix: [[i32;9];9], col: usize, found: &Arc<AtomicBool>, start_time: Instant) {

    if  matrix[0][col] == 0 {
        for i in 1..10{

            if found.load(Ordering::Relaxed) {
                return;
            }

            //valida si se puede posicionar el número
            if validate_vertical(matrix, col, i) && validate_horizontal(matrix, 0, i) && validate_box(matrix, 0, col,  i) {

                //se inserta y se resuelve con recursión
                matrix[0][col] = i;
                recursive_sdk(matrix, 0, 0, found, start_time);
            }
        }
    }
}

fn recursive_sdk(mut matrix: [[i32;9];9], row: usize, col: usize, found: &Arc<AtomicBool>, start_time: Instant) -> bool {

    if found.load(Ordering::Relaxed) {
        return false;
    }

    // sudoku completado
    if row == 9 {
        // solo el primero imprime
        if !found.swap(true, Ordering::Relaxed) {
            for i in 0..9 {
                for j in 0..9 {
                    print!("{} ", matrix[i][j]);
                }
                println!();
            }
            let duration = start_time.elapsed();
            println!("Sudoku resuelto en {:?}", duration);
        }
        return true;
    }

    // si termina la fila, paso a la siguiente
    if col == 9 {
        return recursive_sdk(matrix, row + 1, 0, found, start_time);
    }

    //si la casilla está ocupada, avanza
    if matrix[row][col] != 0 {
        return recursive_sdk(matrix, row, col +1, found, start_time);
    }

    //validar si es posible ubicarlo
    for i in 1..10{
        if validate_vertical(matrix, col, i) && validate_horizontal(matrix, row, i) && validate_box(matrix, row, col, i){
            //se inserta y se resuelve con recursión
            matrix[row][col] = i;
            if recursive_sdk(matrix, row, col + 1, found, start_time) {
                return true;
            }
            matrix[row][col] = 0;
        }
    }
    false
}

//Verifica que un número no se repita en su fila
fn validate_horizontal(matrix: [[i32;9];9], row: usize, num:i32) -> bool {

    for i in 0..9{
        if matrix[row][i] == num {
            return false;
        }
    }
    true
}

//Verifica que un número no se repita en su columna
fn validate_vertical(matrix: [[i32;9];9], col: usize, num:i32) -> bool {
    for i in 0..9{
        if matrix[i][col] == num {
            return false;
        }
    }
    true
}

//Verifica que un número no se repita dentro de su cuadro 3x3
fn validate_box(matrix: [[i32;9];9], row: usize, col: usize, num: i32) -> bool {
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;

    for r in start_row..start_row + 3 {
        for c in start_col..start_col + 3 {
            if matrix[r][c] == num {
                return false;
            }
        }
    }
    true
}

//Divide el trabajo en la cantidad de hilos disponibles
fn assign_rows(n: usize, t: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let base = n / t;
    let extra = n % t;
    let mut start = 0;

    for i in 0..t {
        let mut count = base;
        if i < extra {
            count += 1;
        }
        let columns: Vec<usize> = (start..start+count).collect();
        result.push(columns);
        start += count;
    }

    result
}
