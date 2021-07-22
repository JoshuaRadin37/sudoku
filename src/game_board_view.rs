//! Game board view

use crate::game_board::{CellValue, NoteStatus};
use crate::game_board_controller::NoteMode;
use crate::validity::SudokuCorrectness;
use crate::{GameBoardController, GameSettings};
use graphics::types::Color;
use graphics::{character::CharacterCache, Context, Graphics, Text};
/// Stores game board view settings.
pub struct GameBoardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of the game board along the horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color around the the houses
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius of the whole board
    pub board_edge_radius: f64,
    /// Edge radius between the houses
    pub section_edge_radius: f64,
    /// Edge radius between cells
    pub cell_edge_radius: f64,
    /// The color of the selected cell
    pub selected_cell_background_color: Color,
    /// Text color
    pub text_color: Color,
    /// Text color for denies
    pub deny_text_color: Color,
    /// Text colors for maybes
    pub maybe_text_color: Color,
    /// Preset Text Color
    pub preset_text_color: Color,
    /// Preset background cell color
    pub preset_background_color: Color,
    /// The error color highlight
    pub error_highlight: Color,
    /// Highlight a number
    pub highlight: Color,
}

impl GameBoardViewSettings {
    /// Creates new game board view settings.
    pub fn new() -> Self {
        let background_color = [0.8, 0.8, 1.0, 1.0];
        Self {
            position: [10.0; 2],
            size: 400.0,
            background_color,
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
            deny_text_color: [1.0, 0.0, 0.0, 1.0],
            maybe_text_color: [0.0, 0.0, 0.1, 1.0],
            preset_text_color: [1.0, 1.0, 1.0, 1.0],
            preset_background_color: from_rgba(94, 34, 107, 1.0),
            error_highlight: [1.0, 0.0, 0.0, 0.3],
            highlight: from_rgba(255, 249, 66, 1.0),
        }
    }
}

fn from_rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a]
}

/// Stores visual information about a game board.
pub struct GameBoardView {
    /// Stores game board view settings.
    pub settings: GameBoardViewSettings,
}

impl GameBoardView {
    /// Creates a new game board view
    pub fn new(settings: GameBoardViewSettings) -> Self {
        GameBoardView { settings }
    }

    /// Draw game board
    pub fn draw<G: Graphics, C>(
        &self,
        game_settings: &GameSettings,
        controller: &GameBoardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        use graphics::{Image, Line, Rectangle, Transformed};

        let settings = &self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];

