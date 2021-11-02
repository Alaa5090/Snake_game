use opengl_graphics::GlGraphics;
use crate::snake::{Direction, Snake};
use crate::food::Food;
use piston::input::*;
pub struct Game {
    gl: GlGraphics,
    square_width: u32,
    food: Food,
    snake:Snake,
    cols:u32,
    rows:u32,
    just_eaten:bool,
    pub score: u32,
}

impl Game {
    pub fn new(gl:GlGraphics,width: u32,food:Food,snake:Snake,cols:u32,rows:u32)->Self
    {
        Self{
        square_width:width,
        gl:gl,
        food:food,
        snake:snake,
        cols,
        rows,just_eaten:false,
        score:0
    }}


    
    
    
    pub fn render(&mut self, args: &RenderArgs) {
            use graphics;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(GREEN, gl);
        });


        //self.food.render(&mut self.gl, args, self.square_width);
        self.food.render(&mut self.gl, args, self.square_width);

        //render snake

        self.snake.render(args);


    }

    
    
    pub fn update(&mut self, args: &UpdateArgs) -> bool {
        
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            use rand::Rng;
            use rand::thread_rng;
            // try my luck
            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0, self.cols);
                let new_y = r.gen_range(0, self.rows);
                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food { x: new_x, y: new_y };
                    break;
                }
            }
        }

        true

    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.d.clone();
        self.snake.d = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        };
    }
}
