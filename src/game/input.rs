use crate::*;
use ggez::{
    Context,
    event::EventHandler,
    input::keyboard::{KeyCode, KeyInput},
};

impl game::Game {
    pub fn impl_key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(key) = input.keycode {
            match key {
                KeyCode::Escape => ctx.request_quit(),
                _ => {}
            }
        }

        Ok(())
    }
}
