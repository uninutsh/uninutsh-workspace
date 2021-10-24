mod camera;
mod room;
use camera::Camera;
use room::Room;
use uninutsh::image::Graphics;
use uninutsh::image::Sprite;
use uninutsh::image::SpritesManager;
use uninutsh::ui::window::{EventHandler, Window, WindowEvent};

struct TheGameHandler {
    room: Room,
    camera: Camera,
    sprites: SpritesManager,
}

impl TheGameHandler {
    fn new() -> TheGameHandler {
        let room = Room::new(16, 8);
        let camera = Camera::new(0., 0.);
        let mut sprites = SpritesManager::new();
        sprites.put(
            "background",
            ["resources", "sprites", "background-0x0.png"]
                .iter()
                .collect(),
        );
        TheGameHandler {
            room,
            camera,
            sprites,
        }
    }
    fn boxed() -> Box<TheGameHandler> {
        Box::new(TheGameHandler::new())
    }
}

impl EventHandler for TheGameHandler {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window) {
        match event {
            WindowEvent::Update(_delta) => {
                //println!("update {}", delta.as_millis());
                window.redraw();
            }
            WindowEvent::Exit => {
                window.close();
            }
            WindowEvent::Draw(pointer) => {
                let graphics = Graphics::reference(pointer);
                let sprite_ptr = self.sprites.get("background");
                let sprite = Sprite::reference(sprite_ptr);
                graphics.put_sprite(sprite, 0, 0);
                graphics.apply();
                //graphics.put_sprite(sprite, x, y);
            }
        }
    }
}

fn main() {
    let window = Window::new("the-game", Some(TheGameHandler::boxed()));
    window.event_loop();
}
