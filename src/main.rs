use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const LANE_WIDTH: u32 = 75;
const INTERSECTION_SIZE: u32 = 340;
const CAR_SIZE: u32 = 30;
const SAFETY_DISTANCE: i32 = 100;
const MAX_SPEED: i32 = 5;
const MIN_SPEED: i32 = 1;
const THROTTLE_DURATION: Duration = Duration::from_millis(800);
const MAX_CARS_IN_INTERSECTION: usize = 4;

#[derive(Debug, Clone, Copy)]
struct Car {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    original_vx: i32,
    original_vy: i32,
    direction: char,
    lane: u8,
    at_intersection: bool,
    has_turned: bool,
    has_stopped: bool,
    entry_time: Instant,
}

impl Car {
    fn new(direction: char, lane: u8) -> Car {
        let (x, y, vx, vy) = match direction {
            'N' => (350 + lane as i32 * LANE_WIDTH as i32, 800, 0, -1),
            'S' => (425 - lane as i32 * LANE_WIDTH as i32, 0, 0, 1),
            'W' => (800, 430 - lane as i32 * LANE_WIDTH as i32, -1, 0),
            'E' => (0, 330 + lane as i32 * LANE_WIDTH as i32, 1,0),
            _ => (0, 0, 0, 0),
        };
        Car {
            x,
            y,
            vx,
            vy,
            original_vx: vx,
            original_vy: vy,
            direction,
            lane,
            at_intersection: false,
            has_turned: false,
            has_stopped: false,
            entry_time: Instant::now(),
        }
    }

    fn update(&mut self, cars: &Vec<Car>) {
        if !self.at_intersection && self.lane == 3 {
            self.turn_right();
        }
    
        if self.should_stop(cars) {
            self.vx = 0;
            self.vy = 0;
            self.has_stopped = true;
        } else {
            if self.at_intersection {
                match self.lane {
                    1 => self.turn_left(),
                    2 => self.go_straight(),
                    _ => (),
                }
            } else {
                self.adjust_speed(cars);
            }
        }
    
        if !self.at_intersection
            && self.x >= (390 - INTERSECTION_SIZE as i32 / 2)
            && self.x <= 390 + INTERSECTION_SIZE as i32 / 2
            && self.y >= (350 - INTERSECTION_SIZE as i32 / 2)
            && self.y <= 350 + INTERSECTION_SIZE as i32 / 2
        {
            let cars_in_intersection = cars.iter().filter(|car| car.at_intersection).count();
            if cars_in_intersection < MAX_CARS_IN_INTERSECTION {
                self.at_intersection = true;
                self.entry_time = Instant::now();
            } else {
                self.vx = 0;
                self.vy = 0;
                self.has_stopped = true;
            }
        }
    
        if self.at_intersection
            && (self.x < 400 - INTERSECTION_SIZE as i32 / 2
                || self.x > 400 + INTERSECTION_SIZE as i32 / 2
                || self.y < 400 - INTERSECTION_SIZE as i32 / 2
                || self.y > 400 + INTERSECTION_SIZE as i32 / 2)
        {
            self.at_intersection = false;
            self.vx = self.original_vx;
            self.vy = self.original_vy;
        }
        self.check_and_resume_if_stopped(cars);
    
        self.x += self.vx;
        self.y += self.vy;
    }
    
    fn will_collide(&self, other: &Car, steps: i32) -> bool {
        let self_rect = Rect::new(self.x, self.y, CAR_SIZE as u32, CAR_SIZE as u32);
        let other_rect = Rect::new(other.x, other.y, CAR_SIZE as u32, CAR_SIZE as u32);
    
        // Vérification des collisions à la position actuelle
        if self_rect.has_intersection(other_rect) {
            return true;
        }
    
        // Vérification des collisions pour chaque étape de mouvement
        for step in 1..=steps {
            let next_x_self = self.x + step * self.vx;
            let next_y_self = self.y + step * self.vy;
            let next_x_other = other.x + step * other.vx;
            let next_y_other = other.y + step * other.vy;
    
            let self_rect_next = Rect::new(next_x_self, next_y_self, CAR_SIZE as u32, CAR_SIZE as u32);
            let other_rect_next = Rect::new(next_x_other, next_y_other, CAR_SIZE as u32, CAR_SIZE as u32);
    
            if self_rect_next.has_intersection(other_rect_next) {
                return true;
            }
    
            // // Vérification des collisions sur les côtés
            // if self_rect_next.left() < other_rect_next.right() &&
            //    self_rect_next.right() > other_rect_next.left() &&
            //    self_rect_next.top() < other_rect_next.bottom() &&
            //    self_rect_next.bottom() > other_rect_next.top() {
            //     return true;
            // }
        }
        false
    }
    
