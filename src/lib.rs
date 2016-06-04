//! The crate `glorious` is a simple framework for message-based games,
//! based on SDL2.

#![feature(question_mark)]

extern crate ref_filter_map;
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

pub use game::Game;
pub use gameobject::Behavior;
pub use input::{InputManager, InputPattern, BoxedInputMapper};
pub use label::Label;
pub use limiter::FrameLimiter;
pub use renderer::{init_renderer, Device, Renderer};
pub use resources::ResourceManager;
pub use sprite::Sprite;
