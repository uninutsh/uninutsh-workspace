use super::{Color, Sprite};
use image::{imageops, io::Reader, Rgba, RgbaImage};
use imageproc::{drawing, rect::Rect};
use palette::{FromColor, Hsv, Srgb};
use std::path::PathBuf;

pub struct Data {
    buffer: RgbaImage,
}

pub fn hsb_to_rgb(hsb: [f64; 3]) -> [u8; 3] {
    let internal_hsv = Hsv::new(hsb[0], hsb[1], hsb[2]);
    let internal_rgb = Srgb::from_color(internal_hsv);
    [
        (internal_rgb.red * 255.) as u8,
        (internal_rgb.green * 255.) as u8,
        (internal_rgb.blue * 255.) as u8,
    ]
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
    pub fn update(&mut self, pixels: Option<Vec<u8>>) -> Option<Vec<u8>> {
        match pixels {
            Some(mut vector) => {
                for y in 0..self.height() as usize {
                    let iy = self.height() as usize - y - 1;
                    for x in 0..self.width() as usize {
                        let pixel = self.buffer.get_pixel(x as u32, y as u32).0;
                        vector[iy * self.width() as usize * 4 + x * 4 + 0] = pixel[0];
                        vector[iy * self.width() as usize * 4 + x * 4 + 1] = pixel[1];
                        vector[iy * self.width() as usize * 4 + x * 4 + 2] = pixel[2];
                        vector[iy * self.width() as usize * 4 + x * 4 + 3] = pixel[3];
                    }
                }
                Some(vector)
            }
            None => None,
        }
    }
    pub fn put_sprite(&mut self, sprite: &mut Sprite, x: u32, y: u32) {
        imageops::overlay(&mut self.buffer, &sprite.internals.buffer, x, y)
    }
    pub fn put(&mut self, x: u32, y: u32, color: Color) {
        self.buffer.put_pixel(x, y, Rgba([color.rgb[0], color.rgb[1], color.rgb[2], color.alpha]))
    }
}
