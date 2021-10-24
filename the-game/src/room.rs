use uninutsh::Vector2;

pub trait Occupant {}

pub struct Room {
    size: Vector2<u32>,
    cells: Vec<Cell>,
}

pub struct Cell {
    position: Vector2<u32>,
    occupant: Option<Box<dyn Occupant>>,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        let position = Vector2::new(x, y);
        let occupant = None;
        Cell { position, occupant }
    }
}

impl Room {
    pub fn new(width: u32, height: u32) -> Room {
        let size = Vector2::new(width, height);
        let mut cells = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y));
            }
        }
        Room { size, cells }
    }
}
