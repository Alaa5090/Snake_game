use std::collections::LinkedList;
use opengl_graphics::GlGraphics;
use piston::input::*;
/// The direction the snake moves in.

#[derive(Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    START
}



#[derive(Clone)]
pub struct Snake_Piece(pub u32,pub u32);

impl Snake_Piece{
    pub fn new ( x:(u32,u32))->Snake_Piece{
        
        Snake_Piece(x.0,x.1)     
        
    }

  

}


pub struct Snake {
    gl: GlGraphics,
   pub snake_parts: LinkedList<Snake_Piece>,
   width: u32,
   pub d: Direction,
}

impl Snake {

    pub fn new(gl: GlGraphics,snake_parts: LinkedList<Snake_Piece>,width: u32,d: Direction,)->Snake{
        
        Snake{
            gl:gl,
            snake_parts:snake_parts,
            width:width,
            d:d
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares = self.snake_parts
            .iter()
            .map(|p| Snake_Piece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect::<Vec<_>>();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }
        

    
     // Move the snake if valid, otherwise returns false.
    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: Snake_Piece =
            (*self.snake_parts.front().expect("No front of snake found.")).clone();

        if (self.d == Direction::UP && new_front.1 == 0)
            || (self.d == Direction::LEFT && new_front.0 == 0)
            || (self.d == Direction::DOWN && new_front.1 == rows-1)
            || (self.d == Direction::RIGHT && new_front.0 == cols-1)
            
        {
            return false;
        }

        match self.d {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
            Direction::START=>new_front.0+=0
        }

        if !just_eaten {
            self.snake_parts.pop_back();
        }

        // Checks self collision.
        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.snake_parts.push_front(new_front);
        true
    }
    //check that snake is collide with itself
   pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|p| x == p.0 && y == p.1)
    }
}


