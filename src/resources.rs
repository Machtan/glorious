
use std::fmt::{self, Debug};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use sdl2_image::LoadTexture;
use sdl2_ttf::{Sdl2TtfContext, Font};
use sprite::Sprite;
use label::Label;

pub struct ResourceManager {
    textures: HashMap<String, Rc<Texture>>,
    sprites: HashMap<String, Sprite>,
    fonts: HashMap<String, Font>,
    labels: HashMap<String, Label>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            textures: HashMap::new(),
            sprites: HashMap::new(),
            fonts:HashMap::new(),
            labels: HashMap::new(),
        }
    }
    
    pub fn load_texture(&mut self, path: &str, renderer: &mut Renderer) 
            -> Result<(), String> {
        let texture = renderer.load_texture(&Path::new(path))?;
        self.textures.insert(String::from(path), Rc::new(texture));
        Ok(())
    }
    
    pub fn load_font(&mut self, name: &str, path: &str, point_size: u16, 
            context: &Sdl2TtfContext)
            -> Result<(), String> {
        let font = try!(context.load_font(&Path::new(path), point_size));
        self.fonts.insert(String::from(name), font);
        Ok(())
    }
    
    pub fn create_sprite(&mut self, name: &str, texture: &str, source: Option<Rect>)
            -> Result<(), String> {
        let texture = if let Some(texture) = self.textures.get(texture) {
            texture.clone()
        } else {
            return Err(format!("The texture at '{}' was not loaded", texture));
        };
        let sprite = Sprite::new(texture, source);
        self.sprites.insert(String::from(name), sprite);
        Ok(())
    }
    
    pub fn create_label(&mut self, name: &str, font: &str, text: &str, 
            color: (u8, u8, u8, u8)) {
        let label = Label::new(font, text, color);
        self.labels.insert(String::from(name), label);
    }
        
    pub fn sprite(&self, name: &str) -> Option<&Sprite> {
        self.sprites.get(name)
    }
    
    pub fn label(&self, name: &str) -> Option<&Label> {
        self.labels.get(name)
    }
    
    pub fn font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }
}

impl Debug for ResourceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "ResourceManager {{ textures: [ ")?;
        for key in self.textures.keys() {
            key.fmt(f)?;
            write!(f, ", ")?;
        }
        write!(f, ", sprites: ")?;
        self.sprites.fmt(f)?;
        write!(f, "}}")
    }
}