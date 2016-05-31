use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::rc::Rc;

use sdl2::render::Texture;
use sdl2_ttf::{Sdl2TtfContext, Font};

use misc::Ellipsis;
use renderer::Renderer;

#[derive(Clone)]
pub struct ResourceManager<'a> {
    renderer: Renderer<'a>,
    ttf_ctx: Rc<Sdl2TtfContext>,
    textures: RefCell<HashMap<String, Rc<Texture>>>,
    // This hack with `Cow` allows us to index the `HashMap` with tuples of unowned strings.
    fonts: RefCell<HashMap<(Cow<'static, str>, u16), Rc<Font>>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new(renderer: Renderer<'a>, ttf_ctx: Rc<Sdl2TtfContext>) -> Self {
        ResourceManager {
            renderer: renderer,
            ttf_ctx: ttf_ctx,
            textures: Default::default(),
            fonts: Default::default(),
        }
    }

    pub fn texture(&self, path: &str) -> Rc<Texture> {
        if let Some(texture) = self.textures.borrow().get(path) {
            return texture.clone();
        }
        let texture = self.renderer.load_texture(path).expect("could not load texture");
        let texture = Rc::new(texture);
        self.textures.borrow_mut().insert(path.to_owned(), texture.clone());
        texture
    }

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

        // TODO: Figure out if it should actually be divided by the scale.
        let point_size = (point_size as f32 * scale) as u16;

        let font = self.ttf_ctx.load_font(path.as_ref(), point_size).expect("could not load font");
        let font = Rc::new(font);
        self.fonts.borrow_mut().insert((path.to_owned().into(), point_size), font.clone());
        font
    }

    #[inline]
    pub fn renderer(&self) -> Renderer<'a> {
        self.renderer.clone()
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
            .field("renderer", &Ellipsis)
            .finish()
    }
}
