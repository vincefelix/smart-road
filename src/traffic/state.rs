use crate::traffic::{Direction, Line, Path};
use crate::app::Statistics;
use macroquad::prelude::get_time;
use macroquad::rand::ChooseRandom;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TrafficState {
    pub lines: [Line; 4],
    pub statistics: Statistics,
    pub pause_time: f64,
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            lines: [
                Line::new(Direction::North),
                Line::new(Direction::East),
                Line::new(Direction::South),
                Line::new(Direction::West),
            ],
            statistics: Statistics::default(),
            pause_time: get_time(),
        }
    }

    pub fn toggle_pause(&mut self) {
        self.statistics.is_open = !self.statistics.is_open;

        if self.statistics.is_open {
            self.pause_time = get_time();
        } else {
            let pause_duration = get_time() - self.pause_time;
            self.lines.iter_mut().for_each(|line| {
                line.path_cars.iter_mut().for_each(|cars| {
                    cars.iter_mut().for_each(|car| {
                        car.start_time += pause_duration;
                    });
                });
            });
        }
    }

    pub fn update(&mut self) {
        let traffic_state = self.clone();

        for line in &mut self.lines {
            line.update(&traffic_state);
        }

        self.statistics.update(&traffic_state);
    }

    pub fn gen_car(&mut self, coming_from: Direction) {
        self.statistics.car_count += 1;
        let line = &mut self.lines[coming_from as usize];

        if let Some(path) = line.get_free_paths().choose() {
            line.gen_car(path.clone());
        }
    }

    pub fn gen_car_random(&mut self) {
        let available_line_paths: Vec<(usize, Rc<Path>)> = self.lines
            .iter()
            .enumerate()
            .flat_map(|(line_index, line)| {
                line.get_free_paths()
                    .iter()
                    .map(move |path| (line_index, path.clone()))
                    .collect::<Vec<_>>()
            })
            .collect();

        if let Some((line_index, path)) = available_line_paths.choose() {
            self.lines[*line_index].gen_car(path.clone());
            self.statistics.car_count += 1;
        }
    }
}
