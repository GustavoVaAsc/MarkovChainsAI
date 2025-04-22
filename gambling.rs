use std::io;

static P: f64 = 0.49;
static INV_P: f64 = 0.51;

fn matrix_vector_multiplication(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>) -> Vec<f64> {
    let n = matrix.len();
    let m = matrix[0].len();
    assert_eq!(m, vector.len(), "Matrix and vector dimensions do not match");

    let mut result = vec![0.0; n];
    for i in 0..n {
        for j in 0..m {
            result[i] += matrix[i][j] * vector[j];
        }
    }
    result
}

fn generate_markov_chain(n: usize, markov_chain: &mut Vec<Vec<f64>>){
    for i in 1..n {
        markov_chain[i][i-1] = P;
        markov_chain[i-1][i] = INV_P;
    }
    markov_chain[0][0] = 1.0;
    markov_chain[1][0] = 0.0;
    markov_chain[n-1][n-1] = 1.0;
    markov_chain[n-2][n-1] = 0.0;
}

fn main(){
    let mut input = String::new();
    println!("Write the number of states");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Invalid integer input");

    let mut markov_chain = vec![vec![0.0; n]; n];
    let mut state_vector = vec![0.0; n];

    generate_markov_chain(n, &mut markov_chain);
    input.clear();
    println!("Write the initial state");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let state:usize = input.trim().parse().expect("Invalid integer input");

    state_vector[state] = 1.0;

    let result_vector = matrix_vector_multiplication(&markov_chain, &state_vector);

    println!("{:?}", result_vector);
}
