use crate::helpers::{Region, VehicleType};
pub use crate::waze_structs::{Bound, Coordinates, WazeAddressAnswer, WazeResult};
use reqwest::header::{HeaderMap, HeaderValue, REFERER, USER_AGENT};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error};

#[derive(Error, Debug)]
pub enum WazeRouteCalculatorError {
    #[error("Failed to get coordinates")]
    FailedToGetCoordinates,

    #[error("Failed to get route")]
    FailedToGetRoute,

    #[error("Waze API error: {0}")]
    WazeApiError(String),

    #[error("Networking error")]
    NetworkError(#[from] reqwest::Error),

    #[error("Serde error")]
    SerializationError(#[from] serde_json::Error),

    #[error("Unknown error")]
    UnknownError,
}

/// A builder for the `WazeRouteCalculator` struct.
#[derive(Debug)]
pub struct WazeRouteCalculatorBuilder {
    pub region: Region,
    pub vehicle_type: VehicleType,
    pub avoid_toll_roads: bool,
    pub avoid_subscription_roads: bool,
    pub avoid_ferries: bool,
    pub base_url: String,
}

impl WazeRouteCalculatorBuilder {
    /// Sets the region for the route calculator.
    ///
    /// # Arguments
    ///
    /// * `region` - The region to set.
    ///
    /// # Returns
    ///
    /// The updated `WazeRouteCalculatorBuilder` instance.
    pub fn set_region(mut self, region: Region) -> Self {
        debug!("region: {:?}", region);
        self.region = region;
        self
    }

    /// Sets the vehicle type for the route calculator.
    ///
    /// # Arguments
    ///
    /// * `vehicle_type` - The vehicle type to set.
    ///
    /// # Returns
    ///
    /// The updated `WazeRouteCalculatorBuilder` instance.
    pub fn set_vehicle_type(mut self, vehicle_type: VehicleType) -> Self {
        debug!("vehicle_type: {:?}", vehicle_type);
        self.vehicle_type = vehicle_type;
        self
    }

    /// Sets whether to avoid subscription roads.
    ///
    /// # Arguments
    ///
    /// * `value` - A boolean indicating whether to avoid subscription roads.
    ///
    /// # Returns
    ///
    /// The updated `WazeRouteCalculatorBuilder` instance.
    pub fn set_avoid_subscription_roads(mut self, value: bool) -> Self {
        self.avoid_subscription_roads = value;
        self
    }

    /// Sets whether to avoid toll roads.
    ///
    /// # Arguments
    ///
    /// * `value` - A boolean indicating whether to avoid toll roads.
    ///
    /// # Returns
    ///
    /// The updated `WazeRouteCalculatorBuilder` instance.
    pub fn set_avoid_toll_roads(mut self, value: bool) -> Self {
        self.avoid_toll_roads = value;
        self
    }

    /// Sets whether to avoid ferries.
    ///
    /// # Arguments
    ///
    /// * `value` - A boolean indicating whether to avoid ferries.
    ///
    /// # Returns
    ///
    /// The updated `WazeRouteCalculatorBuilder` instance.
    pub fn set_avoid_ferries(mut self, value: bool) -> Self {
        self.avoid_ferries = value;
        self
    }

    /// Sets the base URL for the route calculator.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL to set.
    ///
    /// # Returns
    ///
    /// The updated `WazeRouteCalculatorBuilder` instance.
    pub fn set_base_url(mut self, base_url: &str) -> Self {
        debug!("Base URL: {}", base_url);
        self.base_url = base_url.to_string();
        self
    }

    /// Builds the `WazeRouteCalculator` instance.
    ///
    /// # Returns
    ///
    /// A `WazeRouteCalculator` instance with the configured options.
    pub fn build(self) -> WazeRouteCalculator {
        let mut route_options = HashMap::new();
        route_options.insert("AVOID_TRAILS".to_string(), "t".to_string());
        route_options.insert(
            "AVOID_TOLL_ROADS".to_string(),
            if self.avoid_toll_roads {
                "t".to_string()
            } else {
                "f".to_string()
            },
        );
        route_options.insert(
            "AVOID_FERRIES".to_string(),
            if self.avoid_ferries {
                "t".to_string()
            } else {
                "f".to_string()
            },
        );

        debug!("Route options: {:?}", route_options);

        WazeRouteCalculator {
            region: self.region,
            vehicle_type: self.vehicle_type,
            start_coords: None,
            end_coords: None,
            avoid_subscription_roads: self.avoid_subscription_roads,
            route_options,
            base_url: self.base_url,
        }
    }
}

/// A struct representing a Waze route calculator.
#[derive(Debug)]
pub struct WazeRouteCalculator {
    pub region: Region,
    pub vehicle_type: VehicleType,
    pub start_coords: Option<Coordinates>,
    pub end_coords: Option<Coordinates>,
    route_options: HashMap<String, String>,
    avoid_subscription_roads: bool,
    base_url: String,
}

impl WazeRouteCalculator {
    /// Creates a new `WazeRouteCalculatorBuilder` with default values.
    ///
    /// # Returns
    ///
    /// A `WazeRouteCalculatorBuilder` instance with default settings.
    pub fn builder() -> WazeRouteCalculatorBuilder {
        WazeRouteCalculatorBuilder {
            region: Region::EU,
            vehicle_type: VehicleType::CAR,
            avoid_subscription_roads: false,
            avoid_toll_roads: false,
            avoid_ferries: false,
            base_url: WazeRouteCalculator::WAZE_URL.to_string(),
        }
    }

    /// Sets the start and end coordinates based on the provided addresses.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The starting address.
    /// * `end_address` - The ending address.
    ///
    /// # Returns
    ///
    /// A result containing a mutable reference to the `WazeRouteCalculator` instance or an error.
    pub fn with_address(
        &mut self,
        start_address: &str,
        end_address: &str,
    ) -> Result<&mut Self, WazeRouteCalculatorError> {
        self.start_coords = Some(self.address_to_coords(start_address)?);
        self.end_coords = Some(self.address_to_coords(end_address)?);

        debug!(
            "Start coordinates: {}, {}",
            self.start_coords.unwrap().latitude,
            self.start_coords.unwrap().longitude
        );

        debug!(
            "End coordinates: {}, {}",
            self.end_coords.unwrap().latitude,
            self.end_coords.unwrap().longitude
        );

        Ok(self)
    }

    /// Constructs the headers required for the HTTP request.
    ///
    /// # Returns
    ///
    /// A `HeaderMap` containing the necessary headers.
    fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
        headers.insert(
            REFERER,
            HeaderValue::from_str(self.base_url.as_str()).unwrap(),
        );
        headers
    }

    /// Converts an address to coordinates.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to convert.
    ///
    /// # Returns
    ///
    /// A result containing the coordinates or an error.
    pub fn address_to_coords(
        &self,
        address: &str,
    ) -> Result<Coordinates, WazeRouteCalculatorError> {
        let base_coords = WazeRouteCalculator::BASE_COORDS[self.region as usize].1;
        let get_cord_path = WazeRouteCalculator::COORD_SERVERS[self.region as usize].1;

        let url = format!("{}{}", self.base_url, get_cord_path);
        debug!("URL: {}", url);

        let lon_binding = base_coords.lon.to_string();
        let lat_binding = base_coords.lat.to_string();
        let params = [
            ("q", address),
            ("lang", "eng"),
            ("lang", "eng"),
            ("origin", "livemap"),
            ("lon", lon_binding.as_str()),
            ("lat", lat_binding.as_str()),
        ];

        debug!("params: {:?}", params);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .query(&params)
            .headers(self.construct_headers())
            .send()?;

        debug!("Response: {:?}", response);

        if response.status().is_success() {
            let address_answer = response.json::<Value>()?;

            if !address_answer.is_array() {
                error!("Address answer is not an array");
                return Err(WazeRouteCalculatorError::FailedToGetCoordinates);
            }

            for answer in address_answer.as_array().unwrap() {
                if answer.get("city").is_some() {
                    let mut coords = Coordinates {
                        latitude: answer["location"]["lat"].as_f64().unwrap_or_default(),
                        longitude: answer["location"]["lon"].as_f64().unwrap_or_default(),
                        bound: None,
                    };

                    if let Some(bound) = answer.get("bounds") {
                        if bound.is_null() {
                            return Ok(coords);
                        }

                        let top = bound.get("top").unwrap().as_f64().unwrap_or_default();
                        let bottom = bound.get("bottom").unwrap().as_f64().unwrap_or_default();
                        let left = bound.get("left").unwrap().as_f64().unwrap_or_default();
                        let right = bound.get("right").unwrap().as_f64().unwrap_or_default();

                        let new_bound = Bound {
                            top: top.max(bottom),
                            bottom: top.min(bottom),
                            left: left.min(right),
                            right: left.max(right),
                        };

                        coords.bound = Some(new_bound);
                    }
                    return Ok(coords);
                }
            }
            error!("Address answer not an array");
            Err(WazeRouteCalculatorError::FailedToGetCoordinates)
        } else {
            error!("Address answer with status: {}", response.status());
            Err(WazeRouteCalculatorError::FailedToGetCoordinates)
        }
    }

    fn get_route(&self) -> Result<Vec<WazeResult>, WazeRouteCalculatorError> {
        let routing_server = WazeRouteCalculator::ROUTING_SERVERS[self.region as usize].1;
        let from_str = format!(
            "x:{} y:{}",
            self.start_coords.unwrap().longitude,
            self.start_coords.unwrap().latitude
        );
        let to_str = format!(
            "x:{} y:{}",
            self.end_coords.unwrap().longitude,
            self.end_coords.unwrap().latitude
        );
        let options_str = self
            .route_options
            .iter()
            .map(|(opt, value)| format!("{}:{}", opt, value))
            .collect::<Vec<_>>()
            .join(",");

        //TODO: Handle nPaths and time_delta
        let mut params = vec![
            ("from", from_str.as_str()),
            ("to", to_str.as_str()),
            ("at", "0"),
            ("returnJSON", "true"),
            ("returnGeometries", "true"),
            ("returnInstructions", "true"),
            ("timeout", "60000"),
            ("nPaths", "1"),
            ("options", &options_str),
        ];

        if self.vehicle_type != VehicleType::CAR {
            params.push(("vehicleType", self.vehicle_type.to_string()));
        }

        if !self.avoid_subscription_roads {
            params.push(("subscription", "*"));
        }

        debug!("params: {:?}", params);

        let url = format!("{}{}", self.base_url, routing_server);
        debug!("URL: {}", url);

        let client = reqwest::blocking::Client::new();
        let query_res = client
            .get(url)
            .query(&params)
            .headers(self.construct_headers())
            .send()?;

        debug!("Response: {:?}", query_res);

        if query_res.status().is_success() {
            let waze_route_answer: Value = query_res.json()?;

            if waze_route_answer.get("error").is_none() {
                if let Some(response) = waze_route_answer.get("response") {
                    if let Some(alternatives) = response.get("alternatives") {
                        return Ok(alternatives
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| serde_json::from_value(x.clone()).unwrap())
                            .collect());
                    }

                    if let Some(results) = response.get("results") {
                        Ok(serde_json::from_value(results.clone())?)
                    } else {
                        error!("'results' field not found");
                        Err(WazeRouteCalculatorError::FailedToGetRoute)
                    }
                } else {
                    error!("'response' field not found");
                    Err(WazeRouteCalculatorError::FailedToGetRoute)
                }
            } else {
                let error = waze_route_answer["error"].as_str().unwrap().to_string();
                error!("Waze Error: {}", error);
                Err(WazeRouteCalculatorError::WazeApiError(error))
            }
        } else {
            Err(WazeRouteCalculatorError::FailedToGetRoute)
        }
    }

