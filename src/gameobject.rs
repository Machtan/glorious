use renderer::Renderer;

/// The behavior/logic part of an objects.
pub trait Behavior<S> {
    /// The messages used by the game.
    type Message;

    /// Initializes the object when it is added to the game.
    fn initialize(&mut self, _state: &mut S, _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Updates the object each frame.
    fn update(&mut self, _state: &mut S, _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Handles new messages since the last frame.
    fn handle(&mut self,
              _state: &mut S,
              _message: Self::Message,
              _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Renders the object.
    fn render(&mut self, _state: &S, _renderer: &mut Renderer) {
        // Do nothing by default
    }
}

impl<'a, S, B> Behavior<S> for [&'a mut B]
    where B: Behavior<S>,
          B::Message: Clone
{
    type Message = B::Message;

    fn initialize(&mut self, state: &mut S, queue: &mut Vec<Self::Message>) {
        for child in self {
            child.initialize(state, queue);
        }
    }

    fn update(&mut self, state: &mut S, queue: &mut Vec<Self::Message>) {
        for child in self {
            child.update(state, queue);
        }
    }

    fn handle(&mut self, state: &mut S, message: Self::Message, queue: &mut Vec<Self::Message>) {
        for child in self {
            child.handle(state, message.clone(), queue);
        }
    }

    fn render(&mut self, state: &S, renderer: &mut Renderer) {
        for child in self {
            child.render(state, renderer);
        }
    }
}
