extern crate graphics;
extern crate sdl2_window;

use std::vec;

pub enum Neighbor {
    Alive(Cell),
    Dead(Location),
}

#[derive(Clone, PartialEq)]
pub struct Cell {
    pub loc: Location,
}

#[derive(Clone, PartialEq)]
pub struct Location {
    pub x: u64,
    pub y: u64,
}

pub struct Grid {
    grid: Vec<Vec<Option<Cell>>>,
    cells: Vec<Cell>,
}

impl Cell {
    pub fn new(location: Location) {
        Cell {
            loc: location
        }
    }
}

impl Location {
    pub fn new(x: u64, y: u64) {
        Location {
            x: x,
            y: y,
        }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut rows: Vec<Vec<Option<Cell>>> = vec!();
        rows.reserve(height);

        for _ in 0..height {
            rows.push(Vec::with_capacity(width));
        }

        Grid {
            grid: rows,
            cells: vec!(),
        }
    }

    pub fn insert(&mut self, cell: Cell) {
        let loc = cell.loc;
        let gr_loc = self.grid.get_mut(loc.y).get_mut(loc.x);

        if *gr_loc == None {
            *gr_loc = Some(cell);
            self.cells.push(gr_loc.unwrap())
        } else if gr_loc.unwrap() != cell {
            let idx = {
                let old = gr_loc.get_ref();
                let mut i = 0;
                let len = self.cells.len();

                while i < len {
                    if old == &self.cells[i] {
                        break;
                    }
                    i += 1;
                }
                i
            }
            *gr_loc = Some(cell);
            *self.cells.get_mut(idx) = gr_loc.unwrap();
        }
    }

    pub fn neighbors(&self, cell: &Cell) -> Vec<Neighbor> {
        let mut vec = vec!();

        if self.valid(cell.loc) {
            let start = {
                if cell.loc.x > 0 {
                    -1
                } else {
                    0
                }
            };
            let end = {
                if cell.loc.x < self.grid.len() - 1 {
                    2
                } else {
                    1
                }
            };

            for i in start..end {
                if cell.loc.y > 0 {
					let row = &self.grid[cell.loc.y - 1];
					let xpos = i + cell.loc.x as u64;

					vec.push(match row[xpos] {
						Some(blk) => Neighbor::Alive(blk),
						None => Neighbor::Dead(
                            Location::new(xpos, cell.loc.y - 1))
					});
				}
				if cell.loc.y < self.grid.len() - 1 {
                    let row = &self.grid[cell.loc.y + 1];
                    let xpos = i + cell.loc.x as u64;

                    vec.push(match row[xpos] {
                        Some(blk) => Neighbor::Alive(blk),
                        None => Neighbor::Dead(
                            Location::new(xpos, cell.loc.y + 1))
                    });
                }
            }

			let row = &self.grid[cell.loc.y];

			if cell.loc.x > 0 {
				vec.push(match row[cell.loc.x - 1] {
					Some(blk) => Neighbor::Alive(blk),
					None => Neighbor::Dead(
                        Location::new(cell.loc.x - 1, cell.loc.y))
				});
			}

            if cell.loc.x < self.grid[0].len() - 1 {
                vec.push(match row[cell.loc.x + 1] {
                    Some(cell_) => Neighbor::Alive(cell_),
                    None => Neighbor::Dead(
                        Location::new(cell.loc.x + 1, cell.loc.y))
                });
            }
        }

        vec
    }

    pub fn live_neighbors(&self, block: &Cell) -> Vec<Cell> {
        let mut live = vec!();

        for neighbor in self.neighbors(block).move_iter() {
            match neighbor {
                Neighbor::Alive(cell_) => live.push(cell_),
                _ => {}
            }
        }
        live
    }

    pub fn dead_neighbors(&self, block: &Cell) -> Vec<Location> {
        let mut dead = vec!();

        for neighbor in self.neighbors(block).move_iter() {
            match neighbor {
                Neighbor::Dead(loc) => dead.push(loc),
                _ => {}
            }
        }
        dead
    }


    pub fn contains(&self, block: &Cell) -> bool {
        if self.valid(block.loc.x, block.loc.y) {
            self.grid[block.loc.y][block.loc.x].is_some()
        } else {
            false
        }
    }

    pub fn cells_iter(&self) {
        self.cells.iter()
    }

    // Note: (Y, X)
    pub fn size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

	#[inline]
    fn valid(&self, location: Location) -> bool {
        location.y < self.grid.len() && location.x < self.grid[0].len()
    }
}
