use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;
//use sdl2::Sdl;
use rand::Rng;
use std::time::Duration;


const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 1200;
const LANE_WIDTH: u32 = 65;
const INTERSECTION_SIZE: u32 = 400;
const CAR_SIZE: u32 = 50;
const SAFETY_DISTANCE: i32 = 70; // Minimum distance between cars
const MAX_SPEED: i32 = 10; // Maximum speed of the cars
const MIN_SPEED: i32 = 6; // Minimum speed of the cars

#[derive(Debug, Clone, Copy)]
struct Car {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    direction: char,
    lane: u8,
    at_intersection: bool,
}

impl Car {
    fn new(direction: char, lane: u8) -> Car {
        let (x, y, vx, vy) = match direction {
            'N' => (562 + lane as i32 * LANE_WIDTH as i32, 0, 0, 1),
            'S' => (600 - lane as i32 * LANE_WIDTH as i32, 1200, 0, -1),
            'E' => (0, 600 - lane as i32 * LANE_WIDTH as i32, 1, 0),
            'W' => (1200, 560 + lane as i32 * LANE_WIDTH as i32, -1, 0),
            _ => (0, 0, 0, 0),
        };
        Car { x, y, vx, vy, direction, lane, at_intersection: false }
    }
    
    fn update(&mut self, cars: &Vec<Car>) {
        // Check if the car is at the intersection
        if !self.at_intersection && self.x >= 600 - INTERSECTION_SIZE as i32 / 2 && self.x <= 600 + INTERSECTION_SIZE as i32 / 2 && self.y >= 600 - INTERSECTION_SIZE as i32 / 2 && self.y <= 600 + INTERSECTION_SIZE as i32 / 2 {
            self.at_intersection = true;
        }

        if self.at_intersection {
            match self.lane {
                1 => self.turn_left(),
                2 => self.go_straight(),
                3 => self.turn_right(),
                _ => (),
            }
        } else {
            self.adjust_speed(cars);
        }

        self.x += self.vx;
        self.y += self.vy;
    }


    fn turn_left(&mut self) {
        // Déterminer le comportement de virage à gauche en fonction de la direction actuelle
        match self.direction {
            'N' => {
                // Si la voiture dépasse le centre de l'intersection, tourner vers l'ouest
                if self.x > 600 {
                    self.vx = -1;
                    self.vy = 0;
                } else {
                    self.vx = 0;
                    self.vy = -1;
                }
            }
            'S' => {
                // Si la voiture dépasse le centre de l'intersection, tourner vers l'est
                if self.x < 600 {
                    self.vx = 1;
                    self.vy = 0;
                } else {
                    self.vx = 0;
                    self.vy = 1;
                }
            }
            'E' => {
                // Si la voiture dépasse le centre de l'intersection, tourner vers le nord
                if self.y < 600 {
                    self.vx = 0;
                    self.vy = -1;
                } else {
                    self.vx = 1;
                    self.vy = 0;
                }
            }
            'W' => {
                // Si la voiture dépasse le centre de l'intersection, tourner vers le sud
                if self.y > 600 {
                    self.vx = 0;
                    self.vy = 1;
                } else {
                    self.vx = -1;
                    self.vy = 0;
                }
            }
            _ => (),
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            'N' => {
                if self.x < 600 + INTERSECTION_SIZE as i32 / 2 {
                    self.vx = 1;
                    self.vy = 0;
                } else {
                    self.vx = 0;
                    self.vy = 1;
                }
            }
            'S' => {
                if self.x > 600 - INTERSECTION_SIZE as i32 / 2 {
                    self.vx = -1;
                    self.vy = 0;
                } else {
                    self.vx = 0;
                    self.vy = -1;
                }
            }
            'E' => {
                if self.y > 600 - INTERSECTION_SIZE as i32 / 2 {
                    self.vx = 0;
                    self.vy = -1;
                } else {
                    self.vx = 1;
                    self.vy = 0;
                }
            }
            'W' => {
                if self.y < 600 + INTERSECTION_SIZE as i32 / 2 {
                    self.vx = 0;
                    self.vy = 1;
                } else {
                    self.vx = -1;
                    self.vy = 0;
                }
            }
            _ => (),
        }
    }

    fn go_straight(&mut self) {
        
    }

