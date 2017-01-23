#[macro_use] extern crate log;
extern crate env_logger;
extern crate rand;

use std::vec::Vec;

mod world;

use world::{ WorldState, CellPosition, random_world };

fn main() {
    env_logger::init().unwrap();
    println!("Hello world!");

    let size = ViewSize { x: 10, y: 10 };

    info!("Grid size: {:?}", size);
    let world = random_world(size);
    for cell in world.alive_cells() {
        info!("Cell: {:?}", cell);
    }
}
