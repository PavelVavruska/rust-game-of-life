extern crate piston_window;
extern crate rand;

mod game;
mod drawing;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use drawing::to_gui_coord_u32;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

const WINDOW_HEIGHT: usize = 100;
const WINDOW_WIDTH: usize = 100;


fn main() {
    // Prepare window settings
    let mut window_settings = WindowSettings::new("Rust Game of Life",
    [to_gui_coord_u32(WINDOW_WIDTH as i32), to_gui_coord_u32(WINDOW_HEIGHT as i32)]).exit_on_esc(true);

    // Fix vsync extension error for linux
    window_settings.set_vsync(true); 

    // Create a window
    let mut window: PistonWindow = window_settings.build().unwrap();

    // Create a world
    let mut game = Game::new( 0.5);

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game = game.key_pressed(key);            
        }

        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw_and_prepare_block_for_next_tick(&c, g);
            game.world_array = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];
            game.world_array.copy_from_slice(&game.world_array_next_tick);
            game.world_array_next_tick = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];
        });
    }
}