    /// Calculates the route time and distance based on the provided results.
    ///
    /// # Arguments
    ///
    /// * `results` - A slice of `WazeResult` containing the route segments.
    /// * `real_time` - A boolean indicating whether to use real-time data.
    /// * `stop_at_bounds` - A boolean indicating whether to stop at bounds.
    ///
    /// # Returns
    ///
    /// A tuple containing the route time in minutes and the route distance in kilometers.
    pub fn add_up_route(
        &self,
        results: &[WazeResult],
        real_time: bool,
        stop_at_bounds: bool,
    ) -> (f64, f64) {
        let start_bounds = self
            .start_coords
            .unwrap_or_default()
            .bound
            .unwrap_or_default();
        let end_bounds = self
            .end_coords
            .unwrap_or_default()
            .bound
            .unwrap_or_default();

        /// Checks if a target value is between a minimum and maximum value.
        ///
        /// # Arguments
        ///
        /// * `target` - The target value to check.
        /// * `min` - The minimum value.
        /// * `max` - The maximum value.
        ///
        /// # Returns
        ///
        /// A boolean indicating whether the target is between the min and max values.
        fn between(target: f64, min: f64, max: f64) -> bool {
            target > min && target < max
        }

        let (time, distance) = results
            .iter()
            .fold((0, 0), |(mut time, mut distance), segment| {
                if stop_at_bounds {
                    if let Some(path) = &segment.path {
                        let x = path.x;
                        let y = path.y;
                        if (between(x, start_bounds.left, start_bounds.right)
                            || between(x, end_bounds.left, end_bounds.right))
                            && (between(y, start_bounds.bottom, start_bounds.top)
                                || between(y, end_bounds.bottom, end_bounds.top))
                        {
                            return (time, distance);
                        }
                    }
                }

                if real_time {
                    time += segment.cross_time;
                } else {
                    time += segment.cross_time_without_real_time;
                }
                distance += segment.length;

                (time, distance)
            });

        let route_time = time as f64 / 60.0;
        let route_distance = distance as f64 / 1000.0;
        (route_time, route_distance)
    }

