use std::default::Default;

use opengl_graphics::{ GlGraphics };
use piston::input::{
    RenderArgs,
    UpdateArgs,
    PressEvent,
    Button,
    MouseButton,
    Key,
};
use piston::window::Size;

use world;
use world::{ WorldState, CellPosition, CellState };

enum DrawingMode {
    Kill,
    Spawn,
}

pub struct Game {
    gl: GlGraphics,
    world_state: WorldState,
    mouse_pos: (f64, f64),
    window_size: (u32, u32),
    block_size: f64,
    is_ticking: bool,
    is_drawing: bool,
    drawing_mode: DrawingMode,
}

impl Game {
    pub fn new(
        window_size: (u32, u32),
        gl: GlGraphics,
        world_state: WorldState,
    ) -> Game {
        Game {
            gl: gl,
            mouse_pos: (0.0, 0.0),
            window_size: window_size,
            block_size: 10.0,
            world_state: world_state,
            is_ticking: false,
            is_drawing: false,
            drawing_mode: DrawingMode::Spawn,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let ref mut gl = self.gl;
        let circle_radius = (self.block_size - 2.0) as f64;
        let alive_cells = self.world_state.alive_cells();
        let block_size = self.block_size;
        let is_ticking = self.is_ticking;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const DARK_GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            match is_ticking {
                true => clear(BLACK, gl),
                false => clear(DARK_GRAY, gl),
            };

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

    pub fn tick(&mut self, args: &UpdateArgs) {
        if self.is_ticking {
            self.world_state = world::tick(&self.world_state);
        }
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        info!("Update windown size: {:?}", (width, height));
        self.window_size = (width, height);
    }

    pub fn update_mouse_pos(&mut self, x: f64, y: f64) {
        info!("Mouse position: {:?}", (x, y));
        self.mouse_pos = (x, y);
        if self.is_drawing {
            self.handle_draw();
        }
    }

    pub fn handle_button_release(&mut self, button: Button) {
        info!("button released: {:?}", button);
        match button {
            Button::Mouse(MouseButton::Left) => { self.stop_drawing(); },
            Button::Mouse(MouseButton::Right) => { self.stop_drawing(); },
            _ => {},
        };
    }

    pub fn handle_button_press(&mut self, button: Button) {
        info!("button pressed: {:?}", button);
        match button {
            Button::Mouse(MouseButton::Left) => {
                self.start_drawing(DrawingMode::Spawn);
            },
            Button::Mouse(MouseButton::Right) => {
                self.start_drawing(DrawingMode::Kill);
            }
            Button::Keyboard(Key::Space) => { self.toggle_is_ticking(); },
            Button::Keyboard(Key::Backspace) => { self.kill_all_cells(); },
            _ => {},
        };
    }

    // Private

    fn handle_draw(&mut self) {
        match self.drawing_mode {
            DrawingMode::Kill => { self.kill_cell_at_mouse_location(); },
            DrawingMode::Spawn => { self.create_cell_at_mouse_location(); },
        };
    }

    fn start_drawing(&mut self, mode: DrawingMode) {
        self.is_drawing = true;
        self.drawing_mode = mode;

        self.handle_draw() // Trigger draw instantaneously
    }

    fn stop_drawing(&mut self) {
        self.is_drawing = false;
    }

    fn kill_all_cells(&mut self) {
        self.world_state = WorldState::new();
    }

    fn toggle_is_ticking(&mut self) {
        self.is_ticking = !self.is_ticking;
    }

    fn kill_cell_at_mouse_location(&mut self) {
        let cell_position = self.cell_position_for_mouse_position(self.mouse_pos);
        self.world_state.set_cell(
            cell_position,
            CellState::Dead,
        );
    }

    fn create_cell_at_mouse_location(&mut self) {
        let cell_position = self.cell_position_for_mouse_position(self.mouse_pos);
        self.world_state.set_cell(
            cell_position,
            CellState::Alive,
        );
    }

    fn cell_position_for_mouse_position(&self, mouse_position: (f64, f64)) -> CellPosition {
        let (mut mouse_x, mut mouse_y) = mouse_position;
        mouse_x /= self.block_size;
        mouse_y /= self.block_size;

        CellPosition {
            x: mouse_x.round() as i32,
            y: mouse_y.round() as i32,
        }
    }
}

