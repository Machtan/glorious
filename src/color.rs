use sdl2::pixels::Color as SdlColor;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    #[inline]
    pub fn mul_alpha(self, alpha: u8) -> Color {
        let Color(r, g, b, a) = self;
        let mul: u16 = a as u16 * alpha as u16;
        Color(r, g, b, (mul / 0xff) as u8)
    }
}

impl From<SdlColor> for Color {
    #[inline]
    fn from(color: SdlColor) -> Color {
        match color {
            SdlColor::RGB(r, g, b) => Color(r, g, b, 0xff),
            SdlColor::RGBA(r, g, b, a) => Color(r, g, b, a),
        }
    }
}

impl From<Color> for SdlColor {
    #[inline]
    fn from(color: Color) -> SdlColor {
        let Color(r, g, b, a) = color;
        SdlColor::RGBA(r, g, b, a)
    }
}
