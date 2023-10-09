use std::{sync::Arc, f64::consts::PI};
use lab_3::integration::integral_reduction;


fn task_1() {
    println!("Task 1");
    let f = |x: f64| 1.0 / (f64::sin(2.0*x)).powi(2);
    let a = 0.0;
    let b = PI/2.0;
    let steps = 64;
    
    let af = Arc::new(f);

    let res = integral_reduction(&af, a, b, steps);
    println!("Result: {}", res);
}


fn main() {
    task_1();
}
