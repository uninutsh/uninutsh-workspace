mod internals;
use crate::image::Graphics;
use crate::Rectangle;
use std::ffi::c_void;
use std::rc::Rc;
use std::rc::Weak;
use std::time::Duration;

pub enum WindowEvent {
    Update(Duration),
    Draw(Weak<Graphics>),
    Exit,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window);
}

pub struct Window {
    size_ratio: f64,
    rectangle: Rectangle<i32>,
    graphics: Rc<Graphics>,
    internals: Option<internals::Data>,
    handler: Option<Box<dyn EventHandler>>,
    must_close: bool,
    must_redraw: bool,
}

impl Window {
    pub fn graphics(&self) -> Weak<Graphics> {
        Rc::downgrade(&self.graphics)
    }
    pub fn new(title: &str, handler: Option<Box<dyn EventHandler>>) -> Window {
        let internals = Some(internals::Data::new(title));
        let graphics = Rc::new(Graphics::new(512, 256));
        let rectangle = Rectangle::new(0, 0, 0, 0);
        let size_ratio = 2. / 1.;
        Window {
            size_ratio,
            rectangle,
            graphics,
            internals,
            handler,
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
    pub fn pixels_ptr(&self) -> *const c_void {
        let vector = unsafe { &*self.graphics.pixels().as_ptr() };
        vector.as_ptr() as *const c_void
    }
}
