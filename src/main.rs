use ggez::{
    ContextBuilder, GameResult,
    conf::{WindowMode, WindowSetup},
    event,
};
mod game;

fn main() -> GameResult {
    let (ctx, event_loop) = {
        let mode = WindowMode::default();
        let setup = WindowSetup::default().title("Title");

        ContextBuilder::new("game", "qweenkie")
            .window_mode(mode)
            .window_setup(setup)
            .build()
    }?;
    let gamestate = game::Game::new();

    event::run(ctx, event_loop, gamestate)
}
