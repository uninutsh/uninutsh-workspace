mod internals;

use internals::Data;

pub struct Sprite {
    internals: Data,
    pixels: Option<Vec<u8>>,
}

impl Sprite {
    pub fn new(width: u32, height: u32) -> Sprite {
        let internals = Data::new(width, height);
        let mut vector = Vec::with_capacity(width as usize * height as usize * 4);
        for _y in 0..height as usize {
            for _x in 0..width as usize {
                vector.push(255);
                vector.push(255);
                vector.push(255);
                vector.push(255);
            }
        }
        let pixels = Some(vector);
        let mut sprite = Sprite { internals, pixels };
        sprite.update();
        sprite
    }
    pub fn width(&self) -> u32 {
        self.internals.width()
    }
    pub fn height(&self) -> u32 {
        self.internals.height()
    }
    pub fn update(&mut self) {
        let pixels = self.internals.update(self.pixels.take().unwrap());
        self.pixels = Some(pixels);
    }
    pub fn take_pixels(&mut self) -> Vec<u8> {
        self.pixels.take().unwrap()
    }
}