    fn adjust_speed(&mut self, cars: &Vec<Car>) {
        let mut should_slow_down = false;

        for car in cars.iter() {
            if car as *const Car != self as *const Car {
                if self.will_collide(car, 8) {
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

    fn should_stop(&self, cars: &Vec<Car>) -> bool {
        for car in cars.iter() {
            if car as *const Car != self as *const Car
                && car.direction == self.direction
                && car.lane == self.lane
            {
                let distance = match self.direction {
                    'N' | 'S' => (self.y - car.y).abs(),
                    'E' | 'W' => (self.x - car.x).abs(),
                    _ => SAFETY_DISTANCE,
                };

                if distance <= SAFETY_DISTANCE
                    && ((self.direction == 'N' && self.y < car.y)
                        || (self.direction == 'S' && self.y > car.y)
                        || (self.direction == 'E' && self.x < car.x)
                        || (self.direction == 'W' && self.x > car.x))
                        && !self.has_turned
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
                if self.y < 360 && self.y > 320 {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                    self.direction = 'W';
                    self.has_turned = true;
                } else {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            'S' => {
                if self.y > 420 {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'E';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            'E' => {
                if self.x > 420 && self.x < 460 {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                    self.direction = 'N';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            'W' => {
                if self.x > 320 && self.x < 360 {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                    self.direction = 'S';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            _ => (),
        }
    }

    fn turn_right(&mut self) {
        if self.has_turned {
            return;
        }

        match self.direction {
            'N' => {
                if self.y >= 500 && self.y <= 570 {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'E';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            'S' => {
                if self.y >= 208 && self.y < 700 {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                    self.direction = 'W';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            'E' => {
                if self.x >= 202 {
                    self.vx = 0;
                    self.vy = MAX_SPEED;
                    self.direction = 'S';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = MAX_SPEED;
                    self.vy = 0;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            'W' => {
                if self.x <= 570 {
                    self.vx = 0;
                    self.vy = -MAX_SPEED;
                    self.direction = 'N';
                    self.has_turned = true;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                } else {
                    self.vx = -MAX_SPEED;
                    self.vy = 0;
                    self.original_vx = self.vx;
                    self.original_vy = self.vy;
                }
            }
            _ => (),
        }
    }

    fn go_straight(&mut self) {
        match self.direction {
            'S' => self.vy = MAX_SPEED,
            'N' => self.vy = -MAX_SPEED,
            'W' => self.vx = -MAX_SPEED,
            'E' => self.vx = MAX_SPEED,
            _ => (),
        }
    }

    fn check_and_resume_if_stopped(&mut self, cars: &Vec<Car>) {
        if !self.has_stopped {
            return;
        }

        let mut imminent_collision = false;
        for car in cars.iter() {
            if car as *const Car != self as *const Car && self.will_collide(car, 8) {
                imminent_collision = true;
                break;
            }
        }

        if !imminent_collision {
            println!("Resuming car at position ({}, {})", self.x, self.y);
            self.vx = self.original_vx;
            self.vy = self.original_vy;
            self.has_stopped = false;
        } else {
            println!("Cannot resume car at position ({}, {}): collision imminent", self.x, self.y);
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
                    'N' => 270.0,
                    'S' => 90.0,
                    'E' => 0.0,
                    'W' => 180.0,
                    _ => 0.0,
                },
            ),
            2 => (
                straight_texture,
                match self.direction {
                    'N' => 270.0,
                    'S' => 90.0,
                    'E' => 0.0,
                    'W' => 180.0,
                    _ => 0.0,
                },
            ),
            3 => (
                right_texture,
                match self.direction {
                    'N' => 270.0,
                    'S' => 90.0,
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
    if cars.iter().filter(|car| car.at_intersection).count() < MAX_CARS_IN_INTERSECTION {
        let lane = generate_random_lane();
        let new_car = Car::new(direction, lane);

        let mut safe_to_add = true;
        for car in cars.iter() {
            if ((car.x - new_car.x).abs() <= SAFETY_DISTANCE + 20)
                && ((car.y - new_car.y).abs() <= SAFETY_DISTANCE + 20)
            {
                safe_to_add = false;
                break;
            }
        }

        if safe_to_add {
            cars.push(new_car);
        }
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

fn resume_stopped_cars(cars: &mut Vec<Car>) {
    let mut stopped_cars: Vec<usize> = cars.iter()
        .enumerate()
        .filter_map(|(i, car)| if car.has_stopped { Some(i) } else { None })
        .collect();

    stopped_cars.sort_by_key(|&i| cars[i].entry_time);

    for &i in &stopped_cars {
        let mut imminent_collision = false;
        for (j, other_car) in cars.iter().enumerate() {
            if i != j {
                let will_collide = cars[i].will_collide(other_car, 10);
                if will_collide {
                    imminent_collision = true;
                    break;
                }
            }
        }

        if !imminent_collision {
            cars[i].vx = cars[i].original_vx;
            cars[i].vy = cars[i].original_vy;
            cars[i].has_stopped = false;
        }
    }
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
    let background_texture = texture_creator.load_texture("assets/board1.png")?;
    let left_car_texture = texture_creator.load_texture("assets/car_left.png")?;
    let right_car_texture = texture_creator.load_texture("assets/car_right.png")?;
    let straight_car_texture = texture_creator.load_texture("assets/car_straight.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut cars: Vec<Car> = Vec::new();

    let mut last_add_time = Instant::now() - THROTTLE_DURATION;

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
                    keycode: Some(Keycode::Right),
                    ..
                } => add_car(&mut cars, 'E'),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => add_car(&mut cars, 'W'),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    if Instant::now().duration_since(last_add_time) >= THROTTLE_DURATION {
                        let direction = generate_random_direction();
                        add_car(&mut cars, direction);
                        last_add_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        resume_stopped_cars(&mut cars);

        let cars_snapshot = cars.clone();
        for car in cars.iter_mut() {
            for other_car in &cars_snapshot {
                if car.at_intersection
                    && other_car.at_intersection
                    && car.will_collide(other_car, 8)
                {
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
