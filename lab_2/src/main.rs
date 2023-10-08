use rayon::{current_num_threads};
use nalgebra::{DMatrix};
use lab_2::matop::{matelsum, matelsum_paralel};


fn task_1() {
    let num: usize = current_num_threads();
    println!("CPU cores number: {}", num);
}


fn task_3() {
    let nrows = 2;
    let ncols = 3;
    let m = DMatrix::<f64>::new_random(nrows, ncols);
    println!("{:?}", m);

    let r1 = matelsum(&m);
    let r2 = matelsum_paralel(&m);
    println!("Single: {}, Paralel: {}", r1, r2);
}


fn main() {
    task_1();
    task_3();
}
