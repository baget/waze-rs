# waze-rs
Calculate actual route time and distance with Waze API - based on Python [WazeRouteCalculator](https://github.com/kovacsbalu/WazeRouteCalculator)

Uses serde and reqwest to make requests to Waze API.

![Crates.io Version](https://img.shields.io/crates/v/waze-rs)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/waze-rs)

![Crates.io License](https://img.shields.io/crates/l/waze-rs)

![GitHub Repo stars](https://img.shields.io/github/stars/baget/waze-rs)

## Build
```bash
cargo build
```

## Usage Example

Example on how to use the API (based on waze_rs_sample.rs file)

```rust
let mut wrc = WazeRouteCalculator::builder()
.set_region(Region::IL)
.set_vehicle_type(VehicleType::CAR)
.build();

wrc.set_address("New York, NY, USA", "Princeton, NJ, USA") ?;

let route = wrc.calculate_route() ?;
```

## License
GPL-3.0 (Derived work of WazeRouteCalculator)
