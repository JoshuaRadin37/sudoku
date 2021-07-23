#![deny(missing_docs)]

//! A Sudoku game

#[macro_use]
extern crate bitfield;
#[macro_use]
extern crate serde;

use clap::{App, Arg};
use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, TextureSettings};
use piston::{event_loop::EventLoop, Events, EventSettings, RenderEvent, WindowSettings};

pub use game_board::*;
pub use game_board_controller::GameBoardController;
pub use game_board_view::{GameBoardView, GameBoardViewSettings};
pub use game_settings::GameSettings;

use crate::game_creator::{ByteStringLoader, GameCreator, RandomLoader};

mod game_board;
mod game_board_controller;
mod game_board_view;
mod game_settings;
pub mod game_creator;
pub mod validity;
pub mod advanced_solver;



fn main() {
    let board: GameBoard;

    let app = App::new("Sudoku")
        .arg(
            Arg::with_name("byte_string")
                .help("Uses a byte string to create a sudoku board")
                .takes_value(true)
                .short("b")
                .long("byte"),
        )
        .arg(
            Arg::with_name("random")
                .help("Randomly create a board")
                .short("r")
                .long("rand")
                .min_values(0)
                .max_values(1)
                .conflicts_with_all(&["byte_string"]),
        )
        .get_matches();

    if let Some(byte_string) = app.value_of("byte_string") {
        let loader = ByteStringLoader::from_string(byte_string);
        board = loader
            .into_game()
            .expect("Could not create game from byte string");
    } else if app.is_present("random") {
        board = match app.value_of("random") {
            Some(v) => {
                let num: u64 = v.parse().expect("Give seed is not an integer");
                RandomLoader::from_seed(num)
                    .into_game()
                    .expect("Could not create a random game")
            }
            None => RandomLoader::new()
                .into_game()
                .expect("Could not create a random game"),
        };
    } else {
        board = GameBoard::new();
    }

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
