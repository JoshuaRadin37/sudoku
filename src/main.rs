#![deny(missing_docs)]

//! A Sudoku game

#[macro_use]
extern crate serde;
#[macro_use]
extern crate bitfield;

use piston::{
    event_loop::EventLoop, EventSettings, Events, RenderEvent, Size, UpdateEvent, Window,
    WindowSettings,
};

mod game_board;
pub use game_board::*;

mod game_board_controller;
pub use game_board_controller::GameBoardController;

mod game_board_view;
pub use game_board_view::{GameBoardView, GameBoardViewSettings};

mod game_settings;
use crate::game_creator::{GameCreator, JSONLoader, ByteStringLoader};
pub use game_settings::GameSettings;
use glutin_window::{GlutinWindow, OpenGL};
use graphics::CharacterCache;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use std::env::args;
use clap::{App, Arg};

pub mod game_creator;
pub mod validity;

mod ui;

fn main() {




    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not make window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let json_loader = JSONLoader::from_string(
        r#"
        [
            {
                "x": 0,
                "y": 0,
                "val": 1
            }
        ]
    "#,
    );

    let board: GameBoard;

    let app = App::new("Sudoku")
        .arg(
            Arg::with_name("byte_string")
                .help("Uses a byte string to create a sudoku board")
                .takes_value(true)
                .short("b")
                .long("byte")
        )
        .get_matches();

    if let Some(byte_string) = app.value_of("byte_string") {
        let loader = ByteStringLoader::from_string(byte_string);
        board = loader.into_game().expect("Could not create game from byte string");
    } else {
        board = GameBoard::new();
    }


    let game_settings = GameSettings::new();

    let mut controller = GameBoardController::new(board);
    let game_view_settings = GameBoardViewSettings::new();
    let board_view = GameBoardView::new(game_view_settings);

    while let Some(event) = events.next(&mut window) {
        controller.event(
            board_view.settings.position,
            board_view.settings.size,
            &event,
        );
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);

                board_view.draw(&game_settings, &controller, glyph_cache, &c, g);
            })
        }
    }

    println!("{}", settings.get_exit_on_esc());
}
