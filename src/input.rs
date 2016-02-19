extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// A struct to map events to game messages.
pub struct InputMapper<M: 'static> {
    mappers: Vec<Box<Fn(&Event, &mut Vec<M>) + 'static>>,
}
impl< M: 'static> InputMapper<M> {
    /// Creates a new input mapper
    pub fn new() -> Self {
        InputMapper {
            mappers: Vec::new(),
        }
    }
    
    /// Adds a new event mapping function to the input mapper.
    pub fn add<F>(&mut self, mapper: Box<F>) 
            where F: Fn(&Event, &mut Vec<M>) + 'static {
        self.mappers.push(mapper)
    }
    
    /// Converts the given event to a set of messages.
    pub fn convert(&self, event: &Event) -> Vec<M> {
        let mut messages = Vec::new();
        for mapper in self.mappers.iter() {
            mapper(event, &mut messages);
        }
        messages
    }
}