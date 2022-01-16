#![feature(int_abs_diff)]
use core::convert::{From};
use core::time::Duration;

extern crate simplelog;
use simplelog::*;

use storm::graphics::*;
use storm::event::*;
use storm::*;
use storm::math::Transform;


use storm::audio::*;
mod game;
mod shaders;

use game::*;
use shaders::*;

static FONT: &[u8] = include_bytes!("resources/gomarice_game_continue_02.ttf");
static FIREBALL: &[u8] = include_bytes!("resources/fireball_main.png");
static SOUND: &[u8] = include_bytes!("resources/makoto.flac");

const X_SCALE : u16 = 4;
const Y_SCALE : u16 = 4;
#[derive(Eq, PartialEq)]
pub enum AppState {
    Play,
    Pause,
    Debug
}
#[derive(Eq, PartialEq)]
pub enum GameState {
    Loading,
    Menu,
    Game
}

fn main() {
    //I am initing this logger to avoid an error on mac
    let _ = SimpleLogger::init(LevelFilter::Warn, Config::default());
    // Create the engine context and describe the window.
    start(
        WindowSettings {
            title: String::from("Storm Fighting"),
            display_mode: DisplayMode::Windowed {
                width: 1280 * 2,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        },
        run,
    );
}



fn run() -> impl FnMut(Event) {




    wait_periodic(Some(Duration::from_secs_f32(1.0 / 60.0)));

    let mut game_state = GameState::Menu;

    let mut game: Option<Game> = None;
    let mut menu = Some(Menu::new());

    let boop = Sound::from_flac(SOUND).unwrap();
    let s = boop.play(0.1, 0.0);

    move |event| match event {
        //Process input
        //A subset of these keys go to the character with character_1.key_down or character_1.key_up
        //Others are used to control game flow, like Pause, or Debug
        Event::CloseRequested => request_stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => request_stop(),
            _ => {
                if game.is_some() {
                    game.as_mut().unwrap().key_down(key);
                }
            }
        },
        Event::CursorPressed { button:_, physical_pos:_, normalized_pos } => {
            match game_state {
                GameState::Menu => {
                    menu.as_mut().unwrap().mouse_down(normalized_pos);
                },
                 _ => {

                 }
            }
        },
        Event::CursorReleased { button:_, physical_pos:_, normalized_pos } => {
            match game_state {
                GameState::Menu => {
                    menu.as_mut().unwrap().mouse_up(normalized_pos);
                },
                 _ => {
                     
                 }
            }
        }
        Event::KeyReleased(key) => match key {
            KeyboardButton::Escape => request_stop(),
            _ => {
                if game.is_some() {
                    game.as_mut().unwrap().key_up(key);
                }
            }
        },
        
        Event::Update(_delta) => {
            match game_state {
                GameState::Game => {
                    if game.is_some() {
                        game.as_mut().unwrap().update();
                    }
                },
                GameState::Menu => {
                    let desired_game_state = menu.as_mut().unwrap().tick();
                    if desired_game_state != GameState::Menu {
                        game_state = desired_game_state;
                        game = Some(Game::default());
                    }
                },
                _ => {
                }
            }
        }
        _ => {}
    }
}
