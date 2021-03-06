use std::rc::Rc;
use std::fmt::{self, Debug};
use sdl2::render::{Texture, TextureQuery};

use renderer::Renderer;
use rect::Rect;

/// A rectangular section of a texture rendered on its own.
#[derive(Clone)]
pub struct Sprite {
    /// The area in the texture for this sprite.
    pub rect: Rect,
    texture: Rc<Texture>,
}

impl Sprite {
    /// Creates a new sprite.
    ///
    /// The sprite corresponds to the section of `texture` specified by
    /// `rect`, or the entire texture, if `rect` is `None`.
    pub fn new(texture: Rc<Texture>, rect: Option<Rect>) -> Sprite {
        let rect = rect.unwrap_or_else(|| {
            let TextureQuery { width, height, .. } = texture.query();
            Rect::new(0, 0, width, height)
        });
        Sprite {
            texture: texture,
            rect: rect,
        }
    }

    /// Renders the sprite to the renderer at a given point.
    ///
    /// If `size` is `Some`, the sprite will be scaled to that size.
    ///
    /// # Panics
    ///
    /// Panics if drawing fails for any reason (e.g. driver failure), or
    /// if the provided texture does not belong to the renderer.
    pub fn render(&self, renderer: &mut Renderer, x: i32, y: i32, size: Option<(u32, u32)>) {
        let (w, h) = size.unwrap_or_else(|| (self.rect.width, self.rect.height));
        let dst = Rect::new(x, y, w, h);
        renderer.copy(&*self.texture, Some(self.rect), Some(dst));
    }

    pub fn render_rect<R: Into<Rect>>(&self, renderer: &mut Renderer, rect: R) {
        renderer.copy(&*self.texture, Some(self.rect), Some(rect.into()));
    }
}

impl Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Sprite")
            .field("rect", &self.rect)
            .field("texture", &(..))
            .finish()
    }
}
