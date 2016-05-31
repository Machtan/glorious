
use std::rc::Rc;
use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Font;
use resources::ResourceManager;

enum LabelKind {
    Cached { texture: Rc<Texture> },
    Uncached { font: String, text: String, color: (u8, u8, u8, u8) },
}

pub struct Label {
    size: (u32, u32),
    kind: LabelKind,
}

impl Label {
    pub fn new(font_id: &str, font: &Font, text: &str, color: (u8, u8, u8, u8))
            -> Label {
        let size = font.size_of(text).expect("Could not get size of label");
        Label {
            size: size,
            kind: LabelKind::Uncached {
                font: String::from(font_id),
                text: String::from(text),
                color: color,
            }
        }
    }
    
    pub fn render(&mut self, renderer: &mut Renderer, x: i32, y: i32,
            resources: &ResourceManager) {
        use self::LabelKind::*;
        
        let mut just_cached = false;
        let texture = match self.kind {
            Uncached { ref font, ref text, color: (r, g, b, a) } => {
                let font = resources.font(font).expect("font not found");
                let surface = font.render(text).blended(Color::RGBA(r, g, b, a))
                    .expect("Could not render label");
                let texture = renderer.create_texture_from_surface(&surface)
                    .expect("Could not upload label to texture");
                just_cached = true;
                Rc::new(texture)
            }
            Cached { ref texture, ..} => texture.clone(),
        };
        if just_cached {
            self.kind = Cached { texture: texture };
        }
        let (w, h) = self.size();
        let dest = Rect::new(x, y, w, h);
        renderer.copy(&*self.texture().unwrap(), None, Some(dest));
    }
    
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
    
    pub fn texture(&self) -> Option<Rc<Texture>> {
        match self.kind {
            LabelKind::Cached { ref texture, .. } => Some(texture.clone()),
            _ => None,
        }
    }
}