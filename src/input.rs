use sdl2::event::Event;

pub trait InputManager<M> {
    // TODO: Maybe it should take &mut self
    fn handle<F>(&self, event: &Event, push: &mut F) where F: FnMut(M);
}

pub type Mapper<M: 'static> = Box<Fn(&Event, &mut FnMut(M)) + 'static>;

/// A struct to map events to game messages.
pub struct BoxedInputMapper<M: 'static> {
    mappers: Vec<Mapper<M>>,
}

impl<M: 'static> BoxedInputMapper<M> {
    /// Creates a new input mapper
    pub fn new() -> Self {
        BoxedInputMapper { mappers: Vec::new() }
    }

    /// Adds a new event mapping function to the input mapper.
    pub fn add(&mut self, mapper: Mapper<M>) {
        self.mappers.push(mapper)
    }
}

impl<M: 'static> InputManager<M> for BoxedInputMapper<M> {
    /// Converts the given event to a set of messages.
    fn handle<F>(&self, event: &Event, push: &mut F)
        where F: FnMut(M)
    {
        for mapper in &self.mappers {
            mapper(event, push);
        }
    }
}
