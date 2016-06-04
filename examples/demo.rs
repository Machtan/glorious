#[macro_use]
extern crate glorious;
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use std::rc::Rc;

use glorious::{Behavior, BoxedInputMapper, Game, Renderer, Sprite};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2_image::{INIT_PNG, INIT_JPG};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Message {
    StartMovingLeft,
    StopMovingLeft,
    StartMovingRight,
    StopMovingRight,
    StartMovingUp,
    StopMovingUp,
    StartMovingDown,
    StopMovingDown,
    Quit,
}

const PLAYER_MOVE_SPEED: i32 = 4;
const PLAYER_TEXTURE: &'static str = "assets/raccoon.png";

#[derive(Debug)]
struct Player {
    rect: Rect,
    vx: i32,
    vy: i32,
    left_down: bool,
    right_down: bool,
    up_down: bool,
    down_down: bool,
    sprite: Sprite,
}

impl Player {
    pub fn new(x: i32, y: i32, renderer: &mut Renderer) -> Self {
        let texture = renderer.load_texture(&Path::new(PLAYER_TEXTURE))
            .expect("Could not load the player texture");
        Player {
            vx: 0,
            vy: 0,
            rect: Rect::new(x, y, 32, 32),
            left_down: false,
            right_down: false,
            up_down: false,
            down_down: false,
            sprite: Sprite::new(Rc::new(texture), None),
        }
    }
}

impl Behavior<GameState> for Player {
    type Message = Message;

    /// Updates the object each frame.
    fn update(&mut self, _state: &mut GameState, _queue: &mut Vec<Self::Message>) {
        self.rect.offset(self.vx, self.vy);
    }

    /// Handles new messages since the last frame.
    fn handle(&mut self,
              _state: &mut GameState,
              message: Self::Message,
              _queue: &mut Vec<Self::Message>) {
        use self::Message::*;
        match message {
            StartMovingLeft => {
                self.left_down = true;
                if self.right_down {
                    self.vx = 0;
                } else {
                    self.vx = -PLAYER_MOVE_SPEED;
                }
            }
            StopMovingLeft => {
                self.left_down = false;
                if self.right_down {
                    self.vx = PLAYER_MOVE_SPEED;
                } else {
                    self.vx = 0;
                }
            }
            StartMovingRight => {
                self.right_down = true;
                if self.left_down {
                    self.vx = 0;
                } else {
                    self.vx = PLAYER_MOVE_SPEED;
                }
            }
            StopMovingRight => {
                self.right_down = false;
                if self.left_down {
                    self.vx = -PLAYER_MOVE_SPEED;
                } else {
                    self.vx = 0;
                }
            }
            StartMovingUp => {
                self.up_down = true;
                if self.down_down {
                    self.vy = 0;
                } else {
                    self.vy = -PLAYER_MOVE_SPEED;
                }
            }
            StopMovingUp => {
                self.up_down = false;
                if self.down_down {
                    self.vy = PLAYER_MOVE_SPEED;
                } else {
                    self.vy = 0;
                }
            }
            StartMovingDown => {
                self.down_down = true;
                if self.up_down {
                    self.vy = 0;
                } else {
                    self.vy = PLAYER_MOVE_SPEED;
                }
            }
            StopMovingDown => {
                self.down_down = false;
                if self.up_down {
                    self.vy = -PLAYER_MOVE_SPEED;
                } else {
                    self.vy = 0;
                }
            }
            _ => {}
        }
    }

    /// Renders the object.
    fn render(&mut self, _state: &GameState, renderer: &mut Renderer) {
        self.sprite.render(renderer, self.rect.x(), self.rect.y(), Some((128, 128)));
    }
}

struct GameState {
    example: &'static str,
}

impl GameState {
    pub fn new() -> Self {
        GameState { example: "empty" }
    }
}

struct GameLogic {
    objects: Vec<Box<Behavior<GameState, Message = Message>>>,
}

impl GameLogic {
    pub fn new() -> GameLogic {
        GameLogic { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<Behavior<GameState, Message = Message>>) {
        self.objects.push(object);
    }
}

impl Behavior<GameState> for GameLogic {
    type Message = Message;

    /// Initializes the object when it is added to the game.
    fn initialize(&mut self, state: &mut GameState, new_messages: &mut Vec<Self::Message>) {
        println!("State example : {}", state.example);

        for object in self.objects.iter_mut() {
            object.initialize(state, new_messages);
        }
    }

    /// Updates the object each frame.
    fn update(&mut self, state: &mut GameState, queue: &mut Vec<Self::Message>) {
        for object in self.objects.iter_mut() {
            object.update(state, queue);
        }
    }

    /// Handles new messages since the last frame.
    fn handle(&mut self,
              state: &mut GameState,
              message: Self::Message,
              queue: &mut Vec<Self::Message>) {
        for object in self.objects.iter_mut() {
            object.handle(state, message, queue);
        }
    }

    /// Renders the object.
    fn render(&mut self, state: &GameState, renderer: &mut Renderer) {
        for object in &mut self.objects {
            object.render(state, renderer);
        }
    }
}

fn main() {
    println!("Hello Glory!");

    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2_image::init(INIT_PNG | INIT_JPG).unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let (_, mut renderer) = glorious::init_renderer(window.renderer().build().unwrap());

    // Initialize the game state
    let mut state = GameState::new();
    state.example = "A new value for the state :u";

    let mut logic = GameLogic::new();
    let player = Player::new(50, 50, &mut renderer);
    logic.add(Box::new(player));

    let mut mapper = BoxedInputMapper::new();

    mapper.add(map_event!(Event::Quit { .. }, Message::Quit));

    mapper.add(map_key_pressed!(Keycode::Up, Message::StartMovingUp));
    mapper.add(map_key_pressed!(Keycode::Down, Message::StartMovingDown));
    mapper.add(map_key_pressed!(Keycode::Left, Message::StartMovingLeft));
    mapper.add(map_key_pressed!(Keycode::Right, Message::StartMovingRight));

    mapper.add(map_key_released!(Keycode::Up, Message::StopMovingUp));
    mapper.add(map_key_released!(Keycode::Down, Message::StopMovingDown));
    mapper.add(map_key_released!(Keycode::Left, Message::StopMovingLeft));
    mapper.add(map_key_released!(Keycode::Right, Message::StopMovingRight));

    mapper.add(map_scan_pressed!(Scancode::W, Message::StartMovingUp));
    mapper.add(map_scan_pressed!(Scancode::S, Message::StartMovingDown));
    mapper.add(map_scan_pressed!(Scancode::A, Message::StartMovingLeft));
    mapper.add(map_scan_pressed!(Scancode::D, Message::StartMovingRight));

    mapper.add(map_scan_released!(Scancode::W, Message::StopMovingUp));
    mapper.add(map_scan_released!(Scancode::S, Message::StopMovingLeft));
    mapper.add(map_scan_released!(Scancode::A, Message::StopMovingDown));
    mapper.add(map_scan_released!(Scancode::D, Message::StopMovingRight));

    let event_pump = sdl_context.event_pump().unwrap();

    const MAX_FPS: u32 = 60;
    let mut game = Game::new(MAX_FPS, renderer, event_pump);

    game.run(&mut state, &mapper, &mut logic, |m| *m == Message::Quit);
}
