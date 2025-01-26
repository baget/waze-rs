use tracing_subscriber;
use waze_rs::helpers::{Region, VehicleType};
use waze_rs::waze_route_calculator::WazeRouteCalculator;

fn main() {
    tracing_subscriber::fmt::init();

    println!("Hello Waze-rs!");

    let mut wrc = WazeRouteCalculator::new(Region::IL, VehicleType::CAR, false, false, false);

    wrc.with_address("Maale Adummim", "Tel Aviv, Israel")
        .expect("with_address() failed");

    println!("{:?}", wrc);
    let res = wrc
        .calculate_route()
        .expect("calculate_route_info() failed");
    println!("{:?}", res);
}
