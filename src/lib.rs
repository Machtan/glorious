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
mod resources;

pub use limiter::FrameLimiter;
pub use sprite::Sprite;
pub use label::Label;
pub use input::{InputManager, InputPattern, InputPatternKind, BoxedInputMapper};
pub use game::{Game, ExitSignal};
pub use gameobject::Behavior;
pub use resources::ResourceManager;
