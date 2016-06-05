use std::fmt::{self, Debug};

use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2_ttf::Font;

use color::Color;
use device::Device;
use renderer::Renderer;

/// A text label.
pub struct Label {
    texture: Texture,
    size: (u32, u32),
}

impl Label {
    /// Creates a new label.
    ///
    /// If high-dpi mode is enable for the renderer, then the font is
    /// assumed to be upscaled accordingly. This is automatically
    /// handled when the font is loaded with
    /// [`ResourceManager::font`](struct.ResourceManager.html#method.font).
    ///
    /// # Panics
    ///
    /// Panics if `text` cannot be rendered by `font`, e.g. if the
    /// the font does not contain the needed glyphs.
    #[inline]
    pub fn new(font: &Font, text: &str, color: Color, device: &Device) -> Label {
        let surface = font.render(text)
            .blended(color)
            .expect("could not render label");

        let (tw, th) = surface.size();
        let (sx, sy) = device.scale();
        let size = ((tw as f32 / sx) as u32, (th as f32 / sy) as u32);

        let texture = device.create_texture_from_surface(&surface)
            .expect("could not upload label to texture");

        Label {
            texture: texture,
            size: size,
        }
    }

    /// Renders the font to the renderer.
    ///
    /// # Panics
    ///
    /// Panics if `Renderer::copy` would panic, given the texture of the
    /// label.
    pub fn render(&self, renderer: &mut Renderer, x: i32, y: i32) {
        let (w, h) = self.size;
        let dst = Rect::new(x, y, w, h);
        renderer.copy(&self.texture, None, Some(dst));
    }

    /// Returns the size of the label in terms of the renderer.
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Returns the cached texture, if any.
    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    #[inline]
    pub fn into_texture(self) -> Texture {
        self.texture
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Label")
            .field("size", &self.size)
            .field("texture", &(..))
            .finish()
    }
}
