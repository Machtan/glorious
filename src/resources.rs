use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::path::PathBuf;
use std::rc::Rc;

use sdl2::render::Texture;
use sdl2_ttf::{Sdl2TtfContext, Font};

use renderer::Renderer;

/// A resource manager responsible for loading and caching assets.
#[derive(Clone)]
pub struct ResourceManager<'a> {
    prefix: PathBuf,
    renderer: Renderer<'a>,
    ttf_ctx: Rc<Sdl2TtfContext>,
    textures: RefCell<HashMap<String, Rc<Texture>>>,
    // This hack with `Cow` allows us to index the `HashMap` with tuples of unowned strings.
    fonts: RefCell<HashMap<(Cow<'static, str>, u16), Rc<Font>>>,
}

impl<'a> ResourceManager<'a> {
    /// Creates a new resource manager.
    ///
    /// The `renderer` and `ttf_ctx` are used when loading assets.
    pub fn new(renderer: Renderer<'a>, ttf_ctx: Rc<Sdl2TtfContext>) -> ResourceManager<'a> {
        ResourceManager::with_prefix("", renderer, ttf_ctx)
    }

    pub fn with_prefix<P>(prefix: P,
                          renderer: Renderer<'a>,
                          ttf_ctx: Rc<Sdl2TtfContext>)
                          -> ResourceManager<'a>
        where P: Into<PathBuf>
    {
        ResourceManager {
            prefix: prefix.into(),
            renderer: renderer,
            ttf_ctx: ttf_ctx,
            textures: Default::default(),
            fonts: Default::default(),
        }
    }

    /// Ensures a texture is loaded and returns it.
    ///
    /// If a texture for the given path is already cached, it will be
    /// returned directly. Otherwise the texture will be loaded from
    /// disk.
    ///
    /// # Panics
    ///
    /// Panics if the texture is not cached, and loading it fails (e.g.
    /// if the file pointed to by path does not exist or is malformed).
    pub fn texture(&self, path: &str) -> Rc<Texture> {
        if let Some(texture) = self.textures.borrow().get(path) {
            return texture.clone();
        }
        let mut path_buf = self.prefix.clone();
        path_buf.push(path);
        let texture = self.renderer.load_texture(&path_buf).expect("could not load texture");
        let texture = Rc::new(texture);
        self.textures.borrow_mut().insert(path.to_owned(), texture.clone());
        texture
    }

    /// Ensures a font is loaded and returns it.
    ///
    /// If a font for the given path and point size is already cached,
    /// it will be returned directly. Otherwise the texture will be
    /// loaded from disk. If high-dpi mode is enabled for the renderer,
    /// then the returned font is automatically upscaled appropriately.
    ///
    /// # Panics
    ///
    /// Panics if the font is not cached, and loading it fails (e.g.
    /// if the file pointed to by path does not exist or is malformed).
    pub fn font(&self, path: &str, point_size: u16) -> Rc<Font> {
        if let Some(font) = self.fonts.borrow().get(&(Cow::Borrowed(path), point_size)) {
            return font.clone();
        }
        let (sx, sy) = self.renderer.scale();
        let scale = if sx >= sy {
            sx
        } else {
            sy
        };

        let point_size = (point_size as f32 * scale) as u16;

        let mut path_buf = self.prefix.clone();
        path_buf.push(path);
        let font = self.ttf_ctx.load_font(path.as_ref(), point_size).expect("could not load font");
        let font = Rc::new(font);
        self.fonts.borrow_mut().insert((path.to_owned().into(), point_size), font.clone());
        font
    }

    /// Returns the renderer this resource manager was created with.
    #[inline]
    pub fn renderer(&self) -> Renderer<'a> {
        self.renderer.clone()
    }

    /// Returns the `Sdl2TtfContext` that this resource manager was
    /// created with.
    #[inline]
    pub fn ttf_context(&self) -> Rc<Sdl2TtfContext> {
        self.ttf_ctx.clone()
    }
}

struct IterDebug<I>(I)
    where I: Clone + IntoIterator,
          I::Item: Debug;

impl<I> Debug for IterDebug<I>
    where I: Clone + IntoIterator,
          I::Item: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;
        for value in self.0.clone() {
            write!(f, "{:?}", value)?;
        }
        f.write_str("]")
    }
}

impl<'a> Debug for ResourceManager<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ResourceManager")
            .field("textures", &IterDebug(self.textures.borrow().keys()))
            .field("fonts", &IterDebug(self.fonts.borrow().keys()))
            .field("renderer", &(..))
            .finish()
    }
}
