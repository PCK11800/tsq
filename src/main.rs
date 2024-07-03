pub mod physic_obj;

use macroquad::prelude::*;

#[macroquad::main("tsq")]
async fn main() {

    let mut obj_1 = physic_obj::Actor {
        actor_type: physic_obj::ActorType::Circle,
        x_pos: screen_height() / 2.0,
        y_pos: screen_height() / 2.0,
        width: 15.0,
        height: 15.0,
        radius: 15.0,
        color: YELLOW
    };

    loop {
        clear_background(LIGHTGRAY);
        obj_1.run();
        next_frame().await
    }
}

    