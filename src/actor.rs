use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum ActorType {
    Circle,
    Rectangle,
}

pub struct Actor {
    pub actor_type: ActorType,
    pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub color: Color,
    pub logic: Box<dyn Fn(&mut f32, &mut f32, &mut f32) -> ()>
}

impl Actor {
    fn draw(&mut self) {
        match self.actor_type {
            ActorType::Rectangle => {
                draw_rectangle_ex(self.x_pos, self.y_pos, self.width, self.height, DrawRectangleParams {
                    offset: Vec2::new(0.5, 0.5),
                    rotation: self.rotation,
                    color: self.color
                })
            }
            ActorType::Circle => {
                draw_circle(self.x_pos, self.y_pos, self.width, self.color)
            }
        }
    }

    pub fn run(&mut self) {
        (self.logic)(&mut self.x_pos, &mut self.y_pos, &mut self.rotation);
        self.draw();
    }
}