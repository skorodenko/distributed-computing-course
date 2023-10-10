use nalgebra::{DMatrix};
use lab_4::lineareq::gauss;

fn main() {
    let m = DMatrix::from_row_slice(3, 4, &[
        2.0, -1.0, 0.0, 0.0,
        -1.0, 1.0, 4.0, 13.0,
        1.0, 2.0, 3.0, 14.0
    ]);

    println!("Input: {}", m);
    
    let m = gauss(m);
    println!("Result: {}", m);
}
