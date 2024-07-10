use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use rand::Rng;
use std::time::{Duration, Instant};


const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 1200;
const LANE_WIDTH: u32 = 65;
const INTERSECTION_SIZE: u32 = 370;
const CAR_SIZE: u32 = 50;
const SAFETY_DISTANCE: i32 = 100;
const MAX_SPEED: i32 = 30;
const MIN_SPEED: i32 = 10;
const THROTTLE_DURATION: Duration = Duration::from_millis(800); 
const MAX_CARS_IN_INTERSECTION: usize = 5;


#[derive(Debug, Clone, Copy)]
struct Car {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    direction: char,
    lane: u8,
    at_intersection: bool,
    has_turned: bool,
    entry_time: Instant,
}

impl Car {
    fn new(direction: char, lane: u8) -> Car {
        let (x, y, vx, vy) = match direction {
            'N' => (562 + lane as i32 * LANE_WIDTH as i32, 0, 0, 1),
            'S' => (615 - lane as i32 * LANE_WIDTH as i32, 1200, 0, -1),
            'E' => (0, 610 - lane as i32 * LANE_WIDTH as i32, 1, 0),
            'W' => (1200, 560 + lane as i32 * LANE_WIDTH as i32, -1, 0),
            _ => (0, 0, 0, 0),
        };
        Car {
            x,
            y,
            vx,
            vy,
            direction,
            lane,
            at_intersection: false,
            has_turned: false,
            entry_time: Instant::now(),
        }
    }

    fn update(&mut self, cars: &Vec<Car>) {

        if self.should_stop(cars) {
            self.vx = 0;
            self.vy = 0;
        } else {
            if self.at_intersection {
                match self.lane {
                    1 => self.turn_right(),
                    2 => self.go_straight(),
                    3 => self.turn_left(),
                    _ => (),
                }
            } else {
                self.adjust_speed(cars);
            }

            self.x += self.vx;
            self.y += self.vy;
        }

        if !self.at_intersection
            && self.x >= (590 - INTERSECTION_SIZE as i32 / 2)
            && self.x <= 590 + INTERSECTION_SIZE as i32 / 2
            && self.y >= (530 - INTERSECTION_SIZE as i32 / 2)
            && self.y <= 590 + INTERSECTION_SIZE as i32 / 2
        {
            let cars_in_intersection = cars.iter().filter(|car| car.at_intersection).count();
            if cars_in_intersection < MAX_CARS_IN_INTERSECTION {
                self.at_intersection = true;
                self.entry_time = Instant::now();
            } else {
                // Rendre la voiture stationnaire jusqu'à ce qu'il y ait de la place dans l'intersection
                self.vx = 0;
                self.vy = 0;
            }
        }

        if self.at_intersection
            && (self.x < 600 - INTERSECTION_SIZE as i32 / 2
                || self.x > 600 + INTERSECTION_SIZE as i32 / 2
                || self.y < 600 - INTERSECTION_SIZE as i32 / 2
                || self.y > 600 + INTERSECTION_SIZE as i32 / 2)
        {
            self.at_intersection = false;
            self.vx = self.vx.signum() * MAX_SPEED;
            self.vy = self.vy.signum() * MAX_SPEED;
        }
    }


    fn will_collide(&self, other: &Car, steps: i32) -> bool {
        for step in 1..=steps {
            let next_x_self = self.x + step * self.vx;
            let next_y_self = self.y + step * self.vy;
            let next_x_other = other.x + step * other.vx;
            let next_y_other = other.y + step * other.vy;

            let dx = (next_x_self - next_x_other).abs();
            let dy = (next_y_self - next_y_other).abs();

            if dx < SAFETY_DISTANCE && dy < SAFETY_DISTANCE {
                return true;
            }
        }
        false
    }

