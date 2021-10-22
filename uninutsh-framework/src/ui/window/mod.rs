mod internals;
use super::image::Sprite;
use std::time::Duration;

pub enum WindowEvent {
    Update(Duration),
    Exit,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window);
}

pub struct Window {
    sprite: Sprite,
    internals: Option<internals::Data>,
    handler: Option<Box<dyn EventHandler>>,
    must_close: bool,
    must_redraw: bool,
}

impl Window {
    pub fn new(title: &str, handler: Option<Box<dyn EventHandler>>) -> Window {
        let internals = Some(internals::Data::new(title));
        let sprite = Sprite::new(512, 256);
        Window {
            sprite,
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
}
