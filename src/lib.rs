//! # waze-rs
//!
//! `waze-rs` Calculate actual route time and distance with Waze API - based on Python `WazeRouteCalculator`
//!
//! Uses serde and reqwest to make requests to Waze API.

/// The main struct for the Waze API.
pub mod waze_route_calculator;

/// Structs for the Waze API.
pub mod waze_structs;

/// Helper functions and structs for the Waze API.
pub mod helpers;
