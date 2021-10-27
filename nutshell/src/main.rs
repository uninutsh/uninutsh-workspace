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
    saturation: u64,
    brightness: u64,
}

impl Cell {
    fn new(width: u32, height: u32, color: u64, saturation: u64, brightness: u64) -> Cell {
        let position = Vector2::new(width, height);
        let neighbors = Vec::with_capacity(16);
        Cell {
            _position: position,
            neighbors,
            color,
            saturation,
            brightness,
        }
    }
}

struct Nutshell {
    cells: Vec<Cell>,
    back_cells: Vec<Cell>,
    size: Vector2<u32>,
    colors: u64,
    saturations: u64,
    brightnessess: u64,
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
    fn new(
        width: u32,
        height: u32,
        colors: u64,
        saturations: u64,
        brightnessess: u64,
        radius: u32,
    ) -> Nutshell {
        let size = Vector2::new(width, height);
        let mut cells = Vec::with_capacity(width as usize * height as usize);
        let mut back_cells = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y, 0, 0, 0));
                back_cells.push(Cell::new(x, y, 0, 0, 0));
            }
        }

        let mut nutshell = Nutshell {
            cells,
            back_cells,
            size,
            colors,
            saturations,
            brightnessess,
        };
        for y in 0..height {
            for x in 0..width {
                let neighborhood = nutshell.neighborhood(x, y, radius);
                nutshell.add_neighbors(x, y, neighborhood);
            }
        }
        nutshell
    }
    fn index_at(&self, x: u32, y: u32) -> usize {
        y as usize * self.size.x as usize + x as usize
    }
    fn fill_back(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let index = self.index_at(x, y);
                let color = self.cells[index].color;
                self.back_cells[index].color = color;
                let saturation = self.cells[index].saturation;
                self.back_cells[index].saturation = saturation;
                let brightness = self.cells[index].brightness;
                self.back_cells[index].brightness = brightness;
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
    fn iterate_at(&mut self, x: u32, y: u32) {
        let index = self.index_at(x, y);
        let mut color = self.back_cells[index].color;
        let mut saturation = self.back_cells[index].saturation;
        let mut brightness = self.back_cells[index].brightness;
        for neighbor in &self.back_cells[index].neighbors {
            let index = self.index_at(neighbor.x, neighbor.y);
            color += self.back_cells[index].color;
            saturation += self.back_cells[index].saturation;
            brightness += self.back_cells[index].brightness;
        }
        self.cells[index].color = color % self.colors;
        self.cells[index].saturation = saturation % self.saturations;
        self.cells[index].brightness = brightness % self.brightnessess;
    }
    fn iterate(&mut self) {
        self.fill_back();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                self.iterate_at(x, y);
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
        let width = 127;
        let height = 63;
        let mut nutshell = Nutshell::new(width, height, 5, 4, 3, 1);

        //nutshell.set_color_at(width / 2 - 1, height / 2 - 1, 1);
        //nutshell.set_color_at(width / 2 - 1, height / 2, 1);
        //nutshell.set_color_at(width / 2, height / 2 - 1, 1);
        let index = nutshell.index_at(width / 2, height / 2);
        nutshell.cells[index].color = 1;
        nutshell.cells[index].saturation = 1;
        nutshell.cells[index].brightness = 1;
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
                if self.delta >= Duration::from_millis(20 * 25) {
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
                        let index = self.nutshell.index_at(x, y);
                        let color_level = self.nutshell.cells[index].color;
                        let saturation_level = self.nutshell.cells[index].saturation;
                        let brightness_level = self.nutshell.cells[index].brightness;
                        let hue = color_level as f64 / self.nutshell.colors as f64;
                        let saturation =
                            saturation_level as f64 / (self.nutshell.saturations - 1) as f64;
                        let brightness =
                            brightness_level as f64 / (self.nutshell.brightnessess - 1) as f64;
                        let color = Color::from_hsb(
                            [(hue * 360. + 180.) % 360., saturation, brightness],
                            255,
                        );
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
