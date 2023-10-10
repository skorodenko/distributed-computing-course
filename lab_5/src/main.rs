use lab_5::constcalc::{picalc, mc_picalc};


fn task_1() {
    println!("Task 1");
    let pi = picalc(1e+7 as i32);
    println!("Picalc: {}", pi);
}


fn task_2() {
    println!("Task 2");
    let pi = mc_picalc(1e+7 as i32);
    println!("Monte-Carlo Picalc: {}", pi);
}


fn main() {
    task_1();
    task_2();
}
