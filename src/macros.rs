/// Creates a boxed closure that maps SDL2 events to messages.
#[macro_export]
macro_rules! map_event {
    ($pat:pat, $message:expr) => {{
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &$pat = event {
                messages.push($message);
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
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &KeyDown { keycode: Some(key), .. } = event {
                if key == $keycode {
                    messages.push($message);
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
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &KeyUp { keycode: Some(key), .. } = event {
                if key == $keycode {
                    messages.push($message);
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
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &KeyDown { scancode: Some(scan), .. } = event {
                if scan == $scancode {
                    messages.push($message);
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
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &KeyUp { scancode: Some(scan), .. } = event {
                if scan == $scancode {
                    messages.push($message);
                }
            }
        })
    }};
}
