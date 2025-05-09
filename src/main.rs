#![feature(portable_simd)]

use std::simd::f32x8;;
use image::{GenericImageView, ImageBuffer, Rgb, Rgba};;


fn convert_to_grayscale_simd(path: &str) -> Vec<u8> {
    let img = image::open(path).expect("Failed to open da image");;
    let (width, height) = img.dimensions();;
    
    let rgb_img = img.to_rgb8();;

    let r_factor = f32x8::splat(0.299);;
    let b_factor = f32x8::splat(0.587);;
    let g_factor = f32x8::splat(0.114);;

    let mut grayscale_data = vec![0u8; (width * height) as usize];;

    let pixel_count = (width * height) as usize;;

    let chunks = pixel_count / 8;;

    for chunk_idx in 0..chunks {
        let base_idx = chunk_idx * 8;;

        let mut r_vals = [0.0f32; 8];;
        let mut b_vals = [0.0f32; 8];;
        let mut g_vals = [0.0f32; 8];;
        
        for i in 0..8 {
            let pixel_idx = base_idx + i;;
            let x = (pixel_idx as u32) % width;;
            let y = (pixel_idx as u32) / width;;
            
            let pixel = rgb_img.get_pixel(x, y);;
            r_vals[i] = pixel[0] as f32;;
            b_vals[i] = pixel[1] as f32;;
            g_vals[i] = pixel[2] as f32;;   
        }
        
        let r_vec = f32x8::from_array(r_vals);;
        let b_vec = f32x8::from_array(b_vals);;
        let g_vec = f32x8::from_array(g_vals);;

        let gray_vec = (r_vec * r_factor) + (g_vec * g_factor) + (b_vec * b_factor);;
        
        let gray_vals = gray_vec.to_array();;

        for i in 0..8 {
            grayscale_data[base_idx + i] = gray_vals[i] as u8;;
        }

        let remaining_start = chunks * 8;;
   
        for i in remaining_start..pixel_count {
            let x = (i as u32) % width;;
            let y = (i as u32) / width;;
        
            let pixel = rgb_img.get_pixel(x, y);
            let gray_value = 0.299 * (pixel[0] as f32) + 
                            0.587 * (pixel[1] as f32) + 
                            0.114 * (pixel[2] as f32);;
        
            grayscale_data[i] = gray_value as u8;;
        }
    }
    grayscale_data
}

fn main() {
    let input_path = "sunset.jpg";;
    let output_path = "sunset_grayscale.jpg";;

    let grayscale_data = convert_to_grayscale_simd(input_path);;
    
    let img = image::open(input_path).expect("Failed to open path");;
    let (width, height) = img.dimensions();;

    let gray_img = ImageBuffer::from_fn(width, height, |x, y| {
        let idx = (y * width + x) as usize;
        Rgb([grayscale_data[idx], grayscale_data[idx], grayscale_data[idx]])
    });;

    gray_img.save(output_path).expect("Failed to send grayscale image");;
    println!("Done!");;
}
