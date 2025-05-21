// Mathematically exact (theoretically), but floating-point rounding causes loss of precision for large n.
pub fn fib(n: i32) -> i32 {
    let sqrt5 = 5_f64.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    ((phi.powi(n) / sqrt5).round()) as i32
}
