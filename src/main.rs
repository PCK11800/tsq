use macroquad::prelude::*;

#[macroquad::main("tsq")]
async fn main() {
    let mut circle_1 = CircleBody {
        x_pos: screen_height() / 2.0,
        y_pos: screen_height() / 2.0,
        radius: 15.0,
        color: YELLOW
    };

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) {
            circle_1.x_pos += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            circle_1.x_pos -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            circle_1.y_pos += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            circle_1.y_pos -= 1.0;
        }

        circle_1.draw();
        next_frame().await
    }
}

struct CircleBody {
    x_pos: f32,
    y_pos: f32,
    radius: f32,
    color: Color
}

impl CircleBody {
    fn draw(&self) {
        draw_circle(self.x_pos, self.y_pos, self.radius, self.color)
    }
}
    