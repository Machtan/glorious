use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};

pub trait InputManager<M> {
    /// Pushes the messages generated from the given event to the passed handler.
    fn handle(&self, event: &Event, push: &mut FnMut(M));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputPatternKind {
    Quit,
    KeyPressed {
        key: Keycode,
        is_scancode: bool,
        modifiers: Mod,
    },
    KeyReleased {
        key: Keycode,
        is_scancode: bool,
        modifiers: Mod,
    },
    KeyRepeated {
        key: Keycode,
        is_scancode: bool,
        modifiers: Mod,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct InputPattern {
    pub window_id: u32,
    pub kind: InputPatternKind,
}

impl InputPattern {
    fn new(window_id: u32, kind: InputPatternKind) -> InputPattern {
        InputPattern {
            window_id: window_id,
            kind: kind,
        }
    }

    pub fn quit() -> InputPattern {
        InputPattern::new(0, InputPatternKind::Quit)
    }

    pub fn key_pressed(window_id: u32,
                       key: Keycode,
                       is_scancode: bool,
                       modifiers: Option<Mod>)
                       -> InputPattern {
        InputPattern::new(window_id,
                          InputPatternKind::KeyPressed {
                              key: key,
                              is_scancode: is_scancode,
                              modifiers: modifiers.unwrap_or(Mod::empty()),
                          })
    }

    pub fn key_released(window_id: u32,
                        key: Keycode,
                        is_scancode: bool,
                        modifiers: Option<Mod>)
                        -> InputPattern {
        InputPattern::new(window_id,
                          InputPatternKind::KeyReleased {
                              key: key,
                              is_scancode: is_scancode,
                              modifiers: modifiers.unwrap_or(Mod::empty()),
                          })
    }

    pub fn key_repeated(window_id: u32,
                        key: Keycode,
                        is_scancode: bool,
                        modifiers: Option<Mod>)
                        -> InputPattern {
        InputPattern::new(window_id,
                          InputPatternKind::KeyRepeated {
                              key: key,
                              is_scancode: is_scancode,
                              modifiers: modifiers.unwrap_or(Mod::empty()),
                          })
    }

    pub fn matches(&self, event: &Event) -> bool {
        use sdl2::event::Event::*;

        // TODO: Only tests for `Quit` events for now!
        match *event {
            Quit { .. } => self.kind == InputPatternKind::Quit,
            _ => unimplemented!(),
        }
    }
}

// TODO: The 'static (and probably Fn) bounds make the following a lot less useful.

/// A struct to map events to game messages.
pub struct BoxedInputMapper<M: 'static> {
    mappers: Vec<Box<Fn(&Event, &mut FnMut(M))>>,
}

impl<M: 'static> BoxedInputMapper<M> {
    /// Creates a new input mapper
    pub fn new() -> Self {
        BoxedInputMapper { mappers: Vec::new() }
    }

    /// Adds a new event mapping function to the input mapper.
    pub fn add(&mut self, mapper: Box<Fn(&Event, &mut FnMut(M))>) {
        self.mappers.push(mapper)
    }

    pub fn add_pattern_with<F>(&mut self, pattern: InputPattern, func: F)
        where F: 'static + Fn() -> M
    {
        self.mappers.push(Box::new(move |event, push| {
            if pattern.matches(event) {
                push(func());
            }
        }));
    }
}

impl<M: 'static + Clone> BoxedInputMapper<M> {
    pub fn add_pattern(&mut self, pattern: InputPattern, message: M) {
        self.add_pattern_with(pattern, move || message.clone());
    }
}

impl<M: 'static> InputManager<M> for BoxedInputMapper<M> {
    fn handle(&self, event: &Event, push: &mut FnMut(M)) {
        for mapper in &self.mappers {
            mapper(event, push);
        }
    }
}
