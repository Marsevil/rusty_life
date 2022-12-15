use ggez::{event::EventHandler, GameResult};
mod life_api;

const TARGET_FPS : u32 = 4;
const SCREEN_SIZE: (f32, f32) = (800.0, 800.0);
const GRID_SIZE: life_api::Size = (10, 10);
const GRID_STEP: (f32, f32) = (
    SCREEN_SIZE.0/(GRID_SIZE.0 as f32),
    SCREEN_SIZE.1/(GRID_SIZE.1 as f32));

type Position = (usize, usize);

fn draw_pixel(canvas: &mut ggez::graphics::Canvas, position: Position, color: ggez::graphics::Color) {
    let rect = ggez::graphics::Rect::new(
        (position.0 as f32) * GRID_STEP.0,
        (position.1 as f32) * GRID_STEP.1,
        GRID_STEP.0,
        GRID_STEP.1);
    canvas.draw(
        &ggez::graphics::Quad,
        ggez::graphics::DrawParam::new()
            .dest_rect(rect)
            .color(color)
    )
}

struct GameState {
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
            grid: life_api::Board::from(arr),
        }
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        if !ctx.time.check_update_time(TARGET_FPS) {
            return Ok(());
        }
        self.grid.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = ggez::graphics::Canvas::from_frame(
            ctx,
            ggez::graphics::Color::BLACK);

        self.grid.get_array().indexed_iter()
        .filter(|(_, x)| **x != 0)
        .for_each(
            |(pos, _)| 
                draw_pixel(
                    &mut canvas,
                    pos.clone(),
                    ggez::graphics::Color::WHITE));

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Rusty life", "Marsevil")
        .window_setup(ggez::conf::WindowSetup::default().title("Rust life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new();

    ggez::event::run(ctx, event_loop, state)
}
