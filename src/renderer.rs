#![allow(missing_docs)]

use std::cell::{Ref, RefMut};

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;
use sdl2::rect::Rect as Sdl2Rect;
use sdl2::render::{BlendMode, Renderer as SdlRenderer, Texture};
use rect::Rect;

use color::Color;
use device::Device;

#[inline]
pub fn create_renderer<'a, 'r: 'a>(device: &'a Device<'r>) -> Renderer<'a, 'r> {
    Renderer { device: device }
}

pub struct Renderer<'a, 'r: 'a> {
    device: &'a Device<'r>,
}

impl<'a, 'r> Renderer<'a, 'r> {
    #[inline]
    pub fn borrow(&self) -> Ref<SdlRenderer<'r>> {
        self.device.borrow()
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<SdlRenderer<'r>> {
        self.device.borrow_mut()
    }

    #[inline]
    pub fn device(&self) -> &'a Device<'r> {
        self.device
    }

    #[inline]
    pub fn set_draw_color<C: Into<Color>>(&mut self, color: C) {
        self.borrow_mut().set_draw_color(color.into().into());
    }

    #[inline]
    pub fn draw_color(&self) -> Color {
        self.borrow().draw_color().into()
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
    pub fn set_clip_rect<R: Into<Sdl2Rect>>(&mut self, rect: Option<R>) {
        self.borrow_mut().set_clip_rect(rect.map(|r| r.into()));
    }

    #[inline]
    pub fn set_viewport<R: Into<Sdl2Rect>>(&mut self, rect: Option<R>) {
        self.borrow_mut().set_viewport(rect.map(|r| r.into()));
    }

    #[inline]
    pub fn viewport(&self) -> Rect {
        self.borrow_mut().viewport().into()
    }

    #[inline]
    pub fn clip_rect(&self) -> Option<Rect> {
        self.borrow().clip_rect().map(Rect::from)
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
    pub fn draw_rect<R: Into<Sdl2Rect>>(&mut self, rect: R) -> Result<(), String> {
        self.borrow_mut().draw_rect(rect.into())
    }

    #[inline]
    pub fn draw_rects(&mut self, rects: &[Sdl2Rect]) -> Result<(), String> {
        self.borrow_mut().draw_rects(rects)
    }

    #[inline]
    pub fn fill_rect<R: Into<Sdl2Rect>>(&mut self, rect: R) -> Result<(), String> {
        self.borrow_mut().fill_rect(rect.into())
    }

    #[inline]
    pub fn fill_rects(&mut self, rects: &[Sdl2Rect]) -> Result<(), String> {
        self.borrow_mut().fill_rects(rects)
    }

    #[inline]
    pub fn copy<R: Into<Sdl2Rect>>(&mut self, texture: &Texture, src: Option<R>, dst: Option<R>) {
        self.borrow_mut().copy(texture, src.map(|r| r.into()), dst.map(|r| r.into()));
    }

    #[inline]
    pub fn copy_ex<R: Into<Sdl2Rect>>(&mut self,
                                      texture: &Texture,
                                      src: Option<R>,
                                      dst: Option<R>,
                                      angle: f64,
                                      center: Option<Point>,
                                      flip_horizontal: bool,
                                      flip_vertical: bool)
                                      -> Result<(), String> {
        self.borrow_mut().copy_ex(texture,
                                  src.map(|r| r.into()),
                                  dst.map(|r| r.into()),
                                  angle,
                                  center,
                                  flip_horizontal,
                                  flip_vertical)
    }

    #[inline]
    pub fn read_pixels<R: Into<Sdl2Rect>>(&self,
                                          rect: Option<R>,
                                          format: PixelFormatEnum)
                                          -> Result<Vec<u8>, String> {
        self.borrow().read_pixels(rect.map(|r| r.into()), format)
    }
}
