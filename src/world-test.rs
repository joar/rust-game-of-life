#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;

use std::vec::Vec;

mod world;

use world::{ WorldState, CellPosition, random_world, GridSize };

fn main() {
    env_logger::init().unwrap();
    println!("Hello world!");

    let size = GridSize { x: 10, y: 10 };

    info!(target: "game", "Grid size: {:?}", size);
    let world = random_world(size);
    for cell in world.alive_cells() {
        info!(target: "game", "Cell: {:?}", cell);
    }
}
