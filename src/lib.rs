//! The crate `glorious` is a simple framework for message-based games,
//! based on SDL2.

#![feature(question_mark)]

extern crate ref_filter_map;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

#[macro_use]
mod macros;

mod color;
mod device;
mod game;
mod gameobject;
mod input;
mod label;
mod limiter;
mod sprite;
mod renderer;
mod resources;

pub use color::Color;
pub use device::Device;
pub use game::Game;
pub use gameobject::Behavior;
pub use input::{InputManager, InputPattern, BoxedInputMapper};
pub use label::Label;
pub use limiter::FrameLimiter;
pub use renderer::Renderer;
pub use resources::ResourceManager;
pub use sprite::Sprite;
