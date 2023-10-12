extern crate image;
extern crate rayon;
extern crate nalgebra as na;

use image::{GrayImage, Luma};
use std::collections::VecDeque;
//use rayon::prelude::*;
use na::DMatrix;

fn main() {
    let image_path = "/home/rinkuro/SecondDesktop/Study/DistributedComputing/distributed-computing-course/lab_6/data/test.png";
    let img = image::open(image_path).expect("Failed to open image").to_luma8();
    let (width, height) = img.dimensions();
    let mut labeled_image = GrayImage::new(width, height);
    let mut label = 1;

    let mut visited = DMatrix::from_element(width as usize, height as usize, false);

    for y in 0..height {
        for x in 0..width {
            if !visited[(x as usize, y as usize)] && img.get_pixel(x, y)[0] == 255 {
                let connected_pixels = find_connected_component(&img, (x, y), &mut visited);
                visited = label_connected_component(&connected_pixels, &mut labeled_image, label, visited);
                label += 1;
            }
        }
    }

    let ascii_matrix = na::DMatrix::<u8>::from_vec(32, 32, labeled_image.to_ascii_lowercase());
    println!("{}", ascii_matrix);
}

fn find_connected_component(img: &GrayImage, start_pixel: (u32, u32), visited: &mut DMatrix<bool>) -> Vec<(u32, u32)> {
    let mut component = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_pixel);

    let (width, height) = img.dimensions();

    while let Some((cx, cy)) = queue.pop_front() {
        if cx < width && cy < height && !visited[(cx as usize, cy as usize)] {
            visited[(cx as usize, cy as usize)] = true;
            component.push((cx, cy));

            if img.get_pixel(cx, cy)[0] == 255 {
                if cx + 1 < width {
                    queue.push_back((cx + 1, cy));
                }
                if cx > 0 {
                    queue.push_back((cx - 1, cy));
                }
                if cy + 1 < height {
                    queue.push_back((cx, cy + 1));
                }
                if cy > 0 {
                    queue.push_back((cx, cy - 1));
                }
            }
        }
    }

    component
}

fn label_connected_component(
    connected_pixels: &Vec<(u32, u32)>,
    labeled_image: &mut GrayImage,
    label: u8,
    visited: DMatrix<bool>,
) -> DMatrix<bool> {
    let mut new_visited = visited.clone();
    for (x, y) in connected_pixels {
        labeled_image.put_pixel(*x, *y, Luma([label]));
        new_visited[(*x as usize, *y as usize)] = true;
    }
    new_visited
}