    fn adjust_speed(&mut self, cars: &Vec<Car>) {
        let mut should_slow_down = false;

        for car in cars.iter() {
            if car as *const Car != self as *const Car {
                if self.will_collide(car, 2) {
                    if self.entry_time > car.entry_time {
                        should_slow_down = true;
                        break;
                    }
                }
            }
        }

        if should_slow_down {
            if self.vx != 0 {
                self.vx = self.vx.signum() * MIN_SPEED;
            }
            if self.vy != 0 {
                self.vy = self.vy.signum() * MIN_SPEED;
            }
        } else {
            if self.vx != 0 {
                self.vx = self.vx.signum() * MAX_SPEED;
            }
            if self.vy != 0 {
                self.vy = self.vy.signum() * MAX_SPEED;
            }
        }
    }

    
    // fn should_stop(&self, cars: &Vec<Car>) -> bool {
    //     for car in cars.iter() {
    //         if car as *const Car != self as *const Car && car.direction == self.direction && car.lane == self.lane {
    //             let distance = match self.direction {
    //                 'N' | 'S' => (self.y - car.y).abs(),
    //                 'E' | 'W' => (self.x - car.x).abs(),
    //                 _ => SAFETY_DISTANCE + 1,
    //             };
                
    //             if distance < SAFETY_DISTANCE && (
    //                 (self.direction == 'N' && self.y < car.y) ||
    //                 (self.direction == 'S' && self.y > car.y) ||
    //                 (self.direction == 'E' && self.x < car.x) ||
    //                 (self.direction == 'W' && self.x > car.x)) 
    //             {
    //                 return true;
    //             }
    //         }
    //     }
    //     false
    // }

    fn should_stop(&self, cars: &Vec<Car>) -> bool {
        for car in cars.iter() {
            if car as *const Car != self as *const Car
                && car.direction == self.direction
                && car.lane == self.lane
            {
                let distance = match self.direction {
                    'N' | 'S' => (self.y - car.y).abs(),
                    'E' | 'W' => (self.x - car.x).abs(),
                    _ => SAFETY_DISTANCE + 1,
                };

                
                if distance < SAFETY_DISTANCE
                    && ((self.direction == 'N' && self.y < car.y)
                        || (self.direction == 'S' && self.y > car.y)
                        || (self.direction == 'E' && self.x < car.x)
                        || (self.direction == 'W' && self.x > car.x))
                {
                    return true;
                }
            }
        }
        false
    }


    fn turn_left(&mut self) {

        if self.has_turned {
            return;
        }

        match self.direction {
            'N' => {
                if self.x > 600 {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'E';
                    self.has_turned = true;
                } else {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                }
            }
            'S' => {
                if self.x < 600 {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'W';
                    self.has_turned = true;
                } else {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                }
            }
            'E' => {
                if self.y < 455 {
                    //println!("testE {}", self.y);
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                    self.direction = 'S';
                    self.has_turned = true;
                } else {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                }
            }
            'W' => {
                //println!("testW {}", self.y);
                if self.y > 200 {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                    self.direction = 'N';
                    self.has_turned = true;
                    // self.y = 800;
                }
                else {
                    self.vx = MAX_SPEED;
                    self.vy = MAX_SPEED;
                }
            }
            _ => (),
        }
    }

    fn turn_right(&mut self) {
        if self.has_turned {
            return;
        }

        let turn_radius: i32 = 50;
        let turn_center_x: i32 = 600;
        let turn_center_y: i32 = 600;

        match self.direction {
            'N' => {
                if self.y >= turn_center_y - turn_radius + 72 {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'W';
                    self.has_turned = true;
                } else {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                }
            }
            'S' => {
                if self.y <= turn_center_y + turn_radius - 72 {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'E';
                    self.has_turned = true;
                } else {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                }
            }
            'E' => {
                if self.x >= turn_center_x - turn_radius + 72 {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                    self.direction = 'N';
                    self.has_turned = true;
                } else {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                }
            }
            'W' => {
                if self.x <= (turn_center_x) + turn_radius - 72 {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                    self.direction = 'S';
                    self.has_turned = true;
                } else {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                }
            }
            _ => (),
        }
    }

    fn go_straight(&mut self) {
        match self.direction {
            'N' => self.vy = MAX_SPEED,
            'S' => self.vy = -MAX_SPEED,
            'E' => self.vx = MAX_SPEED,
            'W' => self.vx = -MAX_SPEED,
            _ => (),
        }
    }

    fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        left_texture: &Texture,
        right_texture: &Texture,
        straight_texture: &Texture,
    ) -> Result<(), String> {
        let (texture, angle) = match self.lane {
            1 => (
                left_texture,
                match self.direction {
                    'N' => 90.0,
                    'S' => 270.0,
                    'E' => 0.0,
                    'W' => 180.0,
                    _ => 0.0,
                },
            ),
            2 => (
                straight_texture,
                match self.direction {
                    'N' => 90.0,
                    'S' => 270.0,
                    'E' => 0.0,
                    'W' => 180.0,
                    _ => 0.0,
                },
            ),
            3 => (
                right_texture,
                match self.direction {
                    'N' => 90.0,
                    'S' => 270.0,
                    'E' => 0.0,
                    'W' => 180.0,
                    _ => 0.0,
                },
            ),
            _ => return Ok(()),
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

    let mut safe_to_add = true;
    for car in cars.iter() {
        if ((car.x - new_car.x).abs() <= SAFETY_DISTANCE)
            && ((car.y - new_car.y).abs() <= SAFETY_DISTANCE)
        {
            safe_to_add = false;
            break;
        }
    }

    if safe_to_add {
        cars.push(new_car);
    }
}


fn draw_intersection(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGBA(200, 200, 200, 100));
    canvas
        .fill_rect(Rect::new(
            ((WINDOW_WIDTH) / 2 - INTERSECTION_SIZE / 2) as i32,
            (WINDOW_HEIGHT / 2 - ((INTERSECTION_SIZE) / 2)) as i32,
            INTERSECTION_SIZE,
            INTERSECTION_SIZE,
        ))
        .unwrap();
}

fn draw_car(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    x: i32,
    y: i32,
    angle: f64,
) -> Result<(), String> {
    let center = sdl2::rect::Point::new(CAR_SIZE as i32 / 2, CAR_SIZE as i32 / 2);
    canvas.copy_ex(
        texture,
        None,
        Some(Rect::new(x, y, CAR_SIZE, CAR_SIZE)),
        angle,
        Some(center),
        false,
        false,
    )?;
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

    let mut last_add_time = Instant::now() - THROTTLE_DURATION; // Initialiser à un moment antérieur suffisant

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
                } => {
                    if cars.iter().filter(|car| car.at_intersection).count() < MAX_CARS_IN_INTERSECTION {
                        add_car(&mut cars, 'S');
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if cars.iter().filter(|car| car.at_intersection).count() < MAX_CARS_IN_INTERSECTION {
                        add_car(&mut cars, 'N');
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if cars.iter().filter(|car| car.at_intersection).count() < MAX_CARS_IN_INTERSECTION {
                        add_car(&mut cars, 'W');
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if cars.iter().filter(|car| car.at_intersection).count() < MAX_CARS_IN_INTERSECTION {
                        add_car(&mut cars, 'E');
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    if Instant::now().duration_since(last_add_time) >= THROTTLE_DURATION {
                        if cars.iter().filter(|car| car.at_intersection).count() < MAX_CARS_IN_INTERSECTION {
                            let direction = generate_random_direction();
                            add_car(&mut cars, direction);
                            last_add_time = Instant::now(); // Mettre à jour le dernier moment où une voiture a été ajoutée
                        }
                    }
                }
                _ => {}
            }
        
        }

        let cars_snapshot = cars.clone();
        for car in cars.iter_mut() {
            for other_car in &cars_snapshot {
                if car.at_intersection && other_car.at_intersection && car.will_collide(other_car, 2) {
                    if car.entry_time > other_car.entry_time {
                        car.vx = car.vx.signum() * MIN_SPEED;
                        car.vy = car.vy.signum() * MIN_SPEED;
                    } else {
                        car.vx = car.vx.signum() * MAX_SPEED;
                        car.vy = car.vy.signum() * MAX_SPEED;
                    }
                }
            }
            car.update(&cars_snapshot);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw background
        canvas.copy(&background_texture, None, None)?;

        draw_intersection(&mut canvas);
        for car in &cars {
            car.draw(
                &mut canvas,
                &left_car_texture,
                &right_car_texture,
                &straight_car_texture,
            )?;
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(32));
    }

    Ok(())
}