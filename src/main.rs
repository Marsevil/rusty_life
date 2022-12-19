use ggez::{event::EventHandler, GameResult};
use ggez::graphics::{Rect, DrawParam, Quad, Color};
use ggez::input::{mouse, keyboard};
mod life_api;

const TARGET_FPS : u32 = 4;
const SCREEN_SIZE: (f32, f32) = (800.0, 800.0);
const GRID_SIZE: life_api::Size = (10, 10);
const CELL_SIZE: (f32, f32) = (
	SCREEN_SIZE.0/(GRID_SIZE.0 as f32),
	SCREEN_SIZE.1/(GRID_SIZE.1 as f32));

type Position = (usize, usize);

fn draw_pixel(canvas: &mut ggez::graphics::Canvas, position: Position, color: ggez::graphics::Color) {
	// Draw a pixel depending on the screen size & the grid size.
	let rect = ggez::graphics::Rect::new(
		(position.0 as f32) * CELL_SIZE.0,
		(position.1 as f32) * CELL_SIZE.1,
		CELL_SIZE.0,
		CELL_SIZE.1);
	canvas.draw(
		&ggez::graphics::Quad,
		ggez::graphics::DrawParam::new()
			.dest_rect(rect)
			.color(color)
	)
}

struct GameState {
	is_paused: bool,
	grid: life_api::Board,
}

impl GameState {
	fn new() -> Self {
		let arr = ndarray::array![
			[1, 1, 0, 0, 0, 1, 1, 1, 0, 1],
			[1, 1, 0, 0, 1, 1, 0, 1, 1, 0],
			[0, 1, 1, 1, 0, 0, 0, 1, 0, 0],
			[1, 0, 0, 0, 0, 0, 1, 1, 0, 0],
			[0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
			[1, 1, 0, 0, 0, 1, 0, 0, 0, 1],
			[0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
			[1, 0, 1, 1, 0, 1, 1, 1, 0, 1],
			[1, 1, 0, 0, 1, 0, 1, 1, 0, 0],
			[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
		];
		Self {
			is_paused: true,
			grid: life_api::Board::from(arr),
		}
	}

	fn draw_grid(&self, canvas: &mut ggez::graphics::Canvas) {
		// If the pixel is not null draw a white pixel
		self.grid.get_array().indexed_iter()
			.filter(|(_, x)| **x != 0)
			.for_each(|(pos, _)| draw_pixel(
				canvas,
				(pos.1, pos.0),
				ggez::graphics::Color::WHITE));
	}

	fn draw_hud(&self, canvas: &mut ggez::graphics::Canvas) {
		// Draw pause basically two red rectangles
		if self.is_paused {
			const START_POS: (f32, f32) = (
				SCREEN_SIZE.0 as f32 * 0.02,
				SCREEN_SIZE.1 as f32 * 0.02
			);
			const RECT_DIM: (f32, f32) = (
				SCREEN_SIZE.0 as f32 * 0.05,
				SCREEN_SIZE.1 as f32 * 0.15
			);

			let rect1 = Rect::new(
				START_POS.0,
				START_POS.1,
				RECT_DIM.0,
				RECT_DIM.1
			);
			let rect2 = Rect::new(
				START_POS.0 + 2.0*RECT_DIM.0,
				START_POS.1,
				RECT_DIM.0,
				RECT_DIM.1
			);

			canvas.draw(
				&Quad,
				DrawParam::new()
					.dest_rect(rect1)
					.color(Color::RED));
			canvas.draw(
				&Quad,
				DrawParam::new()
					.dest_rect(rect2)
					.color(Color::RED));
		}
	}
}

impl EventHandler<ggez::GameError> for GameState {
	fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
		// Don't update if the process is too quick
		// Don't update if the game is paused
		if !ctx.time.check_update_time(TARGET_FPS) || self.is_paused {
			return Ok(());
		}

		// Update the grid
		self.grid.update();

		Ok(())
	}

	fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
		// Get clear canvas
		let mut canvas = ggez::graphics::Canvas::from_frame(
			ctx,
			ggez::graphics::Color::BLACK);

		// Draw grid and HUD
		self.draw_grid(&mut canvas);
		self.draw_hud(&mut canvas);

		canvas.finish(ctx)?;

		ggez::timer::yield_now();

		Ok(())
	}

	fn key_down_event(
			&mut self,
			ctx: &mut ggez::Context,
			input: ggez::input::keyboard::KeyInput,
			_repeated: bool,
		) -> Result<(), ggez::GameError> {
		if let Some(key) = input.keycode {
			match key {
				// Pause the game if space bar is pressed
				keyboard::KeyCode::Space => self.is_paused = !self.is_paused,
				// Quit the game if escape key is pressed
				keyboard::KeyCode::Escape => ctx.request_quit(),
				_ => {}
			}
		}

		Ok(())
	}

	fn mouse_button_down_event(
			&mut self,
			_ctx: &mut ggez::Context,
			button: ggez::event::MouseButton,
			x: f32,
			y: f32,
		) -> Result<(), ggez::GameError> {
			let grid_pos = (
				(x / SCREEN_SIZE.0 * GRID_SIZE.0 as f32) as usize,
				(y / SCREEN_SIZE.1 * GRID_SIZE.1 as f32) as usize
			);
		match button {
			mouse::MouseButton::Left => {
				let mut board = self.grid.get_array_mut();
				board[[grid_pos.1, grid_pos.0]] = 1;
			},
			mouse::MouseButton::Right => {
				let mut board = self.grid.get_array_mut();
				board[[grid_pos.1, grid_pos.0]] = 0;
			}
			_ => {}
		}

		Ok(())
	}
}

fn main() -> GameResult {
	// Initialize game context
	let (ctx, event_loop) = ggez::ContextBuilder::new("Rusty life", "Marsevil")
		.window_setup(ggez::conf::WindowSetup::default().title("Rust life"))
		.window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
		.build()?;

	// Initialize global game state
	let state = GameState::new();

	ggez::event::run(ctx, event_loop, state)
}
