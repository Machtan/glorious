extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::pixels::Color;
use limiter::FrameLimiter;
use sdl2::keyboard::Keycode;
use input::InputMapper;
use gameobject::{Behavior, GameObject};

/// An object with the behavior needed for the game.
pub trait GameBehavior{
    type State;
    type Message;
    
    fn update(&mut self, state: &mut Self::State,
        queue: &mut Vec<Self::Message>,
        objects: &mut Vec<GameObject<Self::State, Self::Message>>) {}
    
    fn handle(&mut self, state: &mut Self::State, messages: &Vec<Self::Message>, 
        new_messages: &mut Vec<Self::Message>, 
        objects: &mut Vec<GameObject<Self::State, Self::Message>>) {}
    
    fn render(&self, state: &Self::State, renderer: &mut Renderer, 
        objects: &Vec<GameObject<Self::State, Self::Message>>) {}
    
    /// This is called when the default quit actions are taken (the window is 
    /// closed, or the user presses Escape).
    fn exit_handler(&self, source: &str) -> bool {
        match source {
            "window closed" => true,
            "escape pressed" => true,
            other => false,
        }
    }
}

/// The main game objects.
pub struct Game<'a, S, M: 'static> {
    next_id: u64,
    pub state: S,
    objects: Vec<GameObject<S, M>>,
    message_queue: Vec<M>,
    limiter: FrameLimiter,
    renderer: Renderer<'a>,
    event_pump: EventPump,
    clear_color: Color,
    pub mapper: InputMapper<M>,
    behavior: Box<GameBehavior<State=S, Message=M>>,
}
impl <'a, S, M: 'static> Game<'a, S, M> {
    /// Creates a new game.
    pub fn new(state: S, fps: u32, 
        behavior: Box<GameBehavior<State=S, Message=M>>, 
        mapper: InputMapper<M>, renderer: Renderer<'a>, event_pump: EventPump) 
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
    pub fn add(&mut self, behavior: Box<Behavior<State=S, Message=M>>, 
            name: &str, tags: &[&'static str]) {
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
        for object in self.objects.iter_mut() {
            object.behavior.initialize(
                &mut self.state, object.id, &mut self.message_queue
            );
        }
        
        self.limiter.reset();
        'running: loop {
            // Convert events to messages
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        if self.behavior.exit_handler("window closed") {
                            break 'running
                        }
                    },
                    e @ Event::KeyDown { keycode: Some(Keycode::Escape), .. } 
                    => {
                        if self.behavior.exit_handler("escape pressed") {
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
            
            let mut new_messages: Vec<M> = Vec::new();
            
            // Let the objects handle messages
            for object in self.objects.iter_mut() {
                object.behavior.handle(
                    &mut self.state, object.id, &self.message_queue,
                    &mut new_messages
                );
            }
            // Let the game handle messages
            self.behavior.handle(
                &mut self.state,
                &self.message_queue, &mut new_messages, &mut self.objects
            );
            
            // Update (clear) the message queue
            self.message_queue = new_messages;
            
            // Update the objects and let them send messages
            for object in self.objects.iter_mut() {
                object.behavior.update(
                    &mut self.state, object.id, &mut self.message_queue
                );
            }
            // Update the game
            self.behavior.update(&mut self.state, &mut self.message_queue,
                &mut self.objects);
            
            // Clear the screen
            self.renderer.set_draw_color(self.clear_color);
            self.renderer.clear();
            
            // Render the objects
            for object in self.objects.iter() {
                object.behavior.render(&self.state, &mut self.renderer);
            }
            // Render the game (or let it render something too ;)
            self.behavior.render(&self.state, &mut self.renderer, &self.objects);
            
            // Present
            self.renderer.present();
            
            // Limit frame rate
            self.limiter.limit();
        }
    }
}