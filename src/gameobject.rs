use sdl2::render::Renderer;

/// The behavior/logic part of an objects.
pub trait Behavior {
    /// The global game state.
    type State;
    /// The messages used by the game.
    type Message;

    /// Initializes the object when it is added to the game.
    fn initialize(&mut self,
                  _state: &mut Self::State,
                  _id: u64,
                  _new_messages: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Updates the object each frame.
    fn update(&mut self, _state: &mut Self::State, _id: u64, _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Handles new messages since the last frame.
    fn handle(&mut self,
              _state: &mut Self::State,
              _id: u64,
              _messages: &[Self::Message],
              _new_messages: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Renders the object.
    fn render(&self, _state: &Self::State, _renderer: &mut Renderer) {
        // Do nothing by default
    }
}

/// An object in the game.
pub struct GameObject<S, M> {
    pub id: u64,
    pub name: String,
    pub tags: Vec<&'static str>,
    pub behavior: Box<Behavior<State = S, Message = M>>,
}

impl<S, M> GameObject<S, M> {
    /// Creates a new game object.
    pub fn new(name: &str,
               id: u64,
               behavior: Box<Behavior<State = S, Message = M>>,
               tags: &[&'static str])
               -> GameObject<S, M> {
        GameObject {
            id: id,
            name: name.to_owned(),
            behavior: behavior,
            tags: tags.into(),
        }
    }
}