        // Draw the background.
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw selected cell background
        if let Some(ind) = controller.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind.0 as f64 * cell_size, ind.1 as f64 * cell_size];
            let cell_rect = [
                settings.position[0] + pos[0],
                settings.position[1] + pos[1],
                cell_size,
                cell_size,
            ];
            Rectangle::new(settings.selected_cell_background_color).draw(
                cell_rect,
                &c.draw_state,
                c.transform,
                g,
            );
        }

        // Draw characters

        let text_image = Image::new_color(settings.text_color);
        let preset_text_image = Image::new_color(settings.preset_text_color);
        let highlighted_text_image = Image::new_color(settings.highlight);
        let cell_size = settings.size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
                let pos = [
                    settings.position[0] + i as f64 * cell_size + 15.0,
                    settings.position[1] + j as f64 * cell_size + 34.0,
                ];

                match controller.game_board.cell_value((i, j)) {
                    CellValue::Preset(val) => {
                        {
                            let cell_size = settings.size / 9.0;
                            let pos = [i as f64 * cell_size, j as f64 * cell_size];
                            let cell_rect = [
                                settings.position[0] + pos[0],
                                settings.position[1] + pos[1],
                                cell_size,
                                cell_size,
                            ];

                            Rectangle::new(settings.preset_background_color).draw(
                                cell_rect,
                                &c.draw_state,
                                c.transform,
                                g,
                            );
                        }

                        let char = GameBoardView::char_for_val(val);
                        if let Ok(character) = glyphs.character(34, char) {
                            let ch_x = pos[0] + character.left();
                            let ch_y = pos[1] - character.top();

                            let text_image = if Some(*val) == controller.maybe_highlighted_number {
                                highlighted_text_image.src_rect([
                                    character.atlas_offset[0],
                                    character.atlas_offset[1],
                                    character.atlas_size[0],
                                    character.atlas_size[1],
                                ])
                            } else {
                                preset_text_image.src_rect([
                                    character.atlas_offset[0],
                                    character.atlas_offset[1],
                                    character.atlas_size[0],
                                    character.atlas_size[1],
                                ])
                            };

                            text_image.draw(
                                character.texture,
                                &c.draw_state,
                                c.transform.trans(ch_x, ch_y),
                                g,
                            );
                        }
                    }
                    CellValue::Value(val) => {
                        let char = GameBoardView::char_for_val(val);
                        if let Ok(character) = glyphs.character(34, char) {
                            let ch_x = pos[0] + character.left();
                            let ch_y = pos[1] - character.top();

                            let text_image = if Some(*val) == controller.maybe_highlighted_number {
                                highlighted_text_image.src_rect([
                                    character.atlas_offset[0],
                                    character.atlas_offset[1],
                                    character.atlas_size[0],
                                    character.atlas_size[1],
                                ])
                            } else {
                                text_image.src_rect([
                                    character.atlas_offset[0],
                                    character.atlas_offset[1],
                                    character.atlas_size[0],
                                    character.atlas_size[1],
                                ])
                            };

                            text_image.draw(
                                character.texture,
                                &c.draw_state,
                                c.transform.trans(ch_x, ch_y),
                                g,
                            );
                        }
                    }
                    CellValue::Notes { status } => {
                        let mut v = 1;
                        for j in 0..3 {
                            for i in 0..3 {
                                if let Some(status) = status[j * 3 + i] {
                                    let char = GameBoardView::char_for_val(&v);
                                    if let Ok(character) = glyphs.character(12, char) {
                                        let ch_x = pos[0]
                                            + (i as f64 - 1.0) * cell_size / 3.0
                                            + character.left()
                                            + 4.0;
                                        let ch_y = pos[1] + (j as f64 - 1.0) * cell_size / 3.0
                                            - character.top()
                                            - 7.0;

                                        let mut text_image = text_image.src_rect([
                                            character.atlas_offset[0],
                                            character.atlas_offset[1],
                                            character.atlas_size[0],
                                            character.atlas_size[1],
                                        ]);

                                        text_image.color = Some(match status {
                                            NoteStatus::Maybe => {
                                                if Some(v) == controller.maybe_highlighted_number {
                                                    self.settings.highlight
                                                } else {
                                                    self.settings.maybe_text_color
                                                }
                                            }
                                            NoteStatus::Deny => self.settings.deny_text_color,
                                        });

                                        let transform = c.transform.trans(ch_x, ch_y);

                                        text_image.draw(
                                            character.texture,
                                            &c.draw_state,
                                            transform,
                                            g,
                                        );
                                    }
                                }

                                v += 1;
                            }
                        }
                    }
                    CellValue::Empty => {}
                }
            }
        }

        // Declare the format for cell and section lines.

        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);

        for i in 0..9 {
            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            let hline = [settings.position[0], y, x2, y];

            // Draw section line
            if (i % 3) == 0 {
                section_edge.draw(vline, &c.draw_state, c.transform, g);
                section_edge.draw(hline, &c.draw_state, c.transform, g);
            }
            // Draw regular line
            else {
                cell_edge.draw(vline, &c.draw_state, c.transform, g);
                cell_edge.draw(hline, &c.draw_state, c.transform, g);
            }
        }

        // Draw board edge
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );

        let mut text = Text::new(18);
        let transform = c
            .transform
            .trans(25.0, self.settings.size + self.settings.position[0] + 20.0);

        match controller.note_mode {
            NoteMode::Value => {
                text.color = self.settings.text_color;
                text.draw("Set (V)alue Mode", glyphs, &c.draw_state, transform, g)
                    .map_err(|_| "Couldn't write text to screen")
                    .unwrap();
            }
            NoteMode::Maybe => {
                text.color = self.settings.maybe_text_color;
                text.draw("Set (M)aybe Mode", glyphs, &c.draw_state, transform, g)
                    .map_err(|_| "Couldn't write text to screen")
                    .unwrap();
            }
            NoteMode::Deny => {
                text.color = self.settings.deny_text_color;
                text.draw("Set (D)eny Mode", glyphs, &c.draw_state, transform, g)
                    .map_err(|_| "Couldn't write text to screen")
                    .unwrap();
            }
        }

        let info_text = Text::new_color(self.settings.text_color, 14);
        let transform = c
            .transform
            .trans(25.0, self.settings.size + self.settings.position[0] + 40.0);
        info_text
            .draw(
                "V = value mode, M = Maybe mode, D = Deny mode",
                glyphs,
                &c.draw_state,
                transform,
                g,
            )
            .map_err(|_| "Couldn't write text to screen")
            .unwrap();

        if game_settings.show_errors {
            for (column, row) in controller.game_board.invalid_cells() {
                let pos = [column as f64 * cell_size, row as f64 * cell_size];

                let cell_rect = [
                    settings.position[0] + pos[0],
                    settings.position[1] + pos[1],
                    cell_size,
                    cell_size,
                ];

                Rectangle::new(settings.error_highlight).draw(
                    cell_rect,
                    &c.draw_state,
                    c.transform,
                    g,
                );
            }
        }
    }

    fn char_for_val(val: &u8) -> char {
        match val {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            v => panic!("Invalid value in game board: {}", v),
        }
    }
}
