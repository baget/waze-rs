use waze_rs::helpers::{Region, VehicleType};

use waze_rs::waze_route_calculator::WazeRouteCalculator;

#[cfg(test)]
mod tests {
    use super::*;
    use waze_rs::waze_route_calculator::WazeResult;
    use waze_rs::waze_structs::WazePath;

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
        let _ = env_logger::try_init();

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

        let mut calculator =
            WazeRouteCalculator::new(Region::US, VehicleType::CAR, false, false, false);

        calculator.with_base_url(url.as_str());

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
        let calculator =
            WazeRouteCalculator::new(Region::US, VehicleType::CAR, false, false, false);

        let results = vec![create_mock_waze_result()];

        let (route_time, route_distance) = calculator.add_up_route(&results, true, false);
        pretty_assertions::assert_eq!(route_time, 2.0); // 120 seconds / 60 = 2 minutes
        pretty_assertions::assert_eq!(route_distance, 1.0); // 1000 meters / 1000 = 1 kilometer

        let (route_time, route_distance) = calculator.add_up_route(&results, false, false);
        pretty_assertions::assert_eq!(route_time, 1.6666666666666667); // 100 seconds / 60 = 1.6667 minutes
        pretty_assertions::assert_eq!(route_distance, 1.0); // 1000 meters / 1000 = 1 kilometer
    }
}
