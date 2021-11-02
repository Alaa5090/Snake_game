use crate::snake::Snake;
use graphics::types::Rectangle;
use piston::input::*;
use opengl_graphics::GlGraphics;
pub struct Food {
    pub x: u32,
    pub y: u32,
}

impl Food {
    pub fn new( x: u32, y: u32)->Self{
        Self{x:x,y:y}
        }
    // Return true if snake ate food this update
   pub fn update(&mut self, s: &Snake) -> bool {
    let front = s.snake_parts.front().unwrap();
    if front.0 == self.x && front.1 == self.y {
        true
    } else {
        false
    }
    }
    fn grow_food(&mut self,length:u32,width: u32)->Vec<Rectangle>{
        let x = self.x * width;
        let y = self.y * width;
        
        let mut food:Vec<Rectangle>=vec![];
        for i in 0..length{
            let square = graphics::rectangle::square(x as f64, (x+(20*i)) as f64, width as f64);
            food.push(square);
        }
        food
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        use graphics;

        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BlUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let food=self.grow_food(1, width);
        
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            let mut color=BlUE;
            for square in food{
                graphics::rectangle(color, square, transform, gl);
                if color==BlUE{color=BLACK}
                else{color=BlUE};
                
            }
            
            
            
        });
    }
}
