use std::borrow::BorrowMut;
use std::{fs::File, io::Write};
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;

pub struct PPMWriter {
    output_file: String,
    magic: String,
    width: usize,
    height: usize,
    bit_depth: usize,
    image_data: Option<Rc<RefCell<Vec<u8>>>>,
}

impl PPMWriter {
    pub fn new(
        output_file: &str, 
        width: usize, 
        height: usize, 
        bit_depth: usize) -> Self {
           
        PPMWriter { 
            output_file: String::from(output_file),
            magic: String::from("P6\n"), 
            width: width, 
            height: height, 
            bit_depth: bit_depth, 
            image_data: Some(Rc::new(RefCell::new(vec![0; 3 * width * height]))), 
        }
    }
}

impl PPMWriter {
    pub fn set_image_data(&mut self, data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let mut image_data = self.image_data.as_ref().unwrap();
        let mut image_data = image_data.try_borrow_mut()?;
        let mut image_data = image_data.borrow_mut();

        image_data.copy_from_slice(&data[..]);
        println!("{:?}", &image_data[0..32]);
        println!("{:?}", &image_data.len());

        Ok(())
    }

    pub fn write_output_file(&self) -> Result<(), Box<dyn Error>> {
        let mut f = File::create(self.output_file.clone())?;
        f.write(self.magic.as_bytes());
        let header = format!("{} {}\n{}\n", self.width, self.height, self.bit_depth);
        f.write(header.as_bytes());
        
        let image_data = self.image_data.as_ref().unwrap();
        let image_data = image_data.try_borrow()?;
        
        println!("{:?}", &image_data.len());
        f.write(&image_data[..]);

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::PPMWriter;

    #[test]
    fn can_create_new_writer() {
        let ppm_writer = PPMWriter::new(
            "", 
            200, 
            100, 
            255);
        assert_eq!(ppm_writer.width, 200);
        assert_eq!(ppm_writer.height, 100);
        assert_eq!(ppm_writer.bit_depth, 255);
    }
}
