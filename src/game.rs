extern crate sdl2;

use std::mem;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

use gameobject::{Behavior, GameObject};
use input::InputMapper;
use limiter::FrameLimiter;

/// A value with additional behavior for a game.
pub trait GameBehavior {
    // TODO: Maybe this should only supply `handle_exit`?

    /// The global game state.
    type State;
    /// The messages used by the game.
    type Message;

    fn update(&mut self,
              _state: &mut Self::State,
              _queue: &mut Vec<Self::Message>,
              _objects: &mut Vec<GameObject<Self::State, Self::Message>>) {
        // Do nothing by default
    }

    fn handle(&mut self,
              _state: &mut Self::State,
              _messages: &[Self::Message],
              _new_messages: &mut Vec<Self::Message>,
              _objects: &mut Vec<GameObject<Self::State, Self::Message>>) {
        // Do nothing by default
    }

    fn render(&self,
              _state: &Self::State,
              _renderer: &mut Renderer,
              _objects: &[GameObject<Self::State, Self::Message>]) {
        // Do nothing by default
    }

    /// This is called when the default quit actions are taken (the window is
    /// closed, or the user presses Escape).
    fn handle_exit(&self, source: &str) -> bool {
        // TODO: Maybe source should not be a string?
        match source {
            "window closed" => true,
            "escape pressed" => true,
            _ => false,
        }
    }
}

/// The state needed for a game.
pub struct Game<'a, S, M: 'static> {
    pub state: S,
    pub mapper: InputMapper<M>,

    next_id: u64,
    objects: Vec<GameObject<S, M>>,
    message_queue: Vec<M>,
    limiter: FrameLimiter,
    renderer: Renderer<'a>,
    event_pump: EventPump,
    clear_color: Color,
    behavior: Box<GameBehavior<State = S, Message = M>>,
}

impl<'a, S, M: 'static> Game<'a, S, M> {
    /// Creates a new game.
    pub fn new(state: S,
               fps: u32,
               behavior: Box<GameBehavior<State = S, Message = M>>,
               mapper: InputMapper<M>,
               renderer: Renderer<'a>,
               event_pump: EventPump)
               -> Self {

        Game {
            next_id: 1,
            objects: Vec::new(),
            message_queue: Vec::new(),
            limiter: FrameLimiter::new(fps),
            clear_color: Color::RGBA(255, 255, 255, 255),
            mapper: mapper,
            state: state,
            renderer: renderer,
            event_pump: event_pump,
            behavior: behavior,
        }
    }

    /// Adds a new object to the game.
    pub fn add(&mut self,
               behavior: Box<Behavior<State = S, Message = M>>,
               name: &str,
               tags: &[&'static str]) {
        let id = self.next_id;
        self.next_id += 1;
        let obj = GameObject::new(name, id, behavior, tags);
        self.objects.push(obj);
    }

    /// Starts running the game. Close the window to quit (by default).
    pub fn start(&mut self) {
        // Clear the screen
        self.renderer.set_draw_color(self.clear_color);
        self.renderer.clear();
        self.renderer.present();

        // Initialize the objects
        for object in &mut self.objects {
            object.behavior.initialize(&mut self.state, object.id, &mut self.message_queue);
        }

        let mut new_messages: Vec<M> = Vec::new();

        self.limiter.reset();
        'running: loop {
            // Convert events to messages
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        if self.behavior.handle_exit("window closed") {
                            break 'running;
                        }
                    }
                    e @ Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        if self.behavior.handle_exit("escape pressed") {
                            break 'running;
                        } else {
                            for message in self.mapper.convert(&e) {
                                self.message_queue.push(message);
                            }
                        }
                    }
                    other_event => {
                        for message in self.mapper.convert(&other_event) {
                            self.message_queue.push(message);
                        }
                    }
                }
            }

            // Let the objects handle messages
            for object in &mut self.objects {
                object.behavior.handle(&mut self.state,
                                       object.id,
                                       &self.message_queue,
                                       &mut new_messages);
            }

            // Let the game handle messages
            self.behavior.handle(&mut self.state,
                                 &self.message_queue,
                                 &mut new_messages,
                                 &mut self.objects);

            // Update (clear) the message queue
            mem::swap(&mut self.message_queue, &mut new_messages);
            new_messages.clear();

            // Update the objects and let them send messages
            for object in &mut self.objects {
                object.behavior.update(&mut self.state, object.id, &mut self.message_queue);
            }
            self.behavior.update(&mut self.state, &mut self.message_queue, &mut self.objects);

            // Clear the screen
            self.renderer.set_draw_color(self.clear_color);
            self.renderer.clear();

            // Render the objects
            for object in &self.objects {
                object.behavior.render(&self.state, &mut self.renderer);
            }
            self.behavior.render(&self.state, &mut self.renderer, &self.objects);

            // Present
            self.renderer.present();

            // Limit frame rate
            self.limiter.limit();
        }
    }
}
