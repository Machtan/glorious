use std::rc::Rc;
use std::fmt::{self, Debug};

use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Font;

use renderer::Renderer;

enum State {
    Uncached((u8, u8, u8, u8)),
    Cached(Texture),
}

/// A text label with a font and a color.
pub struct Label {
    text: String,
    font: Rc<Font>,
    size: (u32, u32),
    state: State,
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
    pub fn new<'a, T: Into<String>>(font: Rc<Font>,
                   text: T,
                   color: (u8, u8, u8, u8),
                   renderer: Renderer<'a>)
                   -> Label {
        let text = text.into();
        let (tw, th) = font.size_of(&text).expect("could not calculate size of label");
        let (sx, sy) = renderer.scale();
        let size = ((tw as f32 / sx) as u32, (th as f32 / sy) as u32);

        Label {
            text: text,
            font: font,
            size: size,
            state: State::Uncached(color),
        }
    }

    /// Renders the font to the renderer.
    ///
    /// # Panics
    ///
    /// Panics if the label could not be rendered, either due to a
    /// rendering error, or if a texture was cached for a different
    /// renderer.
    pub fn render(&mut self, renderer: &mut Renderer, x: i32, y: i32) {
        if let State::Uncached((r, g, b, a)) = self.state {
            let surface = self.font
                .render(&self.text)
                .blended(Color::RGBA(r, g, b, a))
                .expect("could not render label");

            let texture = renderer.create_texture_from_surface(&surface)
                .expect("could not upload label to texture");

            self.state = State::Cached(texture);
        }

        if let State::Cached(ref texture) = self.state {
            let (w, h) = self.size;
            let dst = Rect::new(x, y, w, h);

            renderer.copy(texture, None, Some(dst));
        } else {
            unreachable!();
        }
    }

    /// Returns the size of the label in terms of the renderer.
    #[inline]
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Returns the cached texture, if any.
    #[inline]
    pub fn texture(&self) -> Option<&Texture> {
        match self.state {
            State::Cached(ref texture) => Some(texture),
            _ => None,
        }
    }

    /// Returns the text to be renderered.
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the font to render the label with.
    #[inline]
    pub fn font(&self) -> Rc<Font> {
        self.font.clone()
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Label")
            .field("text", &self.text)
            .field("size", &self.size)
            .field("font", &(..))
            .field("state", &(..))
            .finish()
    }
}
