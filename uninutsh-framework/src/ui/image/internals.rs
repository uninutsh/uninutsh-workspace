use image::{Rgba, RgbaImage};
use imageproc::{drawing, rect::Rect};

pub struct Data {
    buffer: RgbaImage,
}

impl Data {
    pub fn new(width: u32, height: u32) -> Data {
        let mut buffer = RgbaImage::new(width, height);
        drawing::draw_filled_rect_mut(
            &mut buffer,
            Rect::at(0, 0).of_size(width, height),
            Rgba([64, 0, 64, 255]),
        );
        Data { buffer }
    }
    pub fn width(&self) -> u32 {
        self.buffer.width()
    }
    pub fn height(&self) -> u32 {
        self.buffer.height()
    }
    pub fn update(&mut self, mut pixels: Vec<u8>) -> Vec<u8> {
        for y in 0..self.height() as usize {
            for x in 0..self.width() as usize {
                let pixel = self.buffer.get_pixel(x as u32, y as u32).0;
                pixels[y * self.width() as usize * 4 + x * 4 + 0] = pixel[0];
                pixels[y * self.width() as usize * 4 + x * 4 + 1] = pixel[1];
                pixels[y * self.width() as usize * 4 + x * 4 + 2] = pixel[2];
                pixels[y * self.width() as usize * 4 + x * 4 + 3] = pixel[3];
                //println!("pixel[0] = {}", pixel[0]);
                //println!("pixel[1] = {}", pixel[1]);
                //println!("pixel[2] = {}", pixel[2]);
                //println!("pixel[3] = {}", pixel[3]);
            }
        }
        return pixels;
    }
}
