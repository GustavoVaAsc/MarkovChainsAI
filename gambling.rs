use std::io;

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

fn generate_markov_chain(n: usize, markov_chain: &mut Vec<Vec<f64>>) {
    for i in 1..(n - 1) {
        markov_chain[i][i - 1] = INV_P;
        markov_chain[i][i + 1] = P;
    }

    // absorbing states
    markov_chain[0][0] = 1.0;
    markov_chain[n - 1][n - 1] = 1.0;
}


fn main(){
    let mut input = String::new();
    println!("Write the number of states");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Invalid integer input");

    let mut markov_chain = vec![vec![0.0; n]; n];
    let mut state_vector = vec![0.0; n];

    generate_markov_chain(n, &mut markov_chain);

    for row in markov_chain.iter(){
        println!("{:?}",row);
    }

    input.clear();

    println!("Write the initial state");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let state:usize = input.trim().parse().expect("Invalid integer input");

    state_vector[state] = 1.0;

    let mut next_state = state_vector;
    for _i in 1..1001 {
        next_state = vector_matrix_multiplication(&next_state,&markov_chain);

        let sum: f64 = next_state.iter().sum();
        if sum != 0.0 {
            for x in next_state.iter_mut() {
                *x /= sum;
            }
        }
    }
    println!("Steady state");
    print!("[ ");
    for value in next_state.iter(){
        print!("{:.2} ",value);
    }
    println!("]");
}