    fn adjust_speed(&mut self, cars: &Vec<Car>) {
        for car in cars.iter() {
            if car as *const Car != self as *const Car {
                let dx = (self.x - car.x).abs();
                let dy = (self.y - car.y).abs();
                if dx < SAFETY_DISTANCE && dy < SAFETY_DISTANCE {
                    if self.vx != 0 {
                        self.vx = self.vx.signum() * MIN_SPEED;
                    }
                    if self.vy != 0 {
                        self.vy = self.vy.signum() * MIN_SPEED;
                    }
                    return;
                }
            }
        }

        if self.vx != 0 {
            self.vx = self.vx.signum() * MAX_SPEED;
        }
        if self.vy != 0 {
            self.vy = self.vy.signum() * MAX_SPEED;
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, left_texture: &Texture, right_texture: &Texture, straight_texture: &Texture) -> Result<(), String> {
        let (texture, angle) = match self.lane {
            1 => (left_texture, match self.direction {
                'N' => 90.0,
                'S' => 270.0,
                'E' => 0.0,
                'W' => 180.0,
                _ => 0.0,
            }),
            2 => (straight_texture, match self.direction {
                'N' => 90.0,
                'S' => 270.0,
                'E' => 0.0,
                'W' => 180.0,
                _ => 0.0,
            }),
            3 => (right_texture, match self.direction {
                'N' => 90.0,
                'S' => 270.0,
                'E' => 0.0,
                'W' => 180.0,
                _ => 0.0,
            }),
            _ => return Ok(()), // Fallback to no-op if lane is invalid
        };
        draw_car(canvas, texture, self.x, self.y, angle)
    }
    
}

fn generate_random_lane() -> u8 {
    rand::thread_rng().gen_range(1..=3)
}

fn generate_random_direction() -> char {
    let directions = ['N', 'S', 'E', 'W'];
    let index = rand::thread_rng().gen_range(0..directions.len());
    directions[index]
}

fn add_car(cars: &mut Vec<Car>, direction: char) {
    let lane = generate_random_lane();
    let new_car = Car::new(direction, lane);

    // Check for safety distance
    let mut safe_to_add = true;
    for car in cars.iter() {
        if ((car.x - new_car.x).abs() < SAFETY_DISTANCE) && ((car.y - new_car.y).abs() < SAFETY_DISTANCE) {
            safe_to_add = false;
            break;
        }
    }

    if safe_to_add {
        cars.push(new_car);
    }
}


fn draw_intersection(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGBA(200, 200, 200, 100)); // Transparent gray color with alpha 128
    canvas.fill_rect(Rect::new(
        (WINDOW_WIDTH / 2 - INTERSECTION_SIZE / 2) as i32,
        (WINDOW_HEIGHT / 2 - INTERSECTION_SIZE / 2) as i32,
        INTERSECTION_SIZE,
        INTERSECTION_SIZE,
    )).unwrap();
}


fn draw_car(canvas: &mut Canvas<Window>, texture: &Texture, x: i32, y: i32, angle: f64) -> Result<(), String> {
    let center = sdl2::rect::Point::new(CAR_SIZE as i32 / 2, CAR_SIZE as i32 / 2);
    canvas.copy_ex(texture, None, Some(Rect::new(x, y, CAR_SIZE, CAR_SIZE)), angle, Some(center), false, false)?;
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Smart Intersection", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture("assets/board.jpg")?;
    let left_car_texture = texture_creator.load_texture("assets/car_left.png")?;
    let right_car_texture = texture_creator.load_texture("assets/car_right.png")?;
    let straight_car_texture = texture_creator.load_texture("assets/car_straight.png")?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut cars: Vec<Car> = Vec::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => add_car(&mut cars, 'N'),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => add_car(&mut cars, 'S'),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => add_car(&mut cars, 'W'),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => add_car(&mut cars, 'E'),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let direction = generate_random_direction();
                    add_car(&mut cars, direction);
                },
                _ => {}
            }
        }

        let cars_snapshot = cars.clone();
        for car in cars.iter_mut() {
            car.update(&cars_snapshot);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw background
        canvas.copy(&background_texture, None, None)?;

        draw_intersection(&mut canvas);
        for car in &cars {
            car.draw(&mut canvas, &left_car_texture, &right_car_texture, &straight_car_texture)?;
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
