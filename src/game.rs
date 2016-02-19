extern crate sdl2;

use std::mem;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

use gameobject::Behavior;
use input::InputMapper;
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
    pub fn new(fps: u32,
               renderer: Renderer<'a>,
               event_pump: EventPump)
               -> Self {

        Game {
            limiter: FrameLimiter::new(fps),
            clear_color: Color::RGBA(255, 255, 255, 255),
            renderer: renderer,
            event_pump: event_pump,
        }
    }

    /// Starts running the game. Close the window to quit (by default).
    pub fn start<S, M, F>(&mut self, 
                          mut state: S,
                          mapper: InputMapper<M>, 
                          mut objects: Vec<&mut Behavior<State=S, Message=M>>,
                          mut on_exit: F)
                          where M: 'static, 
                                F: FnMut(ExitReason) -> bool {
        // Initialize object state
        let mut message_queue: Vec<M> = Vec::new();
        let mut new_messages: Vec<M> = Vec::new();
        
        // Clear the screen
        self.renderer.set_draw_color(self.clear_color);
        self.renderer.clear();
        self.renderer.present();

        // Initialize the objects
        for object in &mut objects {
            object.initialize(&mut state, &mut message_queue);
        }

        

        self.limiter.reset();
        'running: loop {
            // Convert events to messages
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        if on_exit(ExitReason::ApplicationQuit) {
                            break 'running;
                        }
                    }
                    e @ Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        if on_exit(ExitReason::EscapePressed) {
                            break 'running;
                        } else {
                            for message in mapper.convert(&e) {
                                message_queue.push(message);
                            }
                        }
                    }
                    other_event => {
                        for message in mapper.convert(&other_event) {
                            message_queue.push(message);
                        }
                    }
                }
            }

            // Let the objects handle messages
            for object in &mut objects {
                object.handle(&mut state,
                                       &message_queue,
                                       &mut new_messages);
            }

            // Update (clear) the message queue
            mem::swap(&mut message_queue, &mut new_messages);
            new_messages.clear();

            // Update the objects and let them send messages
            for object in &mut objects {
                object.update(&mut state, &mut message_queue);
            }

            // Clear the screen
            self.renderer.set_draw_color(self.clear_color);
            self.renderer.clear();

            // Render the objects
            for object in &objects {
                object.render(&state, &mut self.renderer);
            }

            // Present
            self.renderer.present();

            // Limit frame rate
            self.limiter.limit();
        }
    }
}
