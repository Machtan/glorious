#![allow(missing_docs)]

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::path::Path;

use ref_filter_map::{ref_filter_map, ref_mut_filter_map};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Renderer as SdlRenderer, RendererInfo, Texture, TextureAccess,
                   TextureValueError};
use sdl2::surface::SurfaceRef;
use sdl2::video::WindowRef;
use sdl2_image::LoadTexture;

use renderer::{create_renderer, Renderer};
use rect::Rect;

pub struct Device<'r> {
    inner: RefCell<SdlRenderer<'r>>,
    renderer_created: Cell<bool>,
}

impl<'r> Device<'r> {
    #[inline]
    pub fn new(inner: SdlRenderer<'r>) -> Device<'r> {
        Device {
            inner: RefCell::new(inner),
            renderer_created: Cell::new(false),
        }
    }

    #[inline]
    pub fn create_renderer<'a>(&'a self) -> Renderer<'a, 'r> {
        assert!(!self.renderer_created.get(),
                "renderer already created for this device");
        self.renderer_created.set(true);
        create_renderer(self)
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<SdlRenderer<'r>> {
        self.inner.borrow_mut()
    }

    #[inline]
    pub fn borrow(&self) -> Ref<SdlRenderer<'r>> {
        self.inner.borrow()
    }

    #[inline]
    pub fn into_inner(self) -> SdlRenderer<'r> {
        self.inner.into_inner()
    }

    #[inline]
    pub fn info(&self) -> RendererInfo {
        self.inner.borrow().info()
    }

    #[inline]
    pub fn borrow_window(&self) -> Option<Ref<WindowRef>> {
        ref_filter_map(self.borrow(), |r| r.window())
    }

    #[inline]
    pub fn borrow_surface(&self) -> Option<Ref<SurfaceRef>> {
        ref_filter_map(self.borrow(), |r| r.surface())
    }

    #[inline]
    pub fn borrow_window_mut(&self) -> Option<RefMut<WindowRef>> {
        ref_mut_filter_map(self.borrow_mut(), |r| r.window_mut())
    }

    #[inline]
    pub fn borrow_surface_mut(&mut self) -> Option<RefMut<SurfaceRef>> {
        ref_mut_filter_map(self.borrow_mut(), |r| r.surface_mut())
    }

    #[inline]
    pub fn load_texture<P>(&self, path: P) -> Result<Texture, String>
        where P: AsRef<Path>
    {
        self.borrow().load_texture(path.as_ref())
    }

    #[inline]
    pub fn create_texture(&self,
                          format: PixelFormatEnum,
                          access: TextureAccess,
                          width: u32,
                          height: u32)
                          -> Result<Texture, TextureValueError> {
        self.borrow().create_texture(format, access, width, height)
    }

    #[inline]
    pub fn create_texture_static(&self,
                                 format: PixelFormatEnum,
                                 width: u32,
                                 height: u32)
                                 -> Result<Texture, TextureValueError> {
        self.borrow().create_texture_static(format, width, height)
    }

    #[inline]
    pub fn create_texture_streaming(&self,
                                    format: PixelFormatEnum,
                                    width: u32,
                                    height: u32)
                                    -> Result<Texture, TextureValueError> {
        self.borrow().create_texture_streaming(format, width, height)
    }

    #[inline]
    pub fn create_texture_target(&self,
                                 format: PixelFormatEnum,
                                 width: u32,
                                 height: u32)
                                 -> Result<Texture, TextureValueError> {
        self.borrow().create_texture_target(format, width, height)
    }

    #[inline]
    pub fn create_texture_from_surface<S>(&self, surface: S) -> Result<Texture, TextureValueError>
        where S: AsRef<SurfaceRef>
    {
        self.borrow().create_texture_from_surface(surface)
    }

    #[inline]
    pub fn scale(&self) -> (f32, f32) {
        self.borrow().scale()
    }

    #[inline]
    pub fn output_size(&self) -> Result<(u32, u32), String> {
        self.borrow().output_size()
    }

    #[inline]
    pub fn logical_size(&self) -> (u32, u32) {
        self.borrow().logical_size()
    }

    #[inline]
    pub fn viewport(&self) -> Rect {
        self.borrow().viewport().into()
    }
}
