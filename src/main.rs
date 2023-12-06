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

    //remove all contents of output folder
    fs::read_dir(output_folder)
        .expect("Failed to read folder")
        .flat_map(|entry| entry)
        .for_each(|entry| {
            let _ = fs::remove_file(entry.path());
        });

    let start_time = Instant::now();
    process_images_seq(input_folder, output_folder);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("seq: {:?}", elapsed_time);

    let start_time = Instant::now();
    process_images_par(input_folder, output_folder);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("par: {:?}", elapsed_time);
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

fn process_images_par(input_folder: &str, output_folder: &str) {
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
