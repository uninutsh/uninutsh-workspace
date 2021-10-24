use super::Sprite;
use image::{imageops, io::Reader, Rgba, RgbaImage};
use imageproc::{drawing, rect::Rect};
use std::path::PathBuf;
use std::rc::Weak;

pub struct Data {
    buffer: RgbaImage,
}

impl Data {
    pub fn new(width: u32, height: u32) -> Data {
        let mut buffer = RgbaImage::new(width, height);
        drawing::draw_filled_rect_mut(
            &mut buffer,
            Rect::at(0, 0).of_size(width, height),
            Rgba([255, 255, 255, 255]),
        );
        Data { buffer }
    }
    pub fn load(path: PathBuf) -> Data {
        let buffer = Reader::open(&path)
            .expect(format!("open at {:?}", path.as_os_str()).as_str())
            .decode()
            .expect(format!("decode at {:?}", path.as_os_str()).as_str())
            .to_rgba8();
        Data { buffer }
    }
    pub fn width(&self) -> u32 {
        self.buffer.width()
    }
    pub fn height(&self) -> u32 {
        self.buffer.height()
    }
    pub fn update(&mut self, pixels: Weak<Vec<u8>>) {
        let pixels_vector = unsafe { &mut *(pixels.as_ptr() as *mut Vec<u8>) };
        for y in 0..self.height() as usize {
            let iy = self.height() as usize - y - 1;
            for x in 0..self.width() as usize {
                let pixel = self.buffer.get_pixel(x as u32, y as u32).0;
                pixels_vector[iy * self.width() as usize * 4 + x * 4 + 0] = pixel[0];
                pixels_vector[iy * self.width() as usize * 4 + x * 4 + 1] = pixel[1];
                pixels_vector[iy * self.width() as usize * 4 + x * 4 + 2] = pixel[2];
                pixels_vector[iy * self.width() as usize * 4 + x * 4 + 3] = pixel[3];
            }
        }
    }
    pub fn put_sprite(&mut self, sprite: &mut Sprite, x: u32, y: u32) {
        imageops::overlay(&mut self.buffer, &sprite.internals.buffer, x, y)
    }
}
