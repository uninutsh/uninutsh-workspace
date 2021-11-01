mod internals;
use crate::image::Graphics;
use crate::{Rectangle, Vector2};
use std::time::Duration;

pub enum WindowEvent {
    Update(Duration),
    Draw,
    Exit,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window);
}

pub struct WindowOptions {
    pub update_delta: Duration,
    pub title: String,
    pub size: Vector2<u32>,
    pub graphics_size: Vector2<u32>,
}

pub struct Window {
    size_ratio: f64,
    rectangle: Rectangle<i32>,
    graphics: Option<Graphics>,
    internals: Option<internals::Data>,
    handler: Option<Box<dyn EventHandler>>,
    must_close: bool,
    must_redraw: bool,
}

impl Window {
    pub fn graphics(&mut self) -> Option<Graphics> {
        self.graphics.take()
    }
    pub fn return_graphics(&mut self, graphics: Option<Graphics>) {
        self.graphics = graphics;
    }
    pub fn graphics_width(&self) -> Option<u32> {
        match &self.graphics {
            Some(graphics) => Some(graphics.width()),
            None => None,
        }
    }
    pub fn graphics_height(&self) -> Option<u32> {
        match &self.graphics {
            Some(graphics) => Some(graphics.height()),
            None => None,
        }
    }
    pub fn new(
        options: WindowOptions,
        handler: Box<dyn EventHandler>,
    ) -> Window {
        let internals = Some(internals::Data::new(options.title, options.size, options.update_delta));
        let graphics = Some(Graphics::new(options.graphics_size.x, options.graphics_size.y));
        let rectangle = Rectangle::new(0, 0, 0, 0);
        let size_ratio = options.graphics_size.x as f64 / options.graphics_size.y as f64;
        Window {
            size_ratio,
            rectangle,
            graphics,
            internals,
            handler: Some(handler),
            must_close: false,
            must_redraw: false,
        }
    }
    pub fn event_loop(mut self) {
        let internals = self.internals.take().unwrap();
        internals.event_loop(self);
    }
    pub fn close(&mut self) {
        self.must_close = true;
    }
    pub fn redraw(&mut self) {
        self.must_redraw = true;
    }
    fn excess_width(&self, window_size_ratio: f64) -> bool {
        if window_size_ratio > self.size_ratio {
            return true;
        }
        return false;
    }
    fn update_rectangle(&mut self, width: i32, height: i32) {
        let window_size_ratio = width as f64 / height as f64;
        if self.excess_width(window_size_ratio) {
            self.rectangle.position.y = 0;
            self.rectangle.size.y = height;
            self.rectangle.size.x = (height as f64 * self.size_ratio) as i32;
            let excess = width - self.rectangle.size.x;
            self.rectangle.position.x = excess / 2;
        } else {
            self.rectangle.position.x = 0;
            self.rectangle.size.x = width;
            self.rectangle.size.y = (width as f64 / self.size_ratio) as i32;
            let excess = height - self.rectangle.size.y;
            self.rectangle.position.y = excess / 2;
        }
    }
    pub fn pixels(&mut self) -> Option<Vec<u8>> {
        match &mut self.graphics {
            Some(graphics) => graphics.pixels(),
            None => None,
        }
    }
    pub fn return_pixels(&mut self, pixels: Option<Vec<u8>>) {
        match &mut self.graphics {
            Some(graphics) => {
                graphics.return_pixels(pixels);
            }
            None => {}
        }
    }
}
