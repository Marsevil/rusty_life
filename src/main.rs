use ggez::{event::EventHandler, GameResult, GameError};

use rusty_life::game_state::GameState;
mod life_api;

const DEFAULT_SCREEN_SIZE: (f32, f32) = (800.0, 800.0);

enum AppStates {
	InMenu,
	InGame(GameState),
}

impl EventHandler<GameError> for AppStates {
	fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.draw(_ctx),
			_ => Ok(())
		}
	}

	fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.update(_ctx),
			_ => Ok(())
		}
	}

	fn key_down_event(
			&mut self,
			ctx: &mut ggez::Context,
			input: ggez::input::keyboard::KeyInput,
			_repeated: bool,
		) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.key_down_event(
				ctx,
				input,
				_repeated),
			_ => Ok(())
		}
	}

	fn key_up_event(
		&mut self,
		_ctx: &mut ggez::Context,
		_input: ggez::input::keyboard::KeyInput
	) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.key_up_event(_ctx, _input),
			_ => Ok(())
		}
	}

	fn mouse_button_down_event(
			&mut self,
			_ctx: &mut ggez::Context,
			_button: ggez::event::MouseButton,
			_x: f32,
			_y: f32,
		) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.mouse_button_down_event(
				_ctx,
				_button,
				_x,
				_y),
			_ => Ok(())
		}
	}

	fn mouse_button_up_event(
			&mut self,
			_ctx: &mut ggez::Context,
			_button: ggez::event::MouseButton,
			_x: f32,
			_y: f32,
		) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.mouse_button_up_event(
				_ctx,
				_button,
				_x,
				_y),
			_ => Ok(())
		}
	}

	fn mouse_motion_event(
			&mut self,
			_ctx: &mut ggez::Context,
			_x: f32,
			_y: f32,
			_dx: f32,
			_dy: f32,
		) -> Result<(), GameError> {
		match self {
			Self::InGame(state) => state.mouse_motion_event(
				_ctx,
				_x,
				_y,
				_dx,
				_dy),
			_ => Ok(())
		}
	}
}

fn main() -> GameResult {
	// Initialize game context
	let (ctx, event_loop) = ggez::ContextBuilder::new("Rusty life", "Marsevil")
		.window_setup(ggez::conf::WindowSetup::default()
			.title("Rust life"))
		.window_mode(ggez::conf::WindowMode::default()
			.dimensions(
				DEFAULT_SCREEN_SIZE.0,
				DEFAULT_SCREEN_SIZE.1))
		.build()?;

	// Initialize global game state
	let state = AppStates::InGame(GameState::new());

	ggez::event::run(ctx, event_loop, state)
}
