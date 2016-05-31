use std::rc::Rc;

use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Font;

use renderer::Renderer;

enum State {
    Uncached((u8, u8, u8, u8)),
    Cached(Texture),
}

pub struct Label {
    text: String,
    font: Rc<Font>,
    size: (u32, u32),
    state: State,
}

impl Label {
    #[inline]
    pub fn new(font: Rc<Font>, text: String, color: (u8, u8, u8, u8)) -> Label {
        let size = font.size_of(&text).expect("could not calculate size of label");
        Label {
            text: text.into(),
            font: font,
            size: size,
            state: State::Uncached(color),
        }
    }

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
            let (sx, sy) = renderer.scale();
            // TODO: Figure out if it should actually be multiplied by the scale.
            let dst = Rect::new(x, y, (w as f32 / sx) as u32, (h as f32 / sy) as u32);

            renderer.copy(texture, None, Some(dst));
        } else {
            unreachable!();
        }
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    #[inline]
    pub fn texture(&self) -> Option<&Texture> {
        match self.state {
            State::Cached(ref texture) => Some(texture),
            _ => None,
        }
    }

    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn font(&self) -> &Rc<Font> {
        &self.font
    }
}
