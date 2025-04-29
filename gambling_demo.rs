use std::fs::File; // File library
use std::io::{Write, BufWriter}; // Using Write and Buffered Writer from io library

static P: f64 = 0.49; // p value 
static INV_P: f64 = 0.51; // Inverse p value

// Function to premultiply a vector with a matrix, returns a 64bit float vector
fn vector_matrix_multiplication(vector: &Vec<f64>, matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let n = vector.len(); // Get vector dimension
    let m = matrix[0].len();  // Get matrix column dimension
    assert_eq!(n, matrix.len(), "Vector and matrix dimensions do not match"); // Check if the dimensions match

    let mut result = vec![0.0; m]; // Create the result vector

    
    for i in 0..m { 
        for j in 0..n {
            result[i] += vector[j] * matrix[j][i];  // "Dot product" to get the ith value of the vector
        }
    }
    result // We return the result
}

// Function to generate the markov chains of n states
fn generate_markov_chain(n: usize, markov_chain: &mut Vec<Vec<f64>>){
    // We place the p value in the next state, and 1-p in the previous state, working as a linked list
    // because we could only jump to the state i-1 and i+1 from the state i
    for i in 1..(n - 1) {
        markov_chain[i][i - 1] = INV_P;
        markov_chain[i][i + 1] = P;
    }

    // Absorbing states, placed at the corners of the matrix, ruin or maximum money respectively
    markov_chain[0][0] = 1.0;
    markov_chain[n - 1][n - 1] = 1.0;
}

// Function to write the result in the file
fn compute_and_write(n: usize, start: usize, filename: &str, markov_chain: &Vec<Vec<f64>>){
    let mut state_vector = vec![0.0; n];
    state_vector[start] = 1.0;

    let mut next_state = state_vector;
    // We iterate 1000 times to guarantee convergence
    for _ in 1..1001 {
        next_state = vector_matrix_multiplication(&next_state, &markov_chain);
    }

    // We try to open the file
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file); // Define a new Buffered Writer for the file
    write!(writer, "[").unwrap(); // Write "["
    for (i, value) in next_state.iter().enumerate() { // Iterate the state vector
        write!(writer, "{:.6}", value).unwrap(); // Write the probability of the ith state with 6 decimals
        if i < next_state.len()-1 { // If the value isn't the last one, we place a comma before the probability
            write!(writer, ", ").unwrap();
        }
    }
    writeln!(writer, "]").unwrap(); // Write "]"
}

fn main() {
    let n: usize = 1000; // Number of states

    let mut markov_chain = vec![vec![0.0; n]; n]; // Define the Markov Chain
    generate_markov_chain(n, &mut markov_chain); // Generate the Markov chain for our matrix

    // Write the results into the files (9, 99, 999 are the values because we have a 0-indexed markov chain)
    compute_and_write(n, 9, "steady_10.txt", &markov_chain);
    compute_and_write(n, 99, "steady_100.txt", &markov_chain);
    compute_and_write(n, 999, "steady_1000.txt", &markov_chain);
}
