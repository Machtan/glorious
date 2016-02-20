/// Creates a boxed closure that maps SDL2 events to messages.
#[macro_export]
macro_rules! map_event {
    ($pat:pat, $message:expr) => {{
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
#[macro_export]
macro_rules! map_key_pressed {
    ($keycode:expr, $message:expr) => {{
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
#[macro_export]
macro_rules! map_key_released {
    ($keycode:expr, $message:expr) => {{
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
#[macro_export]
macro_rules! map_scan_pressed {
    ($scancode:expr, $message:expr) => {{
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
#[macro_export]
macro_rules! map_scan_released {
    ($scancode:expr, $message:expr) => {{
        Box::new(|event: &Event, push: &mut FnMut(_)| {
            if let KeyUp { scancode: Some(scan), .. } = *event {
                if scan == $scancode {
                    push($message);
                }
            }
        })
    }};
}
