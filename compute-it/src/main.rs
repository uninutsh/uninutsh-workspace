//#![allow(warnings)]
use uninutsh_framework::ui::window::Window;

fn main() {
    println!("Hello, world!");
    uninutsh_framework::greet();
    let window = Window::new("compute-it");
    window.event_loop();
}
