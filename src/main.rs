use std::error::Error;
use std::env;
use std::process;

use ppm_writer_rs::PPMWriter;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify a path to the output file!");
        process::exit(1);
    }

    let ppm_file = String::from(&args[1]);
    //let ppm_file = String::from("/Users/abtiwary/temp/from_ppm_writer.ppm");

    let width: u16 = 200;
    let height: u16 = 100;
    
    // generate some image data - in this case a 200x100 gradient
    let mut gradient_image_data: Vec<u8> = vec![0; 3 * width as usize * height as usize];
    let mut gradient_image_data_idx = 0;
    for y in (0..height).rev() {
        for x in 0..width {
            let r: f32 = f32::from(x) / f32::from(width);
            let g: f32 = f32::from(y) / f32::from(height);
            let b: f32 = 0.2;
            let ir: u8 = (255.99 * r) as u8;
            let ig: u8 = (255.99 * g) as u8;
            let ib: u8 = (255.99 * b) as u8;

            gradient_image_data[gradient_image_data_idx] = ir;
            gradient_image_data[gradient_image_data_idx+1] = ig;
            gradient_image_data[gradient_image_data_idx+2] = ib;
            gradient_image_data_idx += 3;
        }
    }

    let mut ppm_writer = PPMWriter::new(
        &ppm_file, 
        200, 
        100, 
        255);

    ppm_writer.set_image_data(&gradient_image_data).unwrap();
    ppm_writer.write_output_file();

    Ok(())
}