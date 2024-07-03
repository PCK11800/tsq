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
    pub radius: f32,
    pub color: Color,
    pub player: bool,
}

impl Actor {
    fn draw(&mut self) {
        match self.actor_type {
            ActorType::Rectangle => {
                draw_rectangle(self.x_pos, self.y_pos, self.width, self.height, self.color)
            }
            ActorType::Circle => {
                draw_circle(self.x_pos, self.y_pos, self.radius, self.color)
            }
        }
    }

    pub fn run(&mut self) {
        if self.player {
            // Player control logic
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                self.x_pos += 1.0;
            }
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                self.x_pos -= 1.0;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                self.y_pos += 1.0;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                self.y_pos -= 1.0;
            }
        } else {
            
        }

        self.draw();
    }
}