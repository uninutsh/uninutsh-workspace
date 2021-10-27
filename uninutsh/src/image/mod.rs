mod internals;

use internals::Data;
use std::collections::hash_map::HashMap;
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub struct Color {
    _hsb: [f64; 3],
    rgb: [u8; 3],
    alpha: u8,
}

impl Color {
    pub fn from_hsb(hsb: [f64; 3], alpha: u8) -> Color {
        let rgb = internals::hsb_to_rgb(hsb);
        Color { _hsb: hsb, rgb, alpha }
    }
}

pub struct Graphics {
    sprite: Sprite,
    color: Color,
}

impl Graphics {
    pub fn new(width: u32, height: u32) -> Graphics {
        let sprite = Sprite::new(width, height);
        let color = Color::from_hsb([0., 0., 0.], 255);
        Graphics { sprite, color }
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn width(&self) -> u32 {
        self.sprite.width()
    }
    pub fn height(&self) -> u32 {
        self.sprite.height()
    }
    pub fn pixels(&mut self) -> Option<Vec<u8>> {
        self.sprite.pixels()
    }
    pub fn return_pixels(&mut self, pixels: Option<Vec<u8>>) {
        self.sprite.return_pixels(pixels);
    }
    pub fn put_sprite(&mut self, sprite: &mut Sprite, x: u32, y: u32) {
        self.sprite.put_sprite(sprite, x, y);
    }
    pub fn put(&mut self, x: u32, y: u32) {
        self.sprite.put(x, y, self.color);
    }
    pub fn apply(&mut self) {
        self.sprite.update();
    }
}

pub struct Sprite {
    internals: Data,
    pixels: Option<Vec<u8>>,
}

impl Sprite {
    pub fn new(width: u32, height: u32) -> Sprite {
        let internals = Data::new(width, height);
        let pixels = Sprite::create_pixels(width, height);
        let mut sprite = Sprite { internals, pixels };
        sprite.update();
        sprite
    }
    pub fn load(path: PathBuf) -> Sprite {
        let internals = Data::load(path);
        let pixels = Sprite::create_pixels(internals.width(), internals.height());
        Sprite { internals, pixels }
    }
    fn create_pixels(width: u32, height: u32) -> Option<Vec<u8>> {
        let mut vector = Vec::with_capacity(width as usize * height as usize * 4);
        for _y in 0..height as usize {
            for _x in 0..width as usize {
                vector.push(255);
                vector.push(255);
                vector.push(255);
                vector.push(255);
            }
        }
        Some(vector)
    }
    pub fn width(&self) -> u32 {
        self.internals.width()
    }
    pub fn height(&self) -> u32 {
        self.internals.height()
    }
    pub fn update(&mut self) {
        let pixels = self.pixels();
        let returned = self.internals.update(pixels);
        self.return_pixels(returned);
    }
    pub fn pixels(&mut self) -> Option<Vec<u8>> {
        self.pixels.take()
    }
    pub fn return_pixels(&mut self, pixels: Option<Vec<u8>>) {
        self.pixels = pixels;
    }
    pub fn put_sprite(&mut self, sprite: &mut Sprite, x: u32, y: u32) {
        self.internals.put_sprite(sprite, x, y);
    }
    pub fn put(&mut self, x: u32, y: u32, color: Color) {
        self.internals.put(x, y, color);
    }
}

pub struct SpritesManager {
    dictionary: HashMap<String, Option<Sprite>>,
}

impl SpritesManager {
    pub fn new() -> SpritesManager {
        let dictionary = HashMap::new();
        SpritesManager { dictionary }
    }
    pub fn put(&mut self, name: &str, path: PathBuf) {
        self.dictionary
            .insert(name.to_string(), Some(Sprite::load(path)));
    }
    pub fn get(&mut self, name: &str) -> Option<Sprite> {
        self.dictionary
            .get_mut(name)
            .expect("SpritesManager::get")
            .take()
    }
}
