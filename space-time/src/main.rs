use std::time::Duration;

use uninutsh::{
    image::SpritesManager,
    window::{EventHandler, Window, WindowEvent, WindowOptions},
    Vector2,
};

struct Manager {
    sprites: SpritesManager,
}

impl Manager {
    fn new() -> Manager {
        let mut sprites = SpritesManager::new();
        sprites.put(
            "background0x0",
            ["resources", "sprites", "background0x0.bmp"]
                .iter()
                .collect(),
        );
        Manager { sprites }
    }
}

impl EventHandler for Manager {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window) {
        match event {
            WindowEvent::Exit => {
                window.close();
            }
            WindowEvent::Draw => {
                let mut graphics = window.graphics().take().unwrap();
                let sprite = self.sprites.get("background0x0");
                graphics.put_sprite(sprite, 0, 0);
                graphics.apply();
                window.return_graphics(Some(graphics));
            }
            WindowEvent::Update(_delta) => {
                window.redraw();
            }
        }
    }
}

fn main() {
    let window_options = WindowOptions {
        title: String::from("space-time"),
        size: Vector2::new(1280, 720),
        graphics_size: Vector2::new(512, 256),
        update_delta: Duration::from_millis(16),
    };
    let window = Window::new(window_options, Box::new(Manager::new()));
    window.event_loop();
}
