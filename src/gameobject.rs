use sdl2::render::Renderer;

/// The behavior/logic part of an objects.
pub trait Behavior {
    /// The global game state.
    type State;
    /// The messages used by the game.
    type Message;

    /// Initializes the object when it is added to the game.
    fn initialize(&mut self, _state: &mut Self::State, _new_messages: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Updates the object each frame.
    fn update(&mut self, _state: &mut Self::State, _queue: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Handles new messages since the last frame.
    fn handle(&mut self,
              _state: &mut Self::State,
              _messages: &[Self::Message],
              _new_messages: &mut Vec<Self::Message>) {
        // Do nothing by default
    }

    /// Renders the object.
    fn render(&self, _state: &Self::State, _renderer: &mut Renderer) {
        // Do nothing by default
    }
}
