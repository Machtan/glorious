use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer as SdlRenderer, Texture, TextureValueError};
use sdl2::surface::Surface;
use sdl2_image::LoadTexture;

#[derive(Clone)]
pub struct Renderer<'a> {
    inner: Rc<RefCell<SdlRenderer<'a>>>,
}

impl<'a> Renderer<'a> {
    #[inline]
    pub fn new(inner: SdlRenderer<'a>) -> Renderer<'a> {
        Renderer { inner: Rc::new(RefCell::new(inner)) }
    }

    #[inline]
    pub fn into_inner(self) -> Rc<RefCell<SdlRenderer<'a>>> {
        self.inner
    }

    #[inline]
    pub fn load_texture<P>(&self, path: P) -> Result<Texture, String>
        where P: AsRef<Path>
    {
        self.inner.borrow().load_texture(path.as_ref())
    }

    #[inline]
    pub fn create_texture_from_surface(&self,
                                       surface: &Surface)
                                       -> Result<Texture, TextureValueError> {
        self.inner.borrow().create_texture_from_surface(surface)
    }

    #[inline]
    pub fn copy(&mut self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>) {
        self.inner.borrow_mut().copy(texture, src, dst);
    }

    #[inline]
    pub fn set_draw_color(&mut self, color: Color) {
        self.inner.borrow_mut().set_draw_color(color);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.borrow_mut().clear();
    }

    #[inline]
    pub fn present(&mut self) {
        self.inner.borrow_mut().present();
    }

    #[inline]
    pub fn scale(&self) -> (f32, f32) {
        self.inner.borrow().scale()
    }
}
