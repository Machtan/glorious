extern crate sdl2;
use sdl2::rect::Rect as Sdl2Rect;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    #[inline]
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    #[inline]
    pub fn from_center<C: Into<(i32, i32)>>(center: C, width: u32, height: u32) -> Rect {
        let (cx, cy) = center.into();
        let x = cx - (width / 2) as i32;
        let y = cy - (height / 2) as i32;
        Rect::new(x, y, width, height)
    }

    #[inline]
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn moved_to(&self, x: i32, y: i32) -> Rect {
        Rect::new(x, y, self.width, self.height)
    }

    #[inline]
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    #[inline]
    pub fn moved_by(&self, dx: i32, dy: i32) -> Rect {
        Rect::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    #[inline]
    pub fn center_on(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.x = x - (self.width / 2) as i32;
        self.y = y - (self.height / 2) as i32;
    }

    #[inline]
    pub fn centered_on(&self, pos: (i32, i32)) -> Rect {
        let (x, y) = pos;
        let x = x - (self.width / 2) as i32;
        let y = y - (self.height / 2) as i32;
        Rect::new(x, y, self.width, self.height)
    }

    #[inline]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    #[inline]
    pub fn resized(&self, width: u32, height: u32) -> Rect {
        Rect::new(self.x, self.y, width, height)
    }

    #[inline]
    pub fn intersects<R: Into<Rect>>(&self, other: R) -> bool {
        let other = other.into();
        self.right() > other.left() && self.left() < other.right() && self.top() < other.bottom() &&
        self.bottom() > other.top()
    }

    #[inline]
    pub fn distance_to_rect(&self, other: Rect) -> Option<(u32, u32)> {
        if self.intersects(other) {
            return None;
        }
        let (cx, cy) = self.center();
        let (ocx, ocy) = other.center();
        let x_dist = if cx < ocx {
            // [s] [o]
            other.left() - self.right()
        } else {
            // [o] [s]
            self.left() - other.right()
        };
        let y_dist = if cy < ocy {
            other.top() - self.bottom()
        } else {
            self.top() - other.bottom()
        };
        Some((x_dist as u32, y_dist as u32))
    }

    #[inline]
    pub fn distance_to_x(&self, x: i32) -> Option<u32> {
        if x >= self.right() {
            Some((x - self.right()) as u32)
        } else if x <= self.left() {
            Some((self.left() - x) as u32)
        } else {
            None
        }
    }

    #[inline]
    pub fn distance_to_y(&self, y: i32) -> Option<u32> {
        if y <= self.top() {
            Some((self.top() - y) as u32)
        } else if y >= self.bottom() {
            Some((y - self.bottom()) as u32)
        } else {
            None
        }
    }

    #[inline]
    pub fn overlap_with(&self, other: Rect) -> Option<(u32, u32)> {
        if !self.intersects(other) {
            return None;
        }
        let (cx, cy) = self.center();
        let (ocx, ocy) = other.center();
        let x_overlap = if cx < ocx {
            // [s] [o]
            self.right() - other.left()
        } else {
            // [o] [s]
            other.right() - self.left()
        } as u32;
        let y_overlap = if cy < ocy {
            self.bottom() - other.top()
        } else {
            other.bottom() - self.top()
        } as u32;
        Some((x_overlap, y_overlap))
    }

    #[inline]
    pub fn x_overlap_with(&self, other: Rect) -> Option<u32> {
        if !self.intersects(other) {
            return None;
        }
        let (cx, _) = self.center();
        let (ocx, _) = other.center();
        let x_overlap = if cx < ocx {
            // [s] [o]
            self.right() - other.left()
        } else {
            // [o] [s]
            other.right() - self.left()
        } as u32;
        Some(x_overlap)
    }

    #[inline]
    pub fn y_overlap_with(&self, other: Rect) -> Option<u32> {
        if !self.intersects(other) {
            return None;
        }
        let (_, cy) = self.center();
        let (_, ocy) = other.center();
        let y_overlap = if cy < ocy {
            self.bottom() - other.top()
        } else {
            other.bottom() - self.top()
        } as u32;
        Some(y_overlap)
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[inline]
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn center(&self) -> (i32, i32) {
        (self.x + (self.width / 2) as i32, self.y + (self.height / 2) as i32)
    }

    #[inline]
    pub fn top(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn set_top(&mut self, y: i32) {
        self.y = y;
    }

    #[inline]
    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    #[inline]
    pub fn set_bottom(&mut self, y: i32) {
        self.y = y - self.height as i32;
    }

    #[inline]
    pub fn left(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn set_left(&mut self, x: i32) {
        self.x = x;
    }

    #[inline]
    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    #[inline]
    pub fn set_right(&mut self, x: i32) {
        self.x = x - self.width as i32;
    }

    #[inline]
    pub fn top_left(&self) -> (i32, i32) {
        self.pos()
    }

    #[inline]
    pub fn bottom_left(&self) -> (i32, i32) {
        (self.left(), self.right())
    }

    #[inline]
    pub fn top_right(&self) -> (i32, i32) {
        (self.right(), self.top())
    }

    #[inline]
    pub fn bottom_right(&self) -> (i32, i32) {
        (self.right(), self.bottom())
    }
}


impl Into<Sdl2Rect> for Rect {
    fn into(self) -> Sdl2Rect {
        Sdl2Rect::new(self.x, self.y, self.width, self.height)
    }
}

impl From<Sdl2Rect> for Rect {
    fn from(rect: Sdl2Rect) -> Rect {
        Rect::new(rect.x(), rect.y(), rect.width(), rect.height())
    }
}
