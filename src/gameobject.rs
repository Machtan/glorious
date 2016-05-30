use sdl2::render::Renderer;

/// The behavior/logic part of an objects.
pub trait Behavior {
    /// The global game state.
    type State;
    /// The messages used by the game.
    type Message;

    /// Initializes the object when it is added to the game.
    fn initialize(&mut self, _state: &mut Self::State, _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Updates the object each frame.
    fn update(&mut self, _state: &mut Self::State, _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Handles new messages since the last frame.
    fn handle(&mut self,
              _state: &mut Self::State,
              _message: Self::Message,
              _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Renders the object.
    fn render(&self, _state: &Self::State, _renderer: &mut Renderer) {
        // Do nothing by default
    }
}

impl<'a, B> Behavior for [&'a mut B]
    where B: Behavior,
          B::Message: Clone
{
    type State = B::State;
    type Message = B::Message;

    fn initialize(&mut self, state: &mut Self::State, queue: &mut Vec<Self::Message>) {
        for child in self {
            child.initialize(state, queue);
        }
    }

    fn update(&mut self, state: &mut Self::State, queue: &mut Vec<Self::Message>) {
        for child in self {
            child.update(state, queue);
        }
    }

    fn handle(&mut self,
              state: &mut Self::State,
              message: Self::Message,
              queue: &mut Vec<Self::Message>) {
        for child in self {
            child.handle(state, message.clone(), queue);
        }
    }

    fn render(&self, state: &Self::State, renderer: &mut Renderer) {
        for child in self {
            child.render(state, renderer);
        }
    }
}
