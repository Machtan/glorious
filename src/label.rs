
use std::rc::Rc;
use sdl2::render::{Texture, Renderer, TextureQuery};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use resources::ResourceManager;

enum LabelKind {
    Cached { texture: Rc<Texture>, size: (u32, u32) },
    Uncached { font: String, text: String, color: (u8, u8, u8, u8) },
}

pub struct Label {
    kind: LabelKind,
}

impl Label {
    pub fn new(font: &str, text: &str, color: (u8, u8, u8, u8)) -> Label {
        Label {
            kind: LabelKind::Uncached {
                font: String::from(font),
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
            let TextureQuery { width, height, ..} = texture.query();
            self.kind = Cached { texture: texture, size: (width, height) };
        }
        let (w, h) = self.size().unwrap();
        let dest = Rect::new(x, y, w, h);
        renderer.copy(&*self.texture().unwrap(), None, Some(dest));
    }
    
    pub fn size(&self) -> Option<(u32, u32)> {
        match self.kind {
            LabelKind::Cached { size, .. } => Some(size),
            _ => None,
        }
    }
    
    pub fn texture(&self) -> Option<Rc<Texture>> {
        match self.kind {
            LabelKind::Cached { ref texture, .. } => Some(texture.clone()),
            _ => None,
        }
    }
}