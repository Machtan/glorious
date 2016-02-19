
/// Creates a boxed closure that maps an SDL2 Event pattern into a game message.
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

/// Creates a boxed closure that maps a key press with the given keycode to
/// a specific message.
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

/// Creates a boxed closure that maps a key release with the given keycode to
/// a specific message.
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

/// Creates a boxed closure that maps a key press with the given scancode to
/// a specific message.
#[macro_export]
macro_rules! map_scan_pressed {
    (scan: $scancode:expr, $message:expr) => {{
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &KeyDown { scancode: Some(scan), .. } = event {
                if scan == $scancode {
                    messages.push($message);
                }
            }
        })
    }};
}

/// Creates a boxed closure that maps a key release with the given scancode to
/// a specific message.
#[macro_export]
macro_rules! map_scan_released {
    (scan_up: $scancode:expr, $message:expr) => {{
        Box::new(|event: &Event, messages: &mut Vec<Message>| {
            if let &KeyUp { scancode: Some(scan), .. } = event {
                if scan == $scancode {
                    messages.push($message);
                }
            }
        })
    }};
}