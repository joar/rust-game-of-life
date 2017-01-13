extern crate opengl_graphics;

use opengl_graphics::{ GlGraphics };

pub struct App {
    gl: GlGraphics,
    grid: grid::Grid,
    started: bool,
	block_size: usize,
    count: usize,
    mouse_loc: (f64, f64),
}

impl App {
	#[inline]
	pub fn new(grid_size: (usize, usize), block_size: usize) -> App {
        let grid = {
            let (width, height) = grid_size;

            Grid::new(height, width)
        };

		App {
			gl: GlGraphics::new(),
			grid: grid,
			started: false,
			block_size: block_size,
			count: 30,
			mouse_loc: (0.0, 0.0)
		}
	}

	fn render_grid(&mut self, win_ctx: &Context) {
		for block in self.grid.cells_iter() {

			win_ctx
				.rect((block.location.x * self.block_size) as f64,
					  (block.location.y * self.block_size) as f64,
					  self.block_size as f64,
					  self.block_size as f64)
				.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);
		}
	}

	#[cfg(random)]
	#[inline]
	fn update(&mut self) {
		use std::rand::random;

		let mut x = (random::<f64>() * WINDOW_WIDTH as f64) as uint;
		x = (x - x % BLOCK_SIZE) / BLOCK_SIZE;
		let mut y = (random::<f64>() * WINDOW_HEIGHT as f64) as uint;
		y = (y - y % BLOCK_SIZE) / BLOCK_SIZE;
		self.grid.insert(grid::Cell::new(Location::new(x, y)));
	}

	#[cfg(not(random))]
	#[inline]
	fn update(&mut self) {
		let mut remove = vec!();
		let mut add = vec!();

		for block in self.grid.cells_iter() {
			let live = self.grid.live_neighbors(block);
			let livelen = live.len();
			if livelen != 2 && livelen != 3 {
				remove.push(block.clone());
			}
			for &loc in self.grid.dead_neighbors(block).iter() {
				let cell = Cell::new(loc);
				if self.grid.live_neighbors(&cell).len() == 3 {
					add.push(cell);
				}
			}
		}
		for block in remove.iter() {
			self.grid.remove(block);
		}
		for block in add.move_iter() {
			self.grid.insert(block);
		}
	}

	fn render(&mut self, args: &RenderArgs) {
		(&mut self.gl).viewport(0, 0, args.width as i32, args.height as i32);
		let ref ctx = Context::abs(args.width as f64, args.height as f64);
		ctx.rgb(1.0, 1.0, 1.0).draw(&mut self.gl);

		self.render_grid(ctx);
	}

    fn canvas_size(&self) -> (usize, usize) {
        let (grid_height, grid_width) = self.grid.size();

        (grid_width * self.block_size,
         grid_height * self.block_size)
    }
}

