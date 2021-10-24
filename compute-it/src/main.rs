//#![allow(warnings)]
//use std::time::Duration;
use uninutsh::ui::window::{EventHandler, Window, WindowEvent};

struct ComputeItHandler {}

impl ComputeItHandler {
    fn new() -> ComputeItHandler {
        ComputeItHandler {}
    }
    fn boxed() -> Box<ComputeItHandler> {
        Box::new(ComputeItHandler::new())
    }
}

impl EventHandler for ComputeItHandler {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window) {
        match event {
            WindowEvent::Update(_delta) => {
                //println!("update {}", delta.as_millis());
                window.redraw();
            }
            WindowEvent::Exit => {
                window.close();
            }
            _ => {}
        }
    }
}

fn main() {
    let window = Window::new("compute-it", Some(ComputeItHandler::boxed()));
    window.event_loop();
}
