
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

pub use Snake_game::snake::{Snake,Snake_Piece,Direction};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;
use graphics::Rectangle;




use Snake_game::{game::Game,food::Food};
fn main() {
let opengl = OpenGL::V3_2;

 const COLS: u32 = 30;
 const ROWS: u32 = 20;
const SQUARE_WIDTH: u32 = 20;

let WIDTH = COLS * SQUARE_WIDTH;
let HEIGHT = ROWS * SQUARE_WIDTH;

let mut window: GlutinWindow = WindowSettings::new("Snake Game", [WIDTH, HEIGHT])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

//intantiate objects

let mut game = Game::new(
     GlGraphics::new(opengl),
     SQUARE_WIDTH,
     Food::new(1, 1),
     Snake::new(
         GlGraphics::new(opengl),
         LinkedList::from_iter((vec![Snake_Piece::new((COLS / 2, ROWS / 2))]).into_iter()),
         SQUARE_WIDTH,
         Direction::START,
     ),COLS,ROWS
    );

   


 //end of space

    let mut events = Events::new(EventSettings::new()).ups(2);
    while let Some(e) = events.next(&mut window) {
        if let Some(render_values) = e.render_args() {
       //call object methods here
      
            game.render(&render_values);

            //end of space to call obj methods
        }

        if let Some(update_args )= e.update_args() {
            //call object methods here
           
                 game.update(&update_args);
     
                 //end of space to call obj methods
             

    }


    if let Some(pressed_args )= e.press_args() {
        //call object methods here
       
             game.pressed(&pressed_args);
 
             //end of space to call obj methods
         

}

}

}
