#![feature(question_mark)]
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

mod limiter;
mod sprite;
mod label;
mod input;
mod game;
mod gameobject;
#[macro_use]
mod macros;
mod renderer;
mod resources;

pub use game::{Game, ExitSignal};
pub use gameobject::Behavior;
pub use input::{InputManager, InputPattern, InputPatternKind, BoxedInputMapper};
pub use label::Label;
pub use limiter::FrameLimiter;
pub use renderer::Renderer;
pub use resources::ResourceManager;
pub use sprite::Sprite;
