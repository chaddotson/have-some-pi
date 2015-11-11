// compile with: rustc rust-bbp-formula.rs

// https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula

fn main() {
    let num_steps = 11i32;
    let mut pi_summation = 0.0f64;
    for i in 0..num_steps {
        pi_summation += (1.0f64 / 16.0f64.powf(i as f64)) * ((4.0f64/(8.0f64 * (i as f64)+1.0f64)) - (2.0f64/(8.0f64 * (i as f64)+4.0f64)) - (1.0f64/(8.0f64 * (i as f64)+5.0f64)) - (1.0f64/(8.0f64 * (i as f64)+6.0f64))   );
    }

    println!("Pi = {:.*}", 20, pi_summation);
}
