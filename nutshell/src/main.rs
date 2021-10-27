use std::time::Duration;
use uninutsh::image::Color;
use uninutsh::ui::window::EventHandler;
use uninutsh::ui::window::Window;
use uninutsh::ui::window::WindowEvent;
use uninutsh::Vector2;

#[derive(Clone)]
struct Cell {
    _position: Vector2<u32>,
    neighbors: Vec<Vector2<u32>>,
    color: u64,
}

impl Cell {
    fn new(width: u32, height: u32, color: u64) -> Cell {
        let position = Vector2::new(width, height);
        let neighbors = Vec::with_capacity(16);
        Cell {
            _position: position,
            neighbors,
            color,
        }
    }
}

struct Nutshell {
    cells: Vec<Cell>,
    back_cells: Vec<Cell>,
    size: Vector2<u32>,
    colors_count: u64,
    colors: Vec<Color>,
}

impl Nutshell {
    fn add_neighbors(&mut self, x: u32, y: u32, neighbours: Vec<Vector2<u32>>) {
        let index = self.index_at(x, y);
        for neighbor in neighbours {
            self.cells[index].neighbors.push(neighbor);
            self.back_cells[index].neighbors.push(neighbor);
        }
    }
    fn neighborhood(&self, x: u32, y: u32, radius: u32) -> Vec<Vector2<u32>> {
        let side_length = radius * 2 + 1;
        let center = Vector2::new(x, y);
        let mut position = center;
        for _i in 0..radius {
            position = self.left_pos(position.x, position.y);
        }
        for _i in 0..radius {
            position = self.up_pos(position.x, position.y);
        }
        let mut row_pos = position;
        let mut neighborhood = Vec::with_capacity(side_length as usize * side_length as usize);
        for _y in 0..side_length {
            position = row_pos;
            for _x in 0..side_length {
                /*
                let mut dis_x;
                if center.x > position.x {
                    dis_x = center.x - position.x;
                } else {
                    dis_x = position.x - center.x;
                }
                if dis_x > radius {
                    dis_x = self.size.x - dis_x;
                }
                let mut dis_y;
                if center.y > position.y {
                    dis_y = center.y - position.y;
                } else {
                    dis_y = position.y - center.y;
                }
                if dis_y > radius {
                    dis_y = self.size.y - dis_y;
                }
                if dis_x + dis_y <= radius {
                    neighborhood.push(position);
                }
                */
                neighborhood.push(position);
                position = self.rigth_pos(position.x, position.y);
            }
            row_pos = self.down_pos(row_pos.x, row_pos.y);
        }
        neighborhood
    }
    fn new(width: u32, height: u32, colors_count: u64, radius: u32) -> Nutshell {
        let size = Vector2::new(width, height);
        let mut cells = Vec::with_capacity(width as usize * height as usize);
        let mut back_cells = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y, 0));
                back_cells.push(Cell::new(x, y, 0));
            }
        }
        let mut colors = Vec::with_capacity(colors_count as usize);
        let mut value = 0.;
        let adder = 1. / (colors_count - 1) as f64;
        for _ci in 0..colors_count {
            colors.push(Color::from_hsb([180., 1. / 1., value], 255));
            value += adder;
        }
        let mut nutshell = Nutshell {
            cells,
            back_cells,
            size,
            colors_count,
            colors,
        };
        //let line = width / 4;
        //let anti_line = width - 1 - line;
        for y in 0..height {
            for x in 0..width {
                let neighborhood = nutshell.neighborhood(x, y, radius);
                /*
                if y % 64 == 0 {
                    if x == line {
                        neighbors.push(Vector2::new(anti_line, y));
                    } else if x == anti_line {
                        neighbors.push(Vector2::new(line, y));
                    }
                }
                */
                nutshell.add_neighbors(x, y, neighborhood);
            }
        }
        nutshell
    }
    fn index_at(&self, x: u32, y: u32) -> usize {
        y as usize * self.size.x as usize + x as usize
    }
    fn set_color_at(&mut self, x: u32, y: u32, color: u64) {
        let index = self.index_at(x, y);
        self.cells[index].color = color % self.colors_count;
    }
    fn set_back_color_at(&mut self, x: u32, y: u32, color: u64) {
        let index = self.index_at(x, y);
        self.back_cells[index].color = color % self.colors_count;
    }
    fn back_color(&self, position: Vector2<u32>) -> u64 {
        self.back_color_at(position.x, position.y)
    }
    fn back_color_at(&self, x: u32, y: u32) -> u64 {
        let index = self.index_at(x, y);
        self.back_cells[index].color
    }
    fn color_at(&self, x: u32, y: u32) -> u64 {
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
    fn right(&self, i: u32) -> u32 {
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
    fn rigth_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(self.right(x), y)
    }
    fn left_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(self.left(x), y)
    }
    fn down_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(x, self.down(y))
    }
    fn up_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(x, self.up(y))
    }
    fn get_sum(&self, x: u32, y: u32) -> u64 {
        let index = self.index_at(x, y);
        let mut sum = self.back_cells[index].color;
        for neighbor in &self.back_cells[index].neighbors {
            sum += self.back_color(*neighbor);
        }
        sum
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
        let width = 63;
        let height = 63;
        let mut nutshell = Nutshell::new(width, height, 4, 2);

        //nutshell.set_color_at(width / 2 - 1, height / 2 - 1, 1);
        //nutshell.set_color_at(width / 2 - 1, height / 2, 1);
        //nutshell.set_color_at(width / 2, height / 2 - 1, 1);
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
                if self.delta >= Duration::from_millis(20 * 50) {
                    //println!("delta {}", self.delta.as_millis());
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
