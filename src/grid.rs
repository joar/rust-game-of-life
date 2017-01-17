extern crate graphics;

use std::vec;
use std::mem;
use graphics::math::Vec2d;

pub enum Neighbor {
    Alive(Block),
    Dead(Location),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub loc: Location,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Option<Block>>>,
    blocks: Vec<Block>,
}

impl Block {
    pub fn new(location: Location) -> Block {
        Block {
            loc: location
        }
    }
}

impl Location {
    pub fn new(x: usize, y: usize) -> Location {
        Location {
            x: x,
            y: y,
        }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut rows: Vec<Vec<Option<Block>>> = vec!();
        rows.reserve(height);

        for _ in 0..height {
            rows.push(Vec::with_capacity(width));
        }

        Grid {
            grid: rows,
            blocks: vec!(),
        }
    }

    pub fn insert(&mut self, block: Block) {
        if self.valid(block.loc) {
            if let Some(row) = self.grid.get_mut(block.loc.y) {
                match row.get_mut(block.loc.x) {
                    Some(grid_ref) => {
                        match *grid_ref {
                            Some(_block) => {
                                self.blocks.retain(|b| b != &_block);
                                mem::replace(grid_ref, Some(block));
                            },
                            None => {
                                *grid_ref = Some(block);
                                self.blocks.push(grid_ref.unwrap())
                            }
                        }
                    },
                    None => {},
                }
            }
        }

//
//        if let Some(row) = self.grid.get_mut(loc.y) {
//            if let Some(gr_loc) = row.get_mut(loc.x) {
//                mem::replace(&gr_loc, )
//                if *gr_loc == None {
//                    *gr_loc = Some(block);
//                    self.blocks.push(gr_loc.unwrap())
//                } else if gr_loc.unwrap() != block {
//                    let idx = {
//                        let old = &gr_loc.unwrap();
//                        let mut i = 0;
//                        let len = self.blocks.len();
//
//                        while i < len {
//                            if old == &self.blocks[i] {
//                                break;
//                            }
//                            i += 1;
//                        }
//                        i
//                    };
//
//                    self.blocks[idx] = gr_loc.unwrap();
//                }
//            }
//        }
    }

//    pub fn neighbors(&self, cell: &Cell) -> Vec<Neighbor> {
//        let mut vec = vec!();
//
//        if self.valid(cell.loc) {
//            let start = {
//                if cell.loc.x > 0 {
//                    -1
//                } else {
//                    0
//                }
//            };
//            let end = {
//                if cell.loc.x < self.grid.len() - 1 {
//                    2
//                } else {
//                    1
//                }
//            };
//
//            for i in start..end {
//                if cell.loc.y > 0 {
//					let row = &self.grid[cell.loc.y - 1];
//					let xpos = i + cell.loc.x as u64;
//
//					vec.push(match row[xpos] {
//						Some(blk) => Neighbor::Alive(blk),
//						None => Neighbor::Dead(
//                            Location::new(xpos, cell.loc.y - 1))
//					});
//				}
//				if cell.loc.y < self.grid.len() - 1 {
//                    let row = &self.grid[cell.loc.y + 1];
//                    let xpos = i + cell.loc.x as u64;
//
//                    vec.push(match row[xpos] {
//                        Some(blk) => Neighbor::Alive(blk),
//                        None => Neighbor::Dead(
//                            Location::new(xpos, cell.loc.y + 1))
//                    });
//                }
//            }
//
//			let row = &self.grid[cell.loc.y];
//
//			if cell.loc.x > 0 {
//				vec.push(match row[cell.loc.x - 1] {
//					Some(blk) => Neighbor::Alive(blk),
//					None => Neighbor::Dead(
//                        Location::new(cell.loc.x - 1, cell.loc.y))
//				});
//			}
//
//            if cell.loc.x < self.grid[0].len() - 1 {
//                vec.push(match row[cell.loc.x + 1] {
//                    Some(cell_) => Neighbor::Alive(cell_),
//                    None => Neighbor::Dead(
//                        Location::new(cell.loc.x + 1, cell.loc.y))
//                });
//            }
//        }
//
//        vec
//    }
//
//    pub fn live_neighbors(&self, block: &Cell) -> Vec<Cell> {
//        let mut live = vec!();
//
//        for neighbor in self.neighbors(block).move_iter() {
//            match neighbor {
//                Neighbor::Alive(cell_) => live.push(cell_),
//                _ => {}
//            }
//        }
//        live
//    }
//
//    pub fn dead_neighbors(&self, block: &Cell) -> Vec<Location> {
//        let mut dead = vec!();
//
//        for neighbor in self.neighbors(block).move_iter() {
//            match neighbor {
//                Neighbor::Dead(loc) => dead.push(loc),
//                _ => {}
//            }
//        }
//        dead
//    }


    pub fn contains(&self, block: &Block) -> bool {
        if self.valid(block.loc) {
            self.grid[block.loc.y][block.loc.x].is_some()
        } else {
            false
        }
    }

    pub fn get_cells(&self) -> Vec<Block> {
        self.blocks
    }

    // Note: (Y, X)
    pub fn size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    pub fn height(&self) -> usize { self.grid.len() }
    pub fn width(&self) -> usize { self.grid[0].len() }

	#[inline]
    fn valid(&self, location: Location) -> bool {
        location.y < self.grid.len() && location.x < self.grid[0].len()
    }
}
