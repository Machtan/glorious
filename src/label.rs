
use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Font;
use resources::ResourceManager;

pub struct Label {
    text: String,
    font: String,
    color: (u8, u8, u8, u8),
    size: (u32, u32),
    texture: Option<Texture>,
}

impl Label {
    pub fn new(font_id: &str, font: &Font, text: &str, color: (u8, u8, u8, u8)) -> Label {
        let size = font.size_of(text).expect("Could not get size of label");
        Label {
            text: String::from(text),
            font: String::from(font_id),
            color: color,
            size: size,
            texture: None,
        }
    }

    pub fn render(&mut self,
                  renderer: &mut Renderer,
                  x: i32,
                  y: i32,
                  resources: &ResourceManager) {

        let (w, h) = self.size;
        let dest = Rect::new(x, y, w, h);
        if let Some(ref texture) = self.texture {
            renderer.copy(texture, None, Some(dest));
        } else {
            let font = resources.font(&self.font).expect("font not found");
            let (r, g, b, a) = self.color;
            let surface = font.render(&self.text)
                .blended(Color::RGBA(r, g, b, a))
                .expect("Could not render label");
            let texture = renderer.create_texture_from_surface(&surface)
                .expect("Could not upload label to texture");
            renderer.copy(&texture, None, Some(dest));
            self.texture = Some(texture);
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn texture(&self) -> Option<&Texture> {
        self.texture.as_ref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font(&self) -> &str {
        &self.font
    }
}
