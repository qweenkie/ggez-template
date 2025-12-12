use crate::Game;
use ggez::{
    Context, GameError, event,
    graphics::{Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
};

impl event::EventHandler for Game {
    // Game Loop
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let _delta = ctx.time.delta(); //

        Ok(())
    }

    // Rendering logic
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let canvas = Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }

    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */
    /* EVERYTHING BELOW THIS IS NOT A REQUIRED FUNCTION */
    /*    AND CAN BE SAFELY COMMENTED OUT OR DELETED    */
    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        if let Some(key) = input.keycode {
            match key {
                KeyCode::Escape => ctx.request_quit(),
                _ => {}
            }
        }

        Ok(())
    }
    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput) -> Result<(), GameError> {
        Ok(())
    }

    // A Unicode character was received, usually from keyboard input.
    // This is the intended way of facilitating text input.
    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), GameError> {
        Ok(())
    }

    // The mouse was moved; it provides both absolute x and y coordinates in the window,
    // and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }

    // The mouse wheel was scrolled, vertically (y, positive away from and negative toward the user)
    // or horizontally (x, positive to the right and negative to the left).
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), GameError> {
        Ok(())
    }

    fn mouse_enter_or_leave(
        &mut self,
        _ctx: &mut Context,
        _entered: bool,
    ) -> Result<(), GameError> {
        Ok(())
    }

    // This function is commented out by default, because the default implementation just
    // translates touch input into mouse input, and I cannot replicate that.
    // Only uncomment this if you know what you are doing, because I sure as hell don't.
    //
    // fn touch_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     phase: winit_event::TouchPhase,
    //     x: f64,
    //     y: f64,
    // ) -> Result<(), GameError> {
    //     Ok(())
    // }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        _width: f32,
        _height: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }

    // If this returns true, the error was fatal, so the event loop ends, aborting the game.
    fn on_error(&mut self, _ctx: &mut Context, _origin: event::ErrorOrigin, _e: GameError) -> bool {
        true
    }

    // Called upon a quit event. If it returns true, the game does not exit.
    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        Ok(false)
    }
}
