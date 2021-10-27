use std::num::Wrapping;
use std::time::Duration;
use uninutsh::image::Color;
use uninutsh::ui::window::EventHandler;
use uninutsh::ui::window::Window;
use uninutsh::ui::window::WindowEvent;
use uninutsh::Vector2;

#[derive(Copy, Clone)]
struct Cell {
    _position: Vector2<u32>,
    color: u32,
}

impl Cell {
    fn new(width: u32, height: u32, color: u32) -> Cell {
        let position = Vector2::new(width, height);
        Cell {
            _position: position,
            color,
        }
    }
}

struct Nutshell {
    cells: Vec<Cell>,
    back_cells: Vec<Cell>,
    size: Vector2<u32>,
    colors_count: u32,
    colors: Vec<Color>,
}

impl Nutshell {
    fn new(width: u32, height: u32, colors_count: u32) -> Nutshell {
        let size = Vector2::new(width, height);
        let mut cells = Vec::with_capacity(width as usize * height as usize);
        let mut back_cells = Vec::with_capacity(width as usize * height as usize);
        let mut color: Wrapping<u32> = Wrapping(1);
        let mut a: Wrapping<u32> = Wrapping(0);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y, 0));
                back_cells.push(Cell::new(x, y, color.0 % colors_count));
                let saved = color;
                color += a;
                a = saved;
            }
        }
        let mut colors = Vec::with_capacity(colors_count as usize);
        let mut value = 0.;
        let adder = 1. / (colors_count - 1) as f64;
        for _ci in 0..colors_count {
            colors.push(Color::from_hsb([0., 1. / 3., value], 255));
            value += adder;
        }
        Nutshell {
            cells,
            back_cells,
            size,
            colors_count,
            colors,
        }
    }
    fn index_at(&self, x: u32, y: u32) -> usize {
        y as usize * self.size.x as usize + x as usize
    }
    fn set_color_at(&mut self, x: u32, y: u32, color: u32) {
        let index = self.index_at(x, y);
        self.cells[index].color = color % self.colors_count;
    }
    fn set_back_color_at(&mut self, x: u32, y: u32, color: u32) {
        let index = self.index_at(x, y);
        self.back_cells[index].color = color % self.colors_count;
    }
    fn back_color_at(&self, x: u32, y: u32) -> u32 {
        let index = self.index_at(x, y);
        self.back_cells[index].color
    }
    fn color_at(&self, x: u32, y: u32) -> u32 {
        let index = self.index_at(x, y);
        self.cells[index].color
    }
    fn fill_back(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let color = self.color_at(x, y);
                self.set_back_color_at(x, y, color);
            }
        }
    }
    fn left(&self, i: u32) -> u32 {
        match i {
            0 => self.size.x - 1,
            _ => i - 1,
        }
    }
    fn up(&self, i: u32) -> u32 {
        match i {
            0 => self.size.y - 1,
            _ => i - 1,
        }
    }
    fn rigth(&self, i: u32) -> u32 {
        if i == self.size.x - 1 {
            return 0;
        }
        i + 1
    }
    fn down(&self, i: u32) -> u32 {
        if i == self.size.y - 1 {
            return 0;
        }
        i + 1
    }
    fn get_sum(&self, x: u32, y: u32) -> u32 {
        let left_x = self.left(x);
        let up_y = self.up(y);
        let right_x = self.rigth(x);
        let down_y = self.down(y);
        let left = self.back_color_at(left_x, y);
        let up = self.back_color_at(x, up_y);
        let right = self.back_color_at(right_x, y);
        let down = self.back_color_at(x, down_y);
        left + up + right + down
    }
    fn iterate(&mut self) {
        self.fill_back();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let sum = self.get_sum(x, y);
                self.set_color_at(x, y, sum);
            }
        }
    }
}

struct NutshellManager {
    nutshell: Nutshell,
    delta: Duration,
}

impl NutshellManager {
    fn new() -> NutshellManager {
        let width = 128;
        let height = 64;
        let mut nutshell = Nutshell::new(width, height, 6);

        nutshell.set_color_at(width / 2 - 1, height / 2 - 1, 1);
        nutshell.set_color_at(width / 2 - 1, height / 2, 1);
        nutshell.set_color_at(width / 2, height / 2 - 1, 1);
        nutshell.set_color_at(width / 2, height / 2, 1);

        NutshellManager {
            nutshell,
            delta: Duration::from_secs(0),
        }
    }
}

impl EventHandler for NutshellManager {
    fn handle_event(&mut self, event: WindowEvent, window: &mut Window) {
        match event {
            WindowEvent::Update(delta) => {
                if self.delta >= Duration::from_millis(16*8) {
                    println!("delta {}", self.delta.as_millis());
                    self.nutshell.iterate();
                    window.redraw();
                    self.delta = Duration::from_secs(0);
                }
                self.delta += delta;
            }
            WindowEvent::Exit => {
                println!("Exit event");
                window.close();
            }
            WindowEvent::Draw => {
                let mut graphics = window.graphics().expect("Can not find the graphics object");
                for y in 0..self.nutshell.size.y {
                    for x in 0..self.nutshell.size.x {
                        let color_level = self.nutshell.color_at(x, y);
                        let color = self.nutshell.colors[color_level as usize];
                        graphics.set_color(color);
                        graphics.put(x, y);
                    }
                }
                graphics.apply();
                window.return_graphics(Some(graphics));
            }
        }
    }
}

fn main() {
    let manager = NutshellManager::new();
    let width = manager.nutshell.size.x;
    let height = manager.nutshell.size.y;
    let window = Window::new("Nutshell", Some(Box::new(manager)), width, height);
    window.event_loop();
}
