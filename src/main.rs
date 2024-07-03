pub mod actor;

use macroquad::prelude::*;

#[macroquad::main("tsq")]
async fn main() {

    let mut actor_list: Vec<actor::Actor> = Vec::new();
    let mut obj_1 = actor::Actor {
        actor_type: actor::ActorType::Circle,
        x_pos: screen_height() / 2.0,
        y_pos: screen_height() / 2.0,
        width: 15.0,
        height: 15.0,
        radius: 15.0,
        color: YELLOW,
        player: true,
    };

    let mut obj_2 = actor::Actor {
        actor_type: actor::ActorType::Circle,
        x_pos: screen_height() / 2.0 + 40.0,
        y_pos: screen_height() / 2.0 + 40.0,
        width: 15.0,
        height: 15.0,
        radius: 15.0,
        color: RED,
        player: false,
    };

    actor_list.push(obj_1);
    actor_list.push(obj_2);

    loop {
        clear_background(LIGHTGRAY);
        for actor in actor_list.iter_mut() {
            actor.run();
        }
        next_frame().await
    }
}

    