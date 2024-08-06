mod app;
mod constant;
mod draw;
mod traffic;

use app::*;
use constant::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut smart_road = App::new().await;

    smart_road.run().await;
}
