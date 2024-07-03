pub mod actor;

use macroquad::prelude::*;

#[macroquad::main("tsq")]
async fn main() {

    let mut actor_list: Vec<actor::Actor> = Vec::new();
    let mut obj_1 = actor::Actor {
        actor_type: actor::ActorType::Rectangle,
        x_pos: screen_height() / 2.0,
        y_pos: screen_height() / 2.0,
        width: 90.0,
        height: 30.0,
        rotation: 0.0,
        color: YELLOW,
        logic: Box::new(player_ai)
    };

    let mut obj_2 = actor::Actor {
        actor_type: actor::ActorType::Rectangle,
        x_pos: screen_height() / 2.0 + 40.0,
        y_pos: screen_height() / 2.0 + 40.0,
        width: 90.0,
        height: 30.0,
        rotation: 0.0,
        color: RED,
        logic: Box::new(basic_ai), 
    };

    actor_list.push(obj_1);
    actor_list.push(obj_2);

    loop {
        clear_background(LIGHTGRAY);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);

        // Update Positions
        for actor in actor_list.iter_mut() {
            actor.run();
        }

        // Collision Handling
        let len = actor_list.len();
        for i in 0..len {
            for j in (i+1)..len {
                let (left, right) = actor_list.split_at_mut(j);
                resolve_collision(&mut left[i], &mut right[0]);
            }
        }

        next_frame().await
    }
}

fn resolve_collision(actor_1: &mut actor::Actor, actor_2: &mut actor::Actor) {
    if actor_1.actor_type == actor::ActorType::Rectangle && actor_2.actor_type == actor::ActorType::Rectangle {
        let left_1 = actor_1.x_pos;
        let right_1 = actor_1.x_pos + actor_1.width;
        let top_1 = actor_1.y_pos;
        let bottom_1 = actor_1.y_pos + actor_1.height;

        let left_2 = actor_2.x_pos;
        let right_2 = actor_2.x_pos + actor_2.width;
        let top_2 = actor_2.y_pos;
        let bottom_2 = actor_2.y_pos + actor_2.height;
    }
}

fn player_ai(x_pos: &mut f32, y_pos: &mut f32, rotation: &mut f32) {
    // Define movement speed and rotation speed
    let movement_speed = 4.0;
    let rotation_speed = 0.0349; // approximately 2 degrees in radians

    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        *rotation -= rotation_speed;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        *rotation += rotation_speed;
    }

    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        *x_pos += rotation.cos() * movement_speed;
        *y_pos += rotation.sin() * movement_speed;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        *x_pos -= rotation.cos() * movement_speed;
        *y_pos -= rotation.sin() * movement_speed;
    }
}

fn basic_ai(x_pos: &mut f32, y_pos: &mut f32, rotation: &mut f32) {
    // *x_pos += 1.0;
    // *y_pos += 1.0;
}
    