    /// Calculates the best route info by calling `get_route` and using `add_up_route` to calculate the route time and distance.
    ///
    /// # Returns
    ///
    /// A result containing a tuple with the route time in minutes and the route distance in kilometers, or an error.
    pub fn calculate_route(&self) -> Result<(std::time::Duration, f64), WazeRouteCalculatorError> {
        let route = self.get_route()?;

        let (route_time, route_distance) = self.add_up_route(&route, true, false);

        debug!("Route time: {}", route_time);
        debug!("Route distance: {}", route_distance);

        Ok((
            std::time::Duration::from_secs(route_time as u64 * 60),
            route_distance,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waze_route_calculator::WazeResult;
    use crate::waze_structs::WazePath;

    #[test]
    fn test_vehicle_type_to_string() {
        let vehicle_type = VehicleType::TAXI;
        pretty_assertions::assert_eq!(vehicle_type.to_string(), "TAXI");

        let vehicle_type = VehicleType::MOTORCYCLE;
        pretty_assertions::assert_eq!(vehicle_type.to_string(), "MOTORCYCLE");

        let vehicle_type = VehicleType::CAR;
        pretty_assertions::assert_eq!(vehicle_type.to_string(), "");
    }

    #[test]
    fn test_address_to_coords() {
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 1234,
            ..Default::default()
        };
        let mut server = mockito::Server::new_with_opts(opts);

        // Use one of these addresses to configure your client
        let url = server.url() + "/";

        let mock  = server.mock("GET", "/SearchServer/mozi")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"bounds":null,"businessName":"Address","city":"Detroit","countryName":"United States","location":{"lat":12.34,"lon":56.78},"name":"Address","number":"5512","provider":"waze","segmentId":23621747,"state":null,"stateName":"Michigan","street":"Beaubien St","streetId":1601804},{"bounds":null,"businessName":"Address","city":"Dayton","countryName":"United States","location":{"lat":39.763481974,"lon":-84.184411227},"name":"Address","number":"418","provider":"waze","segmentId":0,"state":"Ohio","stateName":"Ohio","street":"E 1st St","streetId":2417464},{"bounds":null,"businessName":null,"city":"Aberdeen Proving Ground","countryName":"United States","location":{"lat":39.47930145263672,"lon":-76.17952728271484},"name":"Test Hwy, Aberdeen Proving Ground, MD","number":null,"provider":"waze","segmentId":-1,"state":"MD","stateName":"Maryland","street":"Test Hwy","streetId":61686875},{"bounds":null,"businessName":null,"city":null,"countryName":"United States","location":{"lat":39.46147155761719,"lon":-76.19284057617188},"name":"Test Hwy, MD","number":null,"provider":"waze","segmentId":-1,"state":"MD","stateName":"Maryland","street":"Test Hwy","streetId":4704325},{"bounds":null,"businessName":null,"city":"Union","countryName":"United States","location":{"lat":40.686431884765625,"lon":-74.26087188720703},"name":"Andress Ter, Union, NJ","number":null,"provider":"waze","segmentId":-1,"state":"NJ","stateName":"New Jersey","street":"Andress Ter","streetId":2011162}]"#)
            .match_query(mockito::Matcher::Any)
            .create();

        let calculator = WazeRouteCalculator::builder()
            .set_region(Region::US)
            .set_vehicle_type(VehicleType::CAR)
            .set_base_url(url.as_str())
            .build();

        let result = calculator.address_to_coords("Test Address");

        mock.assert();

        pretty_assertions::assert_eq!(result.is_ok(), true);
        let coords = result.unwrap();
        pretty_assertions::assert_eq!(coords.latitude, 12.34);
        pretty_assertions::assert_eq!(coords.longitude, 56.78);
    }

    fn create_mock_waze_result() -> WazeResult {
        WazeResult {
            path: Some(WazePath {
                segment_id: 0,
                node_id: 0,
                x: 0.0,
                y: 0.0,
                direction: false,
            }),
            cross_time: 120,
            cross_time_without_real_time: 100,
            length: 1000,
            ..Default::default()
        }
    }

    #[test]
    fn test_add_up_route() {
        let calculator = WazeRouteCalculator::builder()
            .set_region(Region::US)
            .set_vehicle_type(VehicleType::CAR)
            .build();

        let results = vec![create_mock_waze_result()];

        let (route_time, route_distance) = calculator.add_up_route(&results, true, false);
        pretty_assertions::assert_eq!(route_time, 2.0); // 120 seconds / 60 = 2 minutes
        pretty_assertions::assert_eq!(route_distance, 1.0); // 1000 meters / 1000 = 1 kilometer

        let (route_time, route_distance) = calculator.add_up_route(&results, false, false);
        pretty_assertions::assert_eq!(route_time, 1.6666666666666667); // 100 seconds / 60 = 1.6667 minutes
        pretty_assertions::assert_eq!(route_distance, 1.0); // 1000 meters / 1000 = 1 kilometer
    }
}
