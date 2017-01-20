use std::default::Default;

use opengl_graphics::{ GlGraphics };
use piston::input::{ RenderArgs, UpdateArgs };
use piston::window::Size;

use world;
use world::{ WorldState, CellPosition, CellState, GridSize };

#[derive(Debug,Clone)]
pub struct GameSettings {
    pub block_size: usize,
    pub grid_size: GridSize,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            block_size: 10,
            grid_size: GridSize { y: 64, x: 48 },
        }
    }
}

impl GameSettings {
    pub fn window_height(&self) -> u32 {
        (self.block_size * self.grid_size.y) as u32
    }

    pub fn window_width(&self) -> u32 {
        (self.block_size * self.grid_size.x) as u32
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
    world_state: WorldState,
    settings: GameSettings,
}

impl Game {
    pub fn new(gl: GlGraphics, settings: GameSettings, world_state: WorldState) -> Game {
        Game {
            gl: gl,
            settings: settings,
            world_state: world_state,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let circle_radius = (self.settings.block_size - 2) as f64;
        let alive_cells = self.world_state.alive_cells();
        let block_size = self.settings.block_size;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        (&mut self.gl).draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(BLACK, gl);

            for cell in alive_cells {
                let cx: usize = cell.x * block_size;
                let cy: usize = cell.y * block_size;

                let circle = rectangle::square(
                    cx as f64,
                    cy as f64,
                    circle_radius);

                ellipse(WHITE, circle, c.transform, gl);
            }
        })
    }

	pub fn update(&mut self, args: &UpdateArgs) {
        self.world_state = world::tick(&self.world_state);
	}
}

