extern crate piston;
#[macro_use] extern crate log;
extern crate env_logger;

extern crate opengl_graphics;
extern crate glutin_window;
extern crate graphics;

extern crate nalgebra;

extern crate rand;

use std::default::Default;

use nalgebra::Vector2;
use std::convert::Into;

use graphics::math::Vec2d;

use opengl_graphics::{ GlGraphics, OpenGL };
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use piston::event_loop::Events;
use piston::input::{ RenderEvent, UpdateEvent, RenderArgs, UpdateArgs };
use piston::window::Size;

mod world;
use world::{ CellPosition, CellState, GridSize };

#[derive(Debug)]
pub struct GameSettings {
    pub block_size: usize,
    pub grid_size: GridSize,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            block_size: 10,
            grid_size: [64, 48],
        }
    }
}

impl GameSettings {
    pub fn window_height(&self) -> u32 {
        (self.block_size * self.grid_size[1]) as u32
    }

    pub fn window_width(&self) -> u32 {
        (self.block_size * self.grid_size[0]) as u32
    }

    pub fn window_size(&self) -> Size {
        Size {
            width: self.window_width(),
            height: self.window_height()
        }
    }
}

pub struct Game {
    gl: GlGraphics,
    world_state: world::WorldState,
    settings: GameSettings,
}

impl Game {
    fn render(&self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for cell in self.ali {
                let circle = rectangle::square(
                    cell.loc.x as f64,
                    cell.loc.y as f64,
                    (self.settings.block_size - 2) as f64);

                ellipse(WHITE, circle, c.transform, gl);
            }
        })
    }

	fn update(&mut self, args: &UpdateArgs) {
        self.world_state = world::tick(self.world_state);
	}
}

fn main() {
    let opengl = OpenGL::V3_2;

    let settings: GameSettings = Default::default();

    let mut window: Window = WindowSettings::new(
            "Game of Life",
            settings.window_size()
        )
        .fullscreen(false)
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        world_state: random_world(settings.grid_size),
        settings: settings
    };

    let mut events = window.events();

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
