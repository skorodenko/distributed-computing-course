use lab_4::lineareq::gauss;
use ndarray::prelude::*;

fn main() {
    let m = array![
        [8., 7., 3., 18.],
        [-7., -4., -4., -11.],
        [-6., 5.0, -4.0, -15.0],
    ];

    println!("Input: {}", m);
    
    let m = gauss(m);
    println!("Result: {}", m);
}
