use macroquad::window::Conf;

pub const WINDOW_SIZE: i32 = 700;
pub const ROAD_WIDTH: f32 = 270.0;

pub const CAR_WIDTH: f32 = 70.0;
pub const CAR_LENGTH: f32 = 60.0;

pub const CAR_PADDING: f32 = (ROAD_WIDTH / 2.0 - CAR_WIDTH) / 4.0;
pub const CAR_SAFE_DISTANCE: f32 = 100.0;
pub const MAX_SPEED: f32 = 4.0;

pub const MIN_SPEED: f32 = 2.0;

pub const CAR_ACCELERATION: f32 = 0.1;
pub const CAR_DECELERATION: f32 = 0.5;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "smart-road".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}


pub const STRAIGHT_LENGTH: f32 = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 4.0;
