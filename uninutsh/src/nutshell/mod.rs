use crate::Vector2;

pub struct Cell {
    pub neighbors: Vec<Vector2<u32>>,
    pub color: u64,
    pub saturation: u64,
    pub brightness: u64,
}

impl Cell {
    pub fn new(color: u64, saturation: u64, brightness: u64) -> Cell {
        let neighbors = Vec::with_capacity(16);
        Cell {
            neighbors,
            color,
            saturation,
            brightness,
        }
    }
}

pub struct Cells {
    pub front: Vec<Cell>,
    pub back: Vec<Cell>,
    pub colors: u64,
    pub saturations: u64,
    pub brightnessess: u64,
}

pub struct Nutshell {
    pub cells: Vec<Cells>,
    pub size: Vector2<u32>,
}

impl Nutshell {
    pub fn new(
        layers: usize,
        size: Vector2<u32>,
        colors: Vec<u64>,
        saturations: Vec<u64>,
        brightnessess: Vec<u64>,
    ) -> Nutshell {
        let mut cells = Vec::with_capacity(layers);
        for i in 0..layers {
            let mut front = Vec::with_capacity(size.x as usize * size.y as usize);
            let mut back = Vec::with_capacity(size.x as usize * size.y as usize);
            for _y in 0..size.y {
                for _x in 0..size.x {
                    front.push(Cell::new(0, 0, 0));
                    back.push(Cell::new(0, 0, 0));
                }
            }
            let pack = Cells {
                front,
                back,
                colors: colors[i],
                saturations: saturations[i],
                brightnessess: brightnessess[i],
            };
            cells.push(pack);
        }
        let mut nutshell = Nutshell { cells, size };
        for y in 0..size.y {
            for x in 0..size.x {
                let neighborhood = nutshell.neighborhood(x, y, 1);
                nutshell.add_neighbors(x, y, neighborhood);
            }
        }
        nutshell
    }
    pub fn neighborhood(&self, x: u32, y: u32, radius: u32) -> Vec<Vector2<u32>> {
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
                neighborhood.push(position);
                position = self.rigth_pos(position.x, position.y);
            }
            row_pos = self.down_pos(row_pos.x, row_pos.y);
        }
        neighborhood
    }

    pub fn left(&self, i: u32) -> u32 {
        match i {
            0 => self.size.x - 1,
            _ => i - 1,
        }
    }
    pub fn up(&self, i: u32) -> u32 {
        match i {
            0 => self.size.y - 1,
            _ => i - 1,
        }
    }
    pub fn right(&self, i: u32) -> u32 {
        if i == self.size.x - 1 {
            return 0;
        }
        i + 1
    }
    pub fn down(&self, i: u32) -> u32 {
        if i == self.size.y - 1 {
            return 0;
        }
        i + 1
    }
    pub fn rigth_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(self.right(x), y)
    }
    pub fn left_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(self.left(x), y)
    }
    pub fn down_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(x, self.down(y))
    }
    pub fn up_pos(&self, x: u32, y: u32) -> Vector2<u32> {
        Vector2::new(x, self.up(y))
    }
    pub fn index_at(&self, x: u32, y: u32) -> usize {
        y as usize * self.size.x as usize + x as usize
    }
    pub fn add_neighbors(&mut self, x: u32, y: u32, neighbours: Vec<Vector2<u32>>) {
        let index = self.index_at(x, y);
        for neighbor in neighbours {
            for cell_pack in &mut self.cells {
                cell_pack.front[index].neighbors.push(neighbor);
                cell_pack.back[index].neighbors.push(neighbor);
            }
        }
    }
    pub fn fill_back(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let index = self.index_at(x, y);
                for cell_pack in &mut self.cells {
                    cell_pack.back[index].color = cell_pack.front[index].color;
                    cell_pack.back[index].saturation = cell_pack.front[index].saturation;
                    cell_pack.back[index].brightness = cell_pack.front[index].brightness;

                }
            }
        }
    }
    pub fn height(&self) -> u32 {
        self.size.y
    }
    pub fn width(&self) -> u32 {
        self.size.x
    }
    pub fn back_cell(&mut self, layer: usize, x: u32, y: u32) -> &mut Cell {
        let index = self.index_at(x, y);
        &mut self.cells[layer].back[index]
    }
}
