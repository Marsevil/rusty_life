use crate::life_api;
use ggez::{event::EventHandler, GameResult};
use ggez::input::{mouse, keyboard};
use ggez::graphics::{Canvas, Rect, DrawParam, Quad, Color};

const TARGET_GRID_UPDATE_RATE : u32 = 4;
const GRID_UPDATE_TIMEOUT : f32 = 1.0 / TARGET_GRID_UPDATE_RATE as f32;
const TARGET_FPS : u32 = 30;

type WindowPosition = (f32, f32);
type GridPosition = (usize, usize);

pub struct GameState {
	is_paused: bool,
	grid_update_timeout: f32,
	pressing_click: Option<mouse::MouseButton>,
	grid: life_api::Board,
}

impl GameState {
	pub fn new() -> Self {
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
			grid_update_timeout: GRID_UPDATE_TIMEOUT,
			pressing_click: None,
			grid: life_api::Board::from(arr),
		}
	}

	fn get_grid_pos(&self, ctx: &ggez::Context, window_pos: WindowPosition) -> GridPosition {
		let screen_size = ctx.gfx.drawable_size();
		let grid_size = self.grid.get_array().dim();
		(
			(window_pos.0 / screen_size.0 * grid_size.1 as f32) as usize,
			(window_pos.1 / screen_size.1 * grid_size.0 as f32) as usize
		)
	}

	fn draw_pixel(
		&self,
		canvas: &mut Canvas,
		position: GridPosition,
		color: Color
	) {
		// Process cell size
		let cell_size;
		{
			let grid_size = self.grid.get_array().dim();
			let canvas_rect = canvas.scissor_rect();
			cell_size = (
				canvas_rect.w / grid_size.1 as f32,
				canvas_rect.h / grid_size.0 as f32
			);
		}

		// Draw a pixel depending on the screen size & the grid size.
		let rect = ggez::graphics::Rect::new(
			(position.0 as f32) * cell_size.0,
			(position.1 as f32) * cell_size.1,
			cell_size.0,
			cell_size.1);
		canvas.draw(
			&Quad,
			DrawParam::new()
				.dest_rect(rect)
				.color(color)
		)
	}

	pub fn draw_grid(&self, canvas: &mut ggez::graphics::Canvas) {
		// If the pixel is not null draw a white pixel
		self.grid.get_array().indexed_iter()
			.filter(|(_, x)| **x != 0)
			.for_each(|(pos, _)| self.draw_pixel(
				canvas,
				(pos.1, pos.0),
				ggez::graphics::Color::WHITE));
	}

	pub fn draw_hud(&self, canvas: &mut ggez::graphics::Canvas) {
		let canvas_rect = canvas.scissor_rect();
		// Draw pause basically two red rectangles
		if self.is_paused {
			let start_pos = (
				canvas_rect.w * 0.02,
				canvas_rect.h * 0.02
			);
			let rect_dim = (
				canvas_rect.w * 0.05,
				canvas_rect.h * 0.15
			);

			let rect1 = Rect::new(
				start_pos.0,
				start_pos.1,
				rect_dim.0,
				rect_dim.1
			);
			let rect2 = Rect::new(
				start_pos.0 + 2.0*rect_dim.0,
				start_pos.1,
				rect_dim.0,
				rect_dim.1
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

	pub fn create_pixel(&mut self, grid_pos: (usize, usize)) {
		self
		.grid
		.get_array_mut()[[grid_pos.1, grid_pos.0]] = 1;
	}

	pub fn delete_pixel(&mut self, grid_pos: (usize, usize)) {
		self
		.grid
		.get_array_mut()[[grid_pos.1, grid_pos.0]] = 0;
	}
}

impl EventHandler<ggez::GameError> for GameState {
	fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
		while ctx.time.check_update_time(TARGET_FPS) {
			let delta = 1.0 / TARGET_FPS as f32;
			self.grid_update_timeout -= delta;

			if self.grid_update_timeout < 0.0 && !self.is_paused {
				self.grid.update();
				self.grid_update_timeout = GRID_UPDATE_TIMEOUT;
			}
		}

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
			_x: f32,
			_y: f32,
		) -> Result<(), ggez::GameError> {
			let grid_pos = self.get_grid_pos(_ctx, (_x, _y));
		match button {
			mouse::MouseButton::Left => {
				self.pressing_click = Some(mouse::MouseButton::Left);
				self.create_pixel(grid_pos);
			},
			mouse::MouseButton::Right => {
				self.pressing_click = Some(mouse::MouseButton::Right);
				self.delete_pixel(grid_pos);
			},
			_ => {}
		}

		Ok(())
	}

	fn mouse_button_up_event(
			&mut self,
			_ctx: &mut ggez::Context,
			_button: ggez::event::MouseButton,
			_x: f32,
			_y: f32,
		) -> Result<(), ggez::GameError> {
		match &self.pressing_click {
			Some(key) => {
				if *key == _button {
					self.pressing_click = None;
				}
			},
			None => {}
		}

		Ok(())
	}

	fn mouse_motion_event(
			&mut self,
			_ctx: &mut ggez::Context,
			_x: f32,
			_y: f32,
			_dx: f32,
			_dy: f32,
		) -> Result<(), ggez::GameError> {
		match &self.pressing_click {
			Some(key) => {
				let grid_pos = self.get_grid_pos(_ctx, (_x, _y));
				match key {
					mouse::MouseButton::Left => self.create_pixel(grid_pos),
					mouse::MouseButton::Right => self.delete_pixel(grid_pos),
					_ => {}
				}
			},
			None => {}
		}

		Ok(())
	}
}