use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

/*
Notes:

image are worked on in threads using par_iter but the 
greyscale conversion itself still occurs in sequential.

*/

fn main() {
    //set input and output folders
    let input_folder = r"C:\Users\paddy\Desktop\img_optimized\input";
    let output_folder = r"C:\Users\paddy\Desktop\img_optimized\output";

    //settings
    let num_runs = 2;
    let chunk_size = 100;

    //remove all contents of output folder
    delete_output_content(output_folder);

    //par seq
    delete_output_content(output_folder);
    // Measure the total time for multiple runs
    let total_start_time = Instant::now();

    for _ in 0..num_runs {
        let start_time = Instant::now();
        process_images_seq(input_folder, output_folder);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("fu1: {:?}", elapsed_time);
    }
    // Calculate average time
    let total_end_time = Instant::now();
    let total_elapsed_time = total_end_time - total_start_time;
    let average_time = total_elapsed_time / num_runs as u32;
    println!("Average time for {} runs: {:?}", num_runs, average_time);



    //par half
    delete_output_content(output_folder);
    // Measure the total time for multiple runs
    let total_start_time = Instant::now();

    for _ in 0..num_runs {
        let start_time = Instant::now();
        process_images_par_half(input_folder, output_folder);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("fu1: {:?}", elapsed_time);
    }
    // Calculate average time
    let total_end_time = Instant::now();
    let total_elapsed_time = total_end_time - total_start_time;
    let average_time = total_elapsed_time / num_runs as u32;
    println!("Average time for {} runs: {:?}", num_runs, average_time);



    //par full 1
    delete_output_content(output_folder);
    // Measure the total time for multiple runs
    let total_start_time = Instant::now();

    for _ in 0..num_runs {
        let start_time = Instant::now();
        process_images_par_full1(input_folder, output_folder);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("fu1: {:?}", elapsed_time);
    }
    // Calculate average time
    let total_end_time = Instant::now();
    let total_elapsed_time = total_end_time - total_start_time;
    let average_time = total_elapsed_time / num_runs as u32;
    println!("Average time for {} runs: {:?}", num_runs, average_time);
    




    //par full 2
    delete_output_content(output_folder);
    // Measure the total time for multiple runs
    let total_start_time = Instant::now();

    for _ in 0..num_runs {
        let start_time = Instant::now();
        process_images_par_full2(input_folder, output_folder, chunk_size);
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("fu2: {:?}", elapsed_time);
    }
    // Calculate average time
    let total_end_time = Instant::now();
    let total_elapsed_time = total_end_time - total_start_time;
    let average_time = total_elapsed_time / num_runs as u32;
    println!("Average time for {} runs: {:?}", num_runs, average_time);

}





fn process_images_seq(input_folder: &str, output_folder: &str) {
    //loop over all files in input folder
    for entry in fs::read_dir(input_folder).expect("Failed to read input folder") {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            //check if file is img, need to change later
            if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".png") {
                //load
                let img = image::open(&file_path).expect("Failed to open image");

                //convert
                let gray_img = convert_to_grayscale(&img);

                //output path
                let output_path = format!("{}/{}_output.jpg", output_folder, &file_name[..4]);

                //save and export
                gray_img.save(output_path).expect("Failed to save image");
            }
        }
    }
}

fn process_images_par_half(input_folder: &str, output_folder: &str) {
    //read dir entries and represent in vector
    let entries: Vec<_> = fs::read_dir(input_folder)
        .expect("Failed to read input folder")
        .filter_map(|entry| entry.ok())
        .collect();

    //par_iter to iterate over images in parallel
    entries
        .par_iter()
        .for_each(|entry| {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            //check if file is img, need to change later
            if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".png") {
                // Load the image
                let img = image::open(&file_path).expect("Failed to open image");

                //convert
                let gray_img = convert_to_grayscale(&img);

                //output path
                let output_path = format!("{}/{}_output.jpg", output_folder, &file_name[..4]);

                //save and export
                gray_img.save(output_path).expect("Failed to save image");
            }
        });
}

fn process_images_par_full1(input_folder: &str, output_folder: &str) {
    //images to vector
    let entries: Vec<_> = fs::read_dir(input_folder)
        .expect("Failed to read input folder")
        .filter_map(|entry| entry.ok())
        .collect();

    //par_iter_mut to parallelize the iteration over mutable references
    entries
        //into_par_iter allow ownership transfer
        .into_par_iter()  
        .for_each(|entry| {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            //check format
            if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".png") {
                // Load the image
                let mut img = image::open(&file_path).expect("Failed to open image");

                //convert img to grey using multiple threads
                convert_to_grayscale_par(&mut img);

                //output path
                let output_path = format!("{}/{}_output.jpg", output_folder, &file_name[..4]);

                //save and export
                img.save(output_path).expect("Failed to save image");
            }
        });
}

