use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use nalgebra::{DMatrix, DVector};


pub fn gauss (mut m: DMatrix<f64>) -> DVector<f64> {
    let nrows = m.shape().0;

    // Прямий хід
    let _m = Arc::new(Mutex::new(&mut m));
    (0..nrows).into_iter().for_each(|i| {
        let tmp = *_m.lock().unwrap().index((i,i));
        
        (0..nrows).rev().into_iter().for_each(|j| {
            _m.lock().unwrap()[(i,j)] /= tmp;            
        });
        
        (i+1..nrows).into_par_iter().for_each(|j| {
            let tmp = *_m.lock().unwrap().index((j,i));

            let _m = _m.clone();
            (i..nrows).into_iter().for_each(|k| {
                let mut mm = _m.lock().unwrap();
                mm[(j,k)] -= tmp * (*mm.index((i,k)));
            });
        });
    });
    
    
    // Зворотній хід
    let mut xx = DVector::<f64>::zeros(nrows);
    xx[nrows - 1] = *m.index((nrows - 1, nrows));
    
    (0..nrows-1).rev().into_iter().for_each(|i| {
        xx[i] = *m.index((i, nrows));

        (i+1..nrows).into_iter().for_each(|j| {
            xx[i] -= m.index((i,j)) * xx[j];
        });
    });
    
    xx
}
