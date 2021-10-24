mod internals;

use internals::Data;
use std::collections::hash_map::HashMap;
use std::path::PathBuf;
use std::rc::{Rc, Weak};

pub struct Graphics {
    sprite: Sprite,
}

impl Graphics {
    pub fn new(width: u32, height: u32) -> Graphics {
        let sprite = Sprite::new(width, height);
        Graphics { sprite }
    }
    pub fn width(&self) -> u32 {
        self.sprite.width()
    }
    pub fn height(&self) -> u32 {
        self.sprite.height()
    }
    pub fn pixels(&self) -> Weak<Vec<u8>> {
        self.sprite.pixels()
    }
    pub fn put_sprite(&mut self, sprite: &mut Sprite, x: u32, y: u32) {
        self.sprite.put_sprite(sprite, x, y);
    }
    pub fn reference<'a>(pointer: Weak<Graphics>) -> &'a mut Graphics {
        unsafe { &mut *(pointer.as_ptr() as *mut Graphics) }
    }
    pub fn apply(&mut self) {
        self.sprite.update();
    }
}

pub struct Sprite {
    internals: Data,
    pixels: Rc<Vec<u8>>,
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
    fn create_pixels(width: u32, height: u32) -> Rc<Vec<u8>> {
        let mut vector = Vec::with_capacity(width as usize * height as usize * 4);
        for _y in 0..height as usize {
            for _x in 0..width as usize {
                vector.push(255);
                vector.push(255);
                vector.push(255);
                vector.push(255);
            }
        }
        Rc::new(vector)
    }
    pub fn width(&self) -> u32 {
        self.internals.width()
    }
    pub fn height(&self) -> u32 {
        self.internals.height()
    }
    pub fn update(&mut self) {
        let ptr = self.pixels();
        self.internals.update(ptr);
    }
    pub fn pixels(&self) -> Weak<Vec<u8>> {
        Rc::downgrade(&self.pixels)
    }
    pub fn put_sprite(&mut self, sprite: &mut Sprite, x: u32, y: u32) {
        self.internals.put_sprite(sprite, x, y);
    }
    pub fn reference<'a>(pointer: Weak<Sprite>) -> &'a mut Sprite {
        unsafe { &mut *(pointer.as_ptr() as *mut Sprite) }
    }
}

pub struct SpritesManager {
    dictionary: HashMap<String, Rc<Sprite>>,
}

impl SpritesManager {
    pub fn new() -> SpritesManager {
        let dictionary = HashMap::new();
        SpritesManager { dictionary }
    }
    pub fn put(&mut self, name: &str, path: PathBuf) {
        self.dictionary
            .insert(name.to_string(), Rc::new(Sprite::load(path)));
    }
    pub fn get(&self, name: &str) -> Weak<Sprite> {
        Rc::downgrade(self.dictionary.get(name).expect("SpritesManager::get"))
    }
}
