#[macro_use] extern crate log;
extern crate env_logger;

mod world;

use world::{ WorldState, CellPosition, random_world, GridSize };

fn main() {
    let size = GridSize { x: 10, y: 10 };
    info!("Grid size: {:?}", size);
    let world = random_world();
    info!("World: {:?}", world.alive_cells())
}
