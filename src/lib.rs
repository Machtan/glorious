#![feature(time2)]

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_mixer;

mod limiter;
mod sprite;
mod input;
mod game;
mod gameobject;
#[macro_use]
mod macros;

pub use limiter::FrameLimiter;
pub use sprite::Sprite;
pub use input::InputMapper;
pub use game::Game;
pub use gameobject::Behavior;
