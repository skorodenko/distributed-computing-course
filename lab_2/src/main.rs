use rayon::{current_num_threads};


fn task_1() {
    println!("Task 1:");
    let num: usize = current_num_threads();
    println!("CPU cores number: {}", num);
}


fn main() {
    task_1();
}
