use ggez::{
    ContextBuilder, GameResult,
    conf::{WindowMode, WindowSetup},
    event,
};
mod gameloop;

struct GameState {}
impl GameState {
    fn init() -> Self {
        GameState {}
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = {
        let mode = WindowMode::default().borderless(true).resizable(true);
        let setup = WindowSetup::default().title("title");

        ContextBuilder::new("class", "")
            .window_mode(mode)
            .window_setup(setup)
            .build()
    }?;
    let gamestate = GameState::init();

    event::run(ctx, event_loop, gamestate)
}
