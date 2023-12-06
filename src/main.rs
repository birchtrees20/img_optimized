use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use std::fs;
use std::time::Instant;

fn main() {
    //set input and output folders
    let input_folder = r"C:\Users\paddy\Desktop\MUIC\Rust\final project\image1\input";
    let output_folder = r"C:\Users\paddy\Desktop\MUIC\Rust\final project\image1\output";

    //remove all contents of output folder
    fs::read_dir(output_folder)
        .expect("Failed to read folder")
        .flat_map(|entry| entry)
        .for_each(|entry| {
            let _ = fs::remove_file(entry.path());
        });

    let start_time = Instant::now();

    // Iterate over all files in the input folder
    for entry in fs::read_dir(&input_folder).expect("Failed to read input folder") {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            // Check if the file is an image (you might want to improve this check)
            if file_name.ends_with(".jpg") || file_name.ends_with(".jpeg") || file_name.ends_with(".png") {
                // Load the image
                let img = image::open(&file_path).expect("Failed to open image");

                // Convert the image to grayscale
                let gray_img = convert_to_grayscale(&img);

                // Specify the output image path
                let output_path = format!("{}/{}_output.jpg", output_folder, &file_name[..4]);

                // Save the grayscale image
                gray_img.save(output_path).expect("Failed to save image");
            }
        }
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}

fn convert_to_grayscale(input_img: &DynamicImage) -> RgbaImage {
    let (width, height) = input_img.dimensions();
    let mut gray_img = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Get the pixel color
            let pixel = input_img.get_pixel(x, y);

            // Calculate grayscale value
            let gray_value = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;

            // Set the grayscale pixel in the output image
            gray_img.put_pixel(x, y, Rgba([gray_value, gray_value, gray_value, pixel[3]]));
        }
    }

    gray_img
}
