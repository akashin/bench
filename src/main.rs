use std::time::Instant;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn matrix_multiply(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut result = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn monte_carlo_pi(iterations: usize) -> f64 {
    let seed = [0; 32]; // This ensures the RNG is deterministic
    let mut rng = StdRng::from_seed(seed);
    let mut inside_circle = 0;

    for _ in 0..iterations {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x*x + y*y <= 1.0 {
            inside_circle += 1;
        }
    }

    4.0 * (inside_circle as f64) / (iterations as f64)
}

fn main() {
    // matrix multiplication
    let n = 1100;
    let a = vec![vec![1.0; n]; n];
    let b = vec![vec![1.0; n]; n];
    let start_time = Instant::now();
    let _ = matrix_multiply(&a, &b);
    let elapsed = start_time.elapsed();
    println!("Elapsed: {:?}", elapsed);

    // pi monte-carlo
    let iterations = 500_000_000;
    let start_time_pi = Instant::now();
    let pi_estimate = monte_carlo_pi(iterations);
    let elapsed_pi = start_time_pi.elapsed();
    println!("Estimated Pi: {}", pi_estimate);
    println!("Elapsed: {:?}", elapsed_pi);
}