fn process_images_par_full2(input_folder: &str, output_folder: &str, chunk_size: usize) {
    //images to vector
    let entries: Vec<_> = fs::read_dir(input_folder)
        .expect("Failed to read input folder")
        .filter_map(|entry| entry.ok())
        .collect();

    //par_iter_mut to parallelize the iteration over mutable references
    entries
        //into_par_iter allow ownership transfer
        .into_par_iter()  
        .for_each(|entry| {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            //check format
            if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".png") {
                // Load the image
                let mut img = image::open(&file_path).expect("Failed to open image");

                //convert img to grey using multiple threads
                convert_to_grayscale_par_chunks(&mut img, chunk_size);

                //output path
                let output_path = format!("{}/{}_output.jpg", output_folder, &file_name[..4]);

                //save and export
                img.save(output_path).expect("Failed to save image");
            }
        });
}

fn convert_to_grayscale(input_img: &DynamicImage) -> RgbaImage {
    let (width, height) = input_img.dimensions();
    let mut gray_img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            //get pixel color
            let pixel = input_img.get_pixel(x, y);

            //calc greyscale val
            let gray_value = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;

            //set new pixel in output img
            gray_img.put_pixel(x, y, Rgba([gray_value, gray_value, gray_value, pixel[3]]));
        }
    }

    gray_img
}

fn convert_to_grayscale_par(input_img: &mut image::DynamicImage) {
    let (width, height) = input_img.dimensions();
    let mut gray_img = image::RgbaImage::new(width, height);

    //par_chunks_mut to parallelize the iteration over mutable chunks
    gray_img
        .par_chunks_mut(4) //DO NOT CHANGE
        .enumerate()
        .for_each(|(idx, pixel)| {
            // Calculate the pixel position
            let x = (idx % width as usize) as u32;
            let y = (idx / width as usize) as u32;

            //get pixel col
            let input_pixel = input_img.get_pixel(x, y);

            //calc grey scal val
            let gray_value = (
                0.299 * input_pixel[0] as f32 +
                0.587 * input_pixel[1] as f32 +
                0.114 * input_pixel[2] as f32
            ) as u8;

            //set new pixel in output img
            pixel.copy_from_slice(&[gray_value, gray_value, gray_value, input_pixel[3]]);
        });

    *input_img = image::DynamicImage::ImageRgba8(gray_img);
}

fn convert_to_grayscale_par_chunks(input_img: &mut DynamicImage, chunk_size: usize) {
    let (width, height) = input_img.dimensions();
    let mut gray_img = RgbaImage::new(width, height);

    //chunksize
    //let chunk_size = 100;

    //par chunks
    gray_img
        .par_chunks_mut(4 * chunk_size)
        .enumerate()
        .for_each(|(chunk_idx, chunk)| {
            //calc starting pixel
            let start_pixel_idx = chunk_idx * chunk_size;

            //iterate over pixels within the chunk
            for pixel_offset in 0..chunk_size {
                let pixel_idx = start_pixel_idx + pixel_offset;

                //check if pixel index is within bounds
                if pixel_idx < width as usize * height as usize {
                    //pixel pos
                    let x = (pixel_idx % width as usize) as u32;
                    let y = (pixel_idx / width as usize) as u32;

                    //get pix col
                    let input_pixel = input_img.get_pixel(x, y);

                    //calc grey val
                    let gray_value = (
                        0.299 * input_pixel[0] as f32 +
                        0.587 * input_pixel[1] as f32 +
                        0.114 * input_pixel[2] as f32
                    ) as u8;

                    //set new pixel
                    let chunk_pixel_offset = pixel_offset * 4;
                    chunk[chunk_pixel_offset..chunk_pixel_offset + 4]
                        .copy_from_slice(&[gray_value, gray_value, gray_value, input_pixel[3]]);
                }
            }
        });

    //update output image
    *input_img = DynamicImage::ImageRgba8(gray_img);
}

fn delete_output_content(output_folder: &str) {
    fs::read_dir(output_folder)
    .expect("Failed to read folder")
    .flat_map(|entry| entry)
    .for_each(|entry| {
        let _ = fs::remove_file(entry.path());
    });
}
