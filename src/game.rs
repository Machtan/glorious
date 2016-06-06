use std::mem;

use sdl2::EventPump;

use color::Color;
use gameobject::Behavior;
use input::InputManager;
use limiter::FrameLimiter;
use renderer::Renderer;

/// The state needed for a game.
pub struct Game<'a, 'r: 'a> {
    limiter: FrameLimiter,
    renderer: Renderer<'a, 'r>,
    event_pump: EventPump,
    clear_color: Color,
}

impl<'a, 'r> Game<'a, 'r> {
    /// Creates a new game.
    pub fn new(fps: u32, renderer: Renderer<'a, 'r>, event_pump: EventPump) -> Self {
        Game::with_clear_color(Color(0xff, 0xff, 0xff, 0xff), fps, renderer, event_pump)
    }

    pub fn with_clear_color(color: Color,
                            fps: u32,
                            renderer: Renderer<'a, 'r>,
                            event_pump: EventPump)
                            -> Self {
        Game {
            limiter: FrameLimiter::new(fps),
            clear_color: color,
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    /// Runs the game.
    ///
    /// For each message pushed to the queue, `is_quit_message` is
    /// called to determine if the game should quit.
    pub fn run<B, S, I, F>(&mut self,
                           state: &mut S,
                           manager: &I,
                           behavior: &mut B,
                           is_quit_message: F)
        where B: Behavior<S>,
              I: InputManager<B::Message>,
              F: Fn(&B::Message) -> bool
    {
        // Create message queues
        let mut front = Vec::new();
        let mut back = Vec::new();

        // Clear the screen
        self.renderer.set_draw_color(self.clear_color);
        self.renderer.clear();
        self.renderer.present();

        // Initialize
        behavior.initialize(state, &mut front);
        self.limiter.reset();

        // Main loop
        'running: loop {
            // Handle events
            for event in self.event_pump.poll_iter() {
                manager.handle(&event, &mut |m| front.push(m));
            }

            // Let the objects handle messages
            for m in front.drain(..) {
                if is_quit_message(&m) {
                    break 'running;
                }
                behavior.handle(state, m, &mut back);
            }

            // Swap the message queues
            mem::swap(&mut front, &mut back);

            // Update the objects and let them send messages
            behavior.update(state, &mut front);

            // Clear the screen
            self.renderer.set_draw_color(self.clear_color);
            self.renderer.clear();

            // Render
            behavior.render(state, &mut self.renderer);
            self.renderer.present();

            // Limit frame rate
            self.limiter.limit();
        }
    }
}
