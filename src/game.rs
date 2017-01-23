use std::default::Default;

use opengl_graphics::{ GlGraphics };
use piston::input::{ RenderArgs, UpdateArgs, PressEvent, Button, MouseButton };
use piston::window::Size;

use world;
use world::{ WorldState, CellPosition, CellState, ViewSize };

#[derive(Debug,Clone)]
pub struct GameSettings {
    pub block_size: f64,
    pub view_size: ViewSize,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            block_size: 10.0,
            view_size: ViewSize { y: 64, x: 48 },
        }
    }
}

impl GameSettings {
    pub fn window_height(&self) -> u32 {
        (self.block_size as u32 * self.view_size.y as u32)
    }

    pub fn window_width(&self) -> u32 {
        (self.block_size as u32 * self.view_size.x as u32)
    }

    pub fn window_size(&self) -> Size {
        Size {
            height: self.window_height(),
            width: self.window_width(),
        }
    }
}

pub struct Game {
    gl: GlGraphics,
    world_state: WorldState,
    mouse_pos: (f64, f64),
    settings: GameSettings,
}

impl Game {
    pub fn new(
        gl: GlGraphics,
        settings: GameSettings,
        world_state: WorldState
    ) -> Game {
        Game {
            gl: gl,
            settings: settings,
            mouse_pos: (0.0, 0.0),
            world_state: world_state,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let circle_radius = (self.settings.block_size - 2.0) as f64;
        let alive_cells = self.world_state.alive_cells();
        let block_size = self.settings.block_size;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        (&mut self.gl).draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(BLACK, gl);

            for &cell in alive_cells {
                let cx: i32 = cell.x * block_size as i32;
                let cy: i32 = cell.y * block_size as i32;

                let circle = rectangle::square(
                    cx as f64,
                    cy as f64,
                    circle_radius);

                ellipse(WHITE, circle, c.transform, gl);
            }
        })
    }

    pub fn on_mouse_press(&mut self, button: &MouseButton) {
        info!("Pressed mouse: {:?}", button);
    }

	pub fn update(&mut self, args: &UpdateArgs) {
        self.world_state = world::tick(&self.world_state);
	}
}

