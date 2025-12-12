use crate::*;
use ggez::graphics::{Canvas, Color};

impl game::Game {
    pub fn impl_draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }
}
