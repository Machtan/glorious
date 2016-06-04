#![allow(missing_docs)]

use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use ref_filter_map::{ref_filter_map, ref_mut_filter_map};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Renderer as SdlRenderer, RendererInfo, Texture, TextureAccess,
                   TextureValueError};
use sdl2::surface::SurfaceRef;
use sdl2::video::WindowRef;
use sdl2_image::LoadTexture;

pub fn init_renderer<'a>(renderer: SdlRenderer<'a>) -> (Device<'a>, Renderer<'a>) {
    let rc = Rc::new(RefCell::new(renderer));
    let device = Device { inner: rc.clone() };
    let renderer = Renderer { device: Device { inner: rc } };
    (device, renderer)
}

pub struct Device<'a> {
    inner: Rc<RefCell<SdlRenderer<'a>>>,
}

impl<'a> Device<'a> {
    #[inline]
    pub fn borrow(&self) -> Ref<SdlRenderer<'a>> {
        self.inner.borrow()
    }

    #[inline]
    pub fn info(&self) -> RendererInfo {
        self.borrow().info()
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
        self.borrow().viewport()
    }
}

pub struct Renderer<'a> {
    device: Device<'a>,
}

impl<'a> Renderer<'a> {
    #[inline]
    pub fn borrow_mut(&mut self) -> RefMut<SdlRenderer<'a>> {
        self.device.inner.borrow_mut()
    }

    #[inline]
    pub fn borrow_window_mut(&mut self) -> Option<RefMut<WindowRef>> {
        ref_mut_filter_map(self.borrow_mut(), |r| r.window_mut())
    }

    #[inline]
    pub fn borrow_surface_mut(&mut self) -> Option<RefMut<SurfaceRef>> {
        ref_mut_filter_map(self.borrow_mut(), |r| r.surface_mut())
    }

    #[inline]
    pub fn set_draw_color(&mut self, color: Color) {
        self.borrow_mut().set_draw_color(color);
    }

    #[inline]
    pub fn draw_color(&self) -> Color {
        self.borrow().draw_color()
    }

    #[inline]
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        self.borrow_mut().set_blend_mode(blend);
    }

    #[inline]
    pub fn blend_mode(&self) -> BlendMode {
        self.borrow().blend_mode()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.borrow_mut().clear();
    }

    #[inline]
    pub fn present(&mut self) {
        self.borrow_mut().present();
    }

    #[inline]
    pub fn set_clip_rect(&mut self, rect: Option<Rect>) {
        self.borrow_mut().set_clip_rect(rect);
    }

    #[inline]
    pub fn clip_rect(&self) -> Option<Rect> {
        self.borrow().clip_rect()
    }

    #[inline]
    pub fn draw_point(&mut self, point: Point) -> Result<(), String> {
        self.borrow_mut().draw_point(point)
    }

    #[inline]
    pub fn draw_points(&mut self, points: &[Point]) -> Result<(), String> {
        self.borrow_mut().draw_points(points)
    }

    #[inline]
    pub fn draw_line(&mut self, start: Point, end: Point) -> Result<(), String> {
        self.borrow_mut().draw_line(start, end)
    }

    #[inline]
    pub fn draw_lines(&mut self, points: &[Point]) -> Result<(), String> {
        self.borrow_mut().draw_lines(points)
    }

    #[inline]
    pub fn draw_rect(&mut self, rect: Rect) -> Result<(), String> {
        self.borrow_mut().draw_rect(rect)
    }

    #[inline]
    pub fn draw_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        self.borrow_mut().draw_rects(rects)
    }

    #[inline]
    pub fn fill_rect(&mut self, rect: Rect) -> Result<(), String> {
        self.borrow_mut().fill_rect(rect)
    }

    #[inline]
    pub fn fill_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        self.borrow_mut().fill_rects(rects)
    }

    #[inline]
    pub fn copy(&mut self, texture: &Texture, src: Option<Rect>, dst: Option<Rect>) {
        self.borrow_mut().copy(texture, src, dst);
    }

    #[inline]
    pub fn copy_ex(&mut self,
                   texture: &Texture,
                   src: Option<Rect>,
                   dst: Option<Rect>,
                   angle: f64,
                   center: Option<Point>,
                   flip_horizontal: bool,
                   flip_vertical: bool)
                   -> Result<(), String> {
        self.borrow_mut().copy_ex(texture,
                                  src,
                                  dst,
                                  angle,
                                  center,
                                  flip_horizontal,
                                  flip_vertical)
    }

    #[inline]
    pub fn read_pixels(&self,
                       rect: Option<Rect>,
                       format: PixelFormatEnum)
                       -> Result<Vec<u8>, String> {
        self.borrow().read_pixels(rect, format)
    }
}

impl<'a> Deref for Renderer<'a> {
    type Target = Device<'a>;

    fn deref(&self) -> &Device<'a> {
        &self.device
    }
}
