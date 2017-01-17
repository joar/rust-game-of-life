extern crate piston;
extern crate graphics;
extern crate sdl2_window;

use sdl2_window::{ Sdl2Window };
use piston::input::{ RenderArgs };
use graphics::{ Context };

mod grid;
use grid::{Block, Grid, Location };

mod game;
use game::{ App };


fn main() {
    let mut app: App = App::new{
        grid_size: (64, 48),
        block_size: 10,
    };

    let mut window: Sdl2Window::new(
        &WindowSettings::new("game-of-life", app.canvas_size()))
        .fullscreen(false);

    let grid = grid::Grid::new(64, 64);
}
