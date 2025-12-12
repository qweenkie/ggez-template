use crate::*;
use ggez::event::EventHandler;

impl game::Game {
    pub fn impl_update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let delta = ctx.time.delta();

        Ok(())
    }
}
