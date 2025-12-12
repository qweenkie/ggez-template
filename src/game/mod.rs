use ggez::event::EventHandler;

mod gameloop;
mod input;
mod render;

pub struct Game {}
impl Game {
    pub fn new() -> Self {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.impl_update(ctx)
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.impl_draw(ctx)
    }
    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> Result<(), ggez::GameError> {
        self.impl_key_down_event(ctx, input, repeated)
    }
}
