
use piston_window::*;
use piston_window::types::Color;

use drawing::{draw_rectange};
use super::WINDOW_WIDTH;
use super::WINDOW_HEIGHT;


const WORLD_COLOR: Color = [0.1, 0.9, 0.1, 0.3];


pub struct Game {
    // World buffers
    pub world_array: [[bool;WINDOW_HEIGHT]; WINDOW_WIDTH],
    pub world_array_next_tick: [[bool;WINDOW_HEIGHT]; WINDOW_WIDTH]
}

impl Game {
    pub fn new(probability: f64) -> Game {
        // randomize world
        let mut temp_world = [[false; WINDOW_HEIGHT]; WINDOW_WIDTH];             
        for element in &mut temp_world.iter_mut().flat_map(|r| r.iter_mut()) {
            let prob = probability;
            if rand::random::<f64>() > prob {
                *element = true;
            }
        };

        Game {
            world_array: temp_world,
            world_array_next_tick: [[false; WINDOW_HEIGHT]; WINDOW_WIDTH]
        }
    }
    pub fn key_pressed(self, key: Key) -> Game {
        let probability = match key {
            Key::NumPad1 => 0.1,
            Key::NumPad2 => 0.2,
            Key::NumPad3 => 0.3,
            Key::NumPad4 => 0.4,
            Key::NumPad5 => 0.5,
            Key::NumPad6 => 0.6,
            Key::NumPad7 => 0.7,
            Key::NumPad8 => 0.8,
            Key::NumPad9 => 0.9,
            _ => 0.5
        };

        self.restart_game(probability)
    }

    pub fn draw_and_prepare_block_for_next_tick(&mut self, con: &Context, g: &mut G2d) {
        // Iterate over the world
        for (i, row) in self.world_array.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {

                // Draw if block is alive
                if *col == true {
                    draw_rectange(WORLD_COLOR, i as i32 +1, y as i32 +1, 1, 1, con, g);
                }

                // Prepare next tick state for the block
                if i > 0 && y > 0 && i < WINDOW_WIDTH - 1 as usize && y < WINDOW_HEIGHT - 1 as usize {  // move inside from the border by 1
                    let is_alive_self = self.world_array[i][y] as bool;
                    
                    // left neighbours
                    let is_alive_left_top = self.world_array[i-1][y-1] as u8;
                    let is_alive_left = self.world_array[i][y-1] as u8;
                    let is_alive_left_bottom = self.world_array[i+1][y-1] as u8;
                    
                    // middle neighbours
                    let is_alive_bottom = self.world_array[i+1][y] as u8;
                    let is_alive_top = self.world_array[i-1][y] as u8;
                                        
                    // right neighbours
                    let is_alive_right_bottom = self.world_array[i+1][y+1] as u8;
                    let is_alive_right = self.world_array[i][y+1] as u8;
                    let is_alive_right_top = self.world_array[i-1][y+1] as u8;
                    

                    let count_alive_neighbours = is_alive_left_top + is_alive_left + is_alive_left_bottom + is_alive_bottom + is_alive_right_bottom + is_alive_right + is_alive_right_top + is_alive_top;

                    // Condition for alive block
                    if is_alive_self && 1 < count_alive_neighbours && count_alive_neighbours < 4 {
                        self.world_array_next_tick[i][y] = true;

                    // Condition for dead block
                    } else if !is_alive_self && count_alive_neighbours == 3 {
                        self.world_array_next_tick[i][y] = true;
                    }   
                }
            }
        }
    }

    fn restart_game(self, probability : f64) -> Game {
        Game::new(probability)
    }
}
