use std::fs::File;
use std::io::{Write, BufWriter};

static P: f64 = 0.49;
static INV_P: f64 = 0.51;

fn vector_matrix_multiplication(vector: &Vec<f64>, matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let n = vector.len();
    let m = matrix[0].len();
    assert_eq!(n, matrix.len(), "Vector and matrix dimensions do not match");

    let mut result = vec![0.0; m];
    for i in 0..m {
        for j in 0..n {
            result[i] += vector[j] * matrix[j][i];
        }
    }
    result
}

fn generate_markov_chain(n: usize, markov_chain: &mut Vec<Vec<f64>>){
    for i in 1..(n - 1) {
        markov_chain[i][i - 1] = INV_P;
        markov_chain[i][i + 1] = P;
    }

    markov_chain[0][0] = 1.0;
    markov_chain[n - 1][n - 1] = 1.0;
}

fn compute_and_write(n: usize, start: usize, filename: &str, markov_chain: &Vec<Vec<f64>>){
    let mut state_vector = vec![0.0; n];
    state_vector[start] = 1.0;

    let mut next_state = state_vector;
    for _ in 1..1001 {
        next_state = vector_matrix_multiplication(&next_state, &markov_chain);
        //let sum: f64 = next_state.iter().sum();
        /*
        if sum != 0.0 {
            for x in next_state.iter_mut() {
                *x /= sum;
            }
        }
        */
    }

    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    write!(writer, "[").unwrap();
    for (i, value) in next_state.iter().enumerate() {
        write!(writer, "{:.6}", value).unwrap();
        if i < next_state.len()-1 {
            write!(writer, ", ").unwrap();
        }
    }
    writeln!(writer, "]").unwrap();
}

fn main() {
    let n: usize = 1000;

    let mut markov_chain = vec![vec![0.0; n]; n];
    generate_markov_chain(n, &mut markov_chain);

    compute_and_write(n, 9, "steady_10.txt", &markov_chain);
    compute_and_write(n, 99, "steady_100.txt", &markov_chain);
    compute_and_write(n, 999, "steady_1000.txt", &markov_chain);
}
