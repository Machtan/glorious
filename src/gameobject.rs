extern crate sdl2;

use sdl2::render::Renderer;

/// The behavior/logic part of an objects.
pub trait Behavior {
    /// The global game state.
    type State;
    
    /// THe messages used by the game.
    type Message;
    
    /// Initializes the object when it is added to the game.
    fn initialize(&mut self, state: &mut Self::State, id: u64,
        new_messages: &mut Vec<Self::Message>) {}
    
    /// Updates the object each frame.
    fn update(&mut self, state: &mut Self::State, id: u64,
        queue: &mut Vec<Self::Message>) {}
    
    /// Notifies the object when an event is called.
    fn handle(&mut self, state: &mut Self::State, id: u64,
        messages: &Vec<Self::Message>, 
        new_messages: &mut Vec<Self::Message>) {}
    
    /// Renders the object.
    fn render(&self, state: &Self::State, renderer: &mut Renderer) {}
}

/// An object in the game.
pub struct GameObject<S, M> {
    pub id: u64,
    pub name: String,
    pub tags: Vec<&'static str>,
    pub behavior: Box<Behavior<State=S, Message=M>>
}

impl<S, M> GameObject<S, M> {
    /// Creates a new game object.
    pub fn new(name: &str, id: u64, behavior: Box<Behavior<State=S, Message=M>>,
        tags: &[&'static str])
            -> GameObject<S, M> {
        let mut own_tags = Vec::new();
        own_tags.extend_from_slice(tags);
        GameObject {
            id: id,
            name: name.to_owned(),
            behavior: behavior,
            tags: own_tags,
        }
    }
}