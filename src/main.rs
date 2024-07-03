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
        logic: Box::new(player_ai)
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
        logic: Box::new(|x_pos, y_pos| basic_ai(x_pos, y_pos)), 
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

fn player_ai(x_pos: &mut f32, y_pos: &mut f32) {
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        *x_pos += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        *x_pos -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        *y_pos += 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        *y_pos -= 1.0;
    }
}

fn basic_ai(x_pos: &mut f32, y_pos: &mut f32) {
    // *x_pos += 1.0;
    // *y_pos += 1.0;
}
    