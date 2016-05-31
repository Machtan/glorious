/// Creates a boxed closure that maps SDL2 events to messages.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate glorious;
/// # extern crate sdl2;
/// # fn main() {
/// use sdl2::mouse::Mouse;
/// use sdl2::event::Event::*;
/// use glorious::BoxedInputMapper;
///
/// let mut mapper = BoxedInputMapper::new();
///
/// mapper.add(map_event!(
///     MouseButtonDown { x, y, mouse_btn: Mouse::Left, ..},
///     "Your message type here (using any values from the pattern)"
/// ));
/// # }
/// ```
#[macro_export]
macro_rules! map_event {
    ($pat:pat, $message:expr) => {{
        use sdl2::event::Event;
        Box::new(|event: &Event, push: &mut FnMut(_)| {
            if let $pat = *event {
                push($message);
            }
        })
    }};
}

/// Creates a boxed closure that maps key presses to messages.
///
/// The key is recognized by the event's `keycode`.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate glorious;
/// # extern crate sdl2;
/// # fn main() {
/// use sdl2::keyboard::Keycode;
/// use glorious::BoxedInputMapper;
///
/// let mut mapper = BoxedInputMapper::new();
///
/// mapper.add(map_key_pressed!(Keycode::Left, "Your message type here"));
/// # }
/// ```
#[macro_export]
macro_rules! map_key_pressed {
    ($keycode:expr, $message:expr) => {{
        use sdl2::event::Event;
        use sdl2::event::Event::KeyDown;
        Box::new(|event: &Event, push: &mut FnMut(_)| {
            if let KeyDown { keycode: Some(key), .. } = *event {
                if key == $keycode {
                    push($message);
                }
            }
        })
    }};
}

/// Creates a boxed closure that maps key releases to messages.
///
/// The key is recognized by the event's `keycode`.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate glorious;
/// # extern crate sdl2;
/// # fn main() {
/// use sdl2::keyboard::Keycode;
/// use glorious::BoxedInputMapper;
///
/// let mut mapper = BoxedInputMapper::new();
///
/// mapper.add(map_key_released!(Keycode::Left, "Your message type here"));
/// # }
/// ```
#[macro_export]
macro_rules! map_key_released {
    ($keycode:expr, $message:expr) => {{
        use sdl2::event::Event;
        use sdl2::event::Event::KeyUp;
        Box::new(|event: &Event, push: &mut FnMut(_)| {
            if let KeyUp { keycode: Some(key), .. } = *event {
                if key == $keycode {
                    push($message);
                }
            }
        })
    }};
}

/// Creates a boxed closure that maps key presses to messages.
///
/// The key is recognized by the event's `scancode`.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate glorious;
/// # extern crate sdl2;
/// # fn main() {
/// use sdl2::keyboard::Scancode;
/// use glorious::BoxedInputMapper;
///
/// let mut mapper = BoxedInputMapper::new();
///
/// mapper.add(map_scan_pressed!(Scancode::Left, "Your message type here"));
/// # }
/// ```
#[macro_export]
macro_rules! map_scan_pressed {
    ($scancode:expr, $message:expr) => {{
        use sdl2::event::Event;
        use sdl2::event::Event::KeyDown;
        Box::new(|event: &Event, push: &mut FnMut(_)| {
            if let KeyDown { scancode: Some(scan), .. } = *event {
                if scan == $scancode {
                    push($message);
                }
            }
        })
    }};
}

/// Creates a boxed closure that maps key releases to messages.
///
/// The key is recognized by the event's `scancode`.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate glorious;
/// # extern crate sdl2;
/// # fn main() {
/// use sdl2::keyboard::Scancode;
/// use glorious::BoxedInputMapper;
///
/// let mut mapper = BoxedInputMapper::new();
///
/// mapper.add(map_scan_released!(Scancode::Left, "Your message type here"));
/// # }
/// ```
#[macro_export]
macro_rules! map_scan_released {
    ($scancode:expr, $message:expr) => {{
        use sdl2::event::Event;
        use sdl2::event::Event::KeyUp;
        Box::new(|event: &Event, push: &mut FnMut(_)| {
            if let KeyUp { scancode: Some(scan), .. } = *event {
                if scan == $scancode {
                    push($message);
                }
            }
        })
    }};
}
