use crate::traffic::{Direction, TrafficState};
use macroquad::prelude::*;

pub fn handle_input(traffic_state: &mut TrafficState) {
    if traffic_state.statistics.is_open {
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::P) {
            traffic_state.toggle_pause();
        }
        return;
    }

    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
        traffic_state.toggle_pause();
    }

    if is_key_pressed(KeyCode::Up) {
        traffic_state.gen_car(Direction::South);
    }

    if is_key_pressed(KeyCode::Down) {
        traffic_state.gen_car(Direction::North);
    }

    if is_key_pressed(KeyCode::Right) {
        traffic_state.gen_car(Direction::West);
    }

    if is_key_pressed(KeyCode::Left) {
        traffic_state.gen_car(Direction::East);
    }

    if is_key_pressed(KeyCode::R) {
        traffic_state.gen_car_random();
    }
}
