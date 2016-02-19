//! Contains the sprite.

extern crate sdl2;

use std::rc::Rc;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

/// A renderable image with a settable source position.
#[derive(Clone)]
pub struct Sprite {
    texture: Rc<Texture>,
    source: Rect,
}
impl Sprite {
    /// Creates a new sprite using the given texture, and using the given
    /// rectangle as the source area on that texture.
    /// If no rectangle is given, the whole texture is used.
    pub fn new(texture: Rc<Texture>, source: Option<Rect>) -> Sprite {
        let s = if let Some(rect) = source {
            rect
        } else {
            let TextureQuery { width, height, ..} = texture.query();
            Rect::new(0, 0, width, height)
        };
        Sprite {
            texture: texture,
            source: s,
        }        
    }
    
    /// Renders this sprite at the given position using the renderer,
    /// potentially scaling the sprite to a new size.
    pub fn render(&self, renderer: &mut Renderer, x: i32, y: i32,
            size: Option<(u32, u32)>) {
        let destination = if let Some((width, height)) = size {
            Rect::new(x, y, width, height)
        } else {
            Rect::new(x, y, self.source.width(), self.source.height())
        };
        renderer.copy(&*self.texture, Some(self.source), Some(destination));
    }
}