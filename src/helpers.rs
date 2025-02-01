use crate::waze_route_calculator::WazeRouteCalculator;
use crate::waze_structs::WazeAddressCoordinates;

/// Enum representing the region of the Waze server.
#[derive(Copy, Clone, Debug)]
pub enum Region {
    /// United States
    US = 0,

    /// Europe
    EU,

    /// Israel
    IL,

    /// Australia
    AU,
}

/// Enum representing the vehicle type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VehicleType {
    /// Regular Car
    CAR,

    /// Taxi
    TAXI,

    /// Motorcycle
    MOTORCYCLE,
}

impl VehicleType {
    /// Converts the `VehicleType` enum to a string slice.
    ///
    /// # Returns
    /// * A string slice representing the vehicle type.
    pub fn to_string(&self) -> &str {
        match self {
            VehicleType::CAR => "",
            VehicleType::TAXI => "TAXI",
            VehicleType::MOTORCYCLE => "MOTORCYCLE",
        }
    }
}
impl WazeRouteCalculator {
    /// Base Waze URL
    pub const WAZE_URL: &'static str = "https://www.waze.com/";

    /// Base Coordinates for each region
    pub(crate) const BASE_COORDS: [(Region, WazeAddressCoordinates); 4] = [
        (
            Region::US,
            WazeAddressCoordinates {
                lat: 40.713,
                lon: -74.006,
            },
        ),
        (
            Region::EU,
            WazeAddressCoordinates {
                lat: 47.498,
                lon: 19.040,
            },
        ),
        (
            Region::IL,
            WazeAddressCoordinates {
                lat: 31.768,
                lon: 35.214,
            },
        ),
        (
            Region::AU,
            WazeAddressCoordinates {
                lat: -35.281,
                lon: 149.128,
            },
        ),
    ];

    /// Waze  servers path for each region
    pub(crate) const COORD_SERVERS: [(Region, &'static str); 4] = [
        (Region::US, "SearchServer/mozi"),
        (Region::EU, "row-SearchServer/mozi"),
        (Region::IL, "il-SearchServer/mozi"),
        (Region::AU, "row-SearchServer/mozi"),
    ];

    /// Waze routing servers path for each region
    pub(crate) const ROUTING_SERVERS: [(Region, &'static str); 4] = [
        (Region::US, "RoutingManager/routingRequest"),
        (Region::EU, "row-RoutingManager/routingRequest"),
        (Region::IL, "il-RoutingManager/routingRequest"),
        (Region::AU, "row-RoutingManager/routingRequest"),
    ];
}
