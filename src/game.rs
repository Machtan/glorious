extern crate sdl2;

use std::mem;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

use gameobject::Behavior;
use input::InputManager;
use limiter::FrameLimiter;

pub enum ExitReason {
    ApplicationQuit,
    EscapePressed,
}

/// The state needed for a game.
pub struct Game<'a> {
    limiter: FrameLimiter,
    renderer: Renderer<'a>,
    event_pump: EventPump,
    clear_color: Color,
}

impl<'a> Game<'a> {
    /// Creates a new game.
    pub fn new(fps: u32, renderer: Renderer<'a>, event_pump: EventPump) -> Self {
        Game {
            limiter: FrameLimiter::new(fps),
            clear_color: Color::RGBA(255, 255, 255, 255),
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    /// Runs the game. Close the window to quit (by default).
    pub fn run<B, I, F>(&mut self,
                        mut state: B::State,
                        manager: &I,
                        mut behavior: &mut B,
                        mut on_exit: F)
        where B: Behavior,
              I: InputManager<B::Message>,
              F: FnMut(ExitReason) -> bool
    {
        // Create message queues
        let mut message_queue = Vec::new();
        let mut new_messages = Vec::new();

        // Clear the screen
        self.renderer.set_draw_color(self.clear_color);
        self.renderer.clear();
        self.renderer.present();

        // Initialize
        behavior.initialize(&mut state, &mut message_queue);
        self.limiter.reset();

        // Main loop
        'running: loop {
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        if on_exit(ExitReason::ApplicationQuit) {
                            break 'running;
                        }
                    }
                    e => {
                        if let Event::KeyDown { keycode: Some(Keycode::Escape), .. } = e {
                            if on_exit(ExitReason::EscapePressed) {
                                break 'running;
                            }
                        }
                        // Let the input manager push to the message queue
                        manager.handle(&e, &mut |m| message_queue.push(m));
                    }
                }
            }

            // Let the objects handle messages
            behavior.handle(&mut state, &message_queue, &mut new_messages);

            // Update (clear) the message queue
            mem::swap(&mut message_queue, &mut new_messages);
            new_messages.clear();

            // Update the objects and let them send messages
            behavior.update(&mut state, &mut message_queue);

            // Clear the screen
            self.renderer.set_draw_color(self.clear_color);
            self.renderer.clear();

            // Render
            behavior.render(&state, &mut self.renderer);
            self.renderer.present();

            // Limit frame rate
            self.limiter.limit();
        }
    }
}
