use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use nalgebra::{DMatrix};


pub fn matmul_paralel(m1: &DMatrix<f64>, m2: &DMatrix<f64>) -> DMatrix<f64> {
    let m1shape = m1.shape();
    let m2shape = m2.shape();
    let nrows = m1shape.0;
    let ncols = m2shape.1;

    let iter: Vec<f64> = (0..m2shape.1).into_par_iter().flat_map(move |rj| {
        (0..m1shape.0).into_par_iter().map(move |li| {
            (0..m2shape.0)
                .zip(0..m1shape.1)
                .map(move |(ri, lj)| {
                    m1.index((li, lj)) * m2.index((ri, rj))
                })
                .sum()
        })
    })
    .collect();

    DMatrix::<f64>::from_iterator(nrows, ncols, iter)
}


pub fn matmul(m1: &DMatrix<f64>, m2: &DMatrix<f64>) -> DMatrix<f64> {
    let m1shape = m1.shape();
    let m2shape = m2.shape();
    let nrows = m1shape.0;
    let ncols = m2shape.1;

    let iter: Vec<f64> = (0..m2shape.1).flat_map(move |rj| {
        (0..m1shape.0).map(move |li| {
            (0..m2shape.0)
                .zip(0..m1shape.1)
                .map(move |(ri, lj)| {
                    m1.index((li, lj)) * m2.index((ri, rj))
                })
                .sum()
        })
    })
    .collect();
    
    DMatrix::<f64>::from_iterator(nrows, ncols, iter)
}


pub fn matelsum_paralel(m: &DMatrix<f64>) -> f64 {
    let a = Arc::new(Mutex::new(0.0));
    let shape = m.shape();

    let _a = a.clone();
    (0..shape.0).into_par_iter().map(move |i| {
        let _a = _a.clone();
        (0..shape.1).into_par_iter().map(move |j| {
            let mut sum = _a.lock().unwrap();
            *sum += m.index((i,j)); 
        })
    })
    .collect();
    
    let res = *a.lock().unwrap();
    res
}


pub fn matelsum(m: &DMatrix<f64>) -> f64 {
    let mut a = 0.0;
    let shape = m.shape();

    let _ = (0..shape.0).into_iter().map(move |i| {
        (0..shape.1).into_iter().map(move |j| {
            a += m.index((i,j)); 
        })
    });
    
    a
}