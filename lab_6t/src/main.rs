extern crate nalgebra as na;
extern crate rayon;

use std::sync::{Mutex};

use na::DMatrix;
use rayon::prelude::*;

fn label_4_neighbors(image: &DMatrix<u32>) -> DMatrix<u32> {
    let (rows, cols) = image.shape();
    let mut labels = DMatrix::from_element(rows, cols, 0u32);
    let _labels = Mutex::new(&mut labels);
    let next_label = Mutex::new(1u32);

    (0..rows).into_par_iter().for_each(|i| {
        (0..cols).into_par_iter().for_each(|j| {
            if image[(i, j)] != 0 {
                let mut neighbor_labels = Vec::new();

                let mut _labels = _labels.lock().unwrap();

                if i > 0 && _labels[(i - 1, j)] != 0 {
                    neighbor_labels.push(_labels[(i - 1, j)]);
                }
                if i < rows - 1 && _labels[(i + 1, j)] != 0 {
                    neighbor_labels.push(_labels[(i + 1, j)]);
                }
                if j > 0 && _labels[(i, j - 1)] != 0 {
                    neighbor_labels.push(_labels[(i, j - 1)]);
                }
                if j < cols - 1 && _labels[(i, j + 1)] != 0 {
                    neighbor_labels.push(_labels[(i, j + 1)]);
                }

                match neighbor_labels.iter().cloned().min() {
                    Some(min_label) => {
                        _labels[(i, j)] = min_label;
                        for &label in &neighbor_labels {
                            if label != min_label {
                                relabel_regions(&mut _labels, label, min_label);
                            }
                        }
                    }
                    None => {
                        let mut _next_label = next_label.lock().unwrap();
                        _labels[(i, j)] = *_next_label;
                        *_next_label += 1;
                    }
                }
            }
        });
    });

    labels
}

fn relabel_regions(labels: &mut DMatrix<u32>, old_label: u32, new_label: u32) {
    labels
        .iter_mut()
        .for_each(|label| {
            if old_label == *label {
                *label = new_label
            };
        });
}

fn main() {
    let data = DMatrix::from_row_slice(
        6,
        6,
        &[
            1, 1, 0, 0, 0, 0,
            1, 1, 0, 1, 0, 0,
            1, 0, 0, 0, 0, 0,
            0, 0, 1, 1, 1, 1,
            1, 0, 0, 1, 0, 0,
            1, 0, 0, 1, 0, 1
        ],
    );

    let labeled_image = label_4_neighbors(&data);

    println!("Original Image:");
    println!("{}", data);
    println!("Labeled Image:");
    println!("{}", labeled_image);
}