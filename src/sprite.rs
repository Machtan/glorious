use std::rc::Rc;
use std::fmt::{self, Debug};
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture, TextureQuery};

/// A rectangular section of a texture.
#[derive(Clone)]
pub struct Sprite {
    texture: Rc<Texture>,
    pub rect: Rect,
}

impl Sprite {
    /// Creates a new sprite.
    ///
    /// The sprite corresponds to the section of `texture` specified by
    /// `rect`, or the entire texture, if `rect` is `None`.
    pub fn new(texture: Rc<Texture>, rect: Option<Rect>) -> Sprite {
        let rect = rect.unwrap_or_else(|| {
            let TextureQuery { width, height, ..} = texture.query();
            Rect::new(0, 0, width, height)
        });
        Sprite {
            texture: texture,
            rect: rect,
        }
    }

    /// Renders the sprite.
    ///
    /// If `size` is not `None`, the sprite will be scaled to that size.
    pub fn render(&self, renderer: &mut Renderer, x: i32, y: i32, size: Option<(u32, u32)>) {
        let (w, h) = size.unwrap_or_else(|| (self.rect.width(), self.rect.height()));
        let dest = Rect::new(x, y, w, h);
        renderer.copy(&*self.texture, Some(self.rect), Some(dest));
    }
}

impl Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Sprite {{")?;
        self.rect.fmt(f)?;
        write!(f, "}}")
    }
}
