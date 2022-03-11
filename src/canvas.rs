use std::{path::Path, fs::{OpenOptions}, io::Write};

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas { width, 
                height, 
                pixels: vec![Color::new(0.0, 0.0, 0.0); width * height] }
    }

    /// Writes the color to the given index
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if self.is_in_bound(x, y) {
            self.pixels[y * self.width + x] = color;
        }
    }

    /// Returns a reference to color at given indexes
    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        if self.is_in_bound(x, y) {
            return Some(&self.pixels[y * self.width + x]);
        }
        None
    }

    /// Returns if the given indexes are in bound
    pub fn is_in_bound(&self, x: usize, y: usize) -> bool {
        self.width > x && self.height > y
    }

    /// Returns ppm formatted string
    pub fn to_ppm_format(self) -> String {
        let mut result = String::with_capacity(self.width * self.height * 7);
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        result.push_str(&header);
        
        let mut line_length = 0;
        #[allow(unused_variables)]
        let mut item_count = 0;
        for color in &self.pixels {
            let ppm_colors = color.to_ppm_format();
            if line_length + ppm_colors.len() > 70 {
                line_length = 0;
                item_count = 0;
            }
            result.push_str(&*format!("{} ", ppm_colors.as_str()));
            item_count += 1;
            line_length += ppm_colors.len() + 1;      
        }
        result.push_str("\n");
        
        result
    }

    pub fn write_to_file(self, path: &str) {
        let ppm_formatted_text = self.to_ppm_format();
        let path = Path::new(path);
        let mut file = match  OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path) {
            Ok(f) => f,
            Err(e) => panic!("error while opening file: {}", e)
        };
    
        match file.write_all(ppm_formatted_text.as_bytes()) {
            Ok(_) => println!("The file is written to the {:?}", &path),
            Err(e) => panic!("Error while writing to file: {}", e)
        }
    
    }
}