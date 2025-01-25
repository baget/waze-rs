use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub bound: Option<Bound>,
}

impl From<WazeAddressCoordinates> for Coordinates {
    fn from(coord: WazeAddressCoordinates) -> Self {
        Coordinates {
            latitude: coord.lat,
            longitude: coord.lon,
            bound: None,
        }
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bound {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

/// Type alias for a vector of `WazeAddress` structs.
pub type WazeAddressAnswer = Vec<WazeAddress>;

/// Represents an address in the Waze system.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WazeAddress {
    /// The bounds of the address.
    pub bounds: Option<Bound>,
    /// The business name associated with the address.
    pub business_name: Option<String>,
    /// The city of the address.
    pub city: Option<String>,
    /// The country name of the address.
    pub country_name: Option<String>,
    /// The geographical coordinates of the address.
    pub location: WazeAddressCoordinates,
    /// The name of the address.
    pub name: String,
    /// The number of the address.
    pub number: Option<String>,
    /// The provider of the address.
    pub provider: Option<String>,
    /// The segment ID of the address.
    pub segment_id: i64,
    /// The state of the address, represented as a JSON value.
    pub state: Option<Value>,
    /// The state name of the address.
    pub state_name: Option<String>,
    /// The street of the address.
    pub street: Option<String>,
    /// The street ID of the address.
    pub street_id: i64,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WazeAddressCoordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WazeResult {
    pub path: Option<WazePath>,
    pub length: i64,
    pub cross_time: i64,
    pub cross_time_without_real_time: i64,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WazePath {
    pub segment_id: i64,
    pub node_id: i64,
    pub x: f64,
    pub y: f64,
    pub direction: bool,
}
