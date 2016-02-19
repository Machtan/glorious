use sdl2::event::Event;

pub type Mapper<M: 'static> = Box<Fn(&Event, &mut Vec<M>) + 'static>;

/// A struct to map events to game messages.
pub struct InputMapper<M: 'static> {
    mappers: Vec<Mapper<M>>,
}

impl<M: 'static> InputMapper<M> {
    /// Creates a new input mapper
    pub fn new() -> Self {
        InputMapper { mappers: Vec::new() }
    }

    /// Adds a new event mapping function to the input mapper.
    pub fn add(&mut self, mapper: Mapper<M>) {
        self.mappers.push(mapper)
    }

    /// Converts the given event to a set of messages.
    pub fn convert(&self, event: &Event) -> Vec<M> {
        let mut messages = Vec::new();
        for mapper in &self.mappers {
            mapper(event, &mut messages);
        }
        messages
    }
}
