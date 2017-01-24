use std::collections::HashSet;
use std::collections::hash_set;
use std::vec::Vec;
use std::iter::Iterator;
use std::cell::RefCell;
use rand::{ sample };
use rand;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
pub struct CellPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq,Eq,Clone,Copy)]
pub enum CellState {
    Alive,
    Dead,
}

pub struct WorldState {
    _alive_cells: HashSet<CellPosition>,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            _alive_cells: HashSet::new(),
        }
    }

    pub fn get_cell(&self, cell: CellPosition) -> CellState {
        match self._alive_cells.contains(&cell) {
            true => CellState::Alive,
            false => CellState::Dead,
        }
    }

    pub fn set_cell(&mut self, cell: CellPosition, state: CellState) {
        match state {
            CellState::Alive => { self._alive_cells.insert(cell); },
            CellState::Dead => { self._alive_cells.remove(&cell); }
        }
    }

    pub fn alive_cells(&self) -> hash_set::Iter<CellPosition> {
        self._alive_cells.iter()
    }

    pub fn neighbors(&self, cell: CellPosition) -> Vec<CellPosition> {
        let mut neighbors = Vec::with_capacity(8);

        for xdif in -1..2 {
            for ydif in -1..2 {
                let x: i32 = cell.x + xdif;
                let y: i32 = cell.y + ydif;

                if !(xdif == 0 && ydif == 0) {
                    let neighbor = CellPosition {
                        x: x,
                        y: y,
                    };
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }

    pub fn alive_neighbors(&self, cell: CellPosition) -> usize {
        self.neighbors(cell).iter()
            .map(|&neighbor| self.get_cell(neighbor))
            .filter(|&state| state == CellState::Alive)
            .count()
    }
}

// Rules:
// 1.   Any live cell with fewer than two live neighbors dies, as if caused by
//      under-population.
// 2.   Any live cell with two or three live neighbors lives on to the next
//      generation.
// 3.   Any live cell with more than three live neighbors dies, as if by
//      overcrowding.
// 4.   Any dead cell with exactly three live neighbors becomes a live cell, as
//      if by reproduction.
pub fn tick(world: &mut WorldState) {
    let cells = visit_cells(&world);
    let mut world_mut = world;
    set_cells(&mut world_mut, &cells);
}

fn visit_cells(world: &WorldState) -> Vec<(CellPosition, CellState)> {
    let mut cells_to_visit = HashSet::new();

    let alive_cells = world.alive_cells();

    for &cell in alive_cells {
        cells_to_visit.insert(cell);
        for neighbor in world.neighbors(cell) {
            cells_to_visit.insert(neighbor);
        }
    }

    let mut cells: Vec<(CellPosition, CellState)> = Vec::new();

    for cell in cells_to_visit {
        let cell_state = world.get_cell(cell);
        let alive_neighbors = world.alive_neighbors(cell);
        let new_cell_state = match cell_state {
            CellState::Alive if alive_neighbors < 2 => CellState::Dead,
            CellState::Alive if alive_neighbors > 3 => CellState::Dead,
            CellState::Alive => CellState::Alive,
            CellState::Dead if alive_neighbors == 3 => CellState::Alive,
            CellState::Dead => CellState::Dead
        };
        cells.push((cell, new_cell_state));
    }
    cells
}

fn set_cells(
    world: &mut WorldState,
    cells: &Vec<(CellPosition, CellState)>
) {
    for &item in cells.iter() {
        let (cell, state) = item;
        world.set_cell(cell, state);
    }
}

pub fn random_world(size: (i32, i32)) -> WorldState {
    let mut world_state = WorldState::new();
    let (size_x, size_y) = size;

    let mut rng = rand::thread_rng();

    let num_cells = (size_x * size_y);
    let num_samples = num_cells / 3;

    let samples: Vec<i32> = sample(&mut rng, 0..num_cells, num_samples as usize);

    for &pos in samples.iter() {
        let y: i32 = pos / size_x;
        let x: i32 = pos % size_x;

        info!(target: "game", "x={:?}, y={:?}", x, y);

        let pos = CellPosition {
            x: x,
            y: y
        };

        world_state.set_cell(pos, CellState::Alive)
    }

    world_state
}

