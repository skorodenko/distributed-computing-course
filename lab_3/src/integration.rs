use std::sync::{Arc};
use std::marker::{Sync, Send};
use rayon::prelude::*;


pub fn integral_reduction(f: &Arc<impl Fn(f64) -> f64 + Sync + Send>, a: f64, b: f64, steps: i32) -> f64 {
    let dx = (b - a) / steps as f64;

    (0..steps).into_par_iter().map(move |i| {
        let i = i as f64;
        let x = a + i * dx;

        let function = f(x);
        function * dx
    }).reduce(|| 0.0, |a,b| a + b)
}


#[cfg(test)]
mod tests {
    use super::integral_reduction;
    use std::sync::{Arc};
    use std::f64::{consts::PI, INFINITY};
    
    #[test]
    fn test_example_sample() {
        println!("Task 1");
        let f = |x: f64| 1.0 / (f64::sin(2.0*x)).powi(2);
        let a = 0.0;
        let b = PI/2.0;
        let steps = 64;
        
        let af = Arc::new(f);

        let res = integral_reduction(&af, a, b, steps);
        assert_eq!(res, INFINITY)
    }
}