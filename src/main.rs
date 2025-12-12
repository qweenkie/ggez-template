use ggez::{
    ContextBuilder, GameResult,
    conf::{WindowMode, WindowSetup},
    event,
};
mod gameloop;

// This is the global game state
pub struct Game {}
impl Game {
    pub fn init() -> Self {
        Game {}
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = {
        let mode = WindowMode::default().resizable(true);
        let setup = WindowSetup::default().title("Title");

        ContextBuilder::new("name", "author")
            .window_mode(mode)
            .window_setup(setup)
            .build()
    }?;
    let gamestate = Game::init();

    event::run(ctx, event_loop, gamestate)
}
