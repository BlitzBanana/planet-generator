extern crate serde_derive;

use crate::points::{generate_points, perturb_points};
use crate::elevation::elevate;
use crate::triangulation::{triangulate, Triangulation};

#[derive(Serialize)]
pub struct Point(pub f64, pub f64);

#[derive(Serialize)]
pub struct Map {
  /// List of all Map points (cell centers)
  pub points: Vec<Point>,

  /// List of all Map points elevation
  pub elevation: Vec<f64>,

  /// List of all Map points adjacencies (pointIndex, pointIndex, pointIndex)
  pub triangulation: Triangulation,

  /// List of all Map cells polygons
  pub cells: Vec<Vec<Point>>
}

impl Map {
  pub fn generate(
    seed: String,
    width: f64,
    height: f64,
    spacing: f64,
    chaos: f64
  ) -> Self {
    let seed_value = seed_to_u64(seed);
    let base_points = generate_points(width, height, spacing);
    let points = perturb_points(seed_value, base_points, spacing, chaos);
    let elevation = elevate(seed_value, &points, width, height);
    let triangulation = triangulate(&points);
    let cells = vec![];

    Map { points, elevation, triangulation, cells }
  }
}

fn seed_to_u64(seed: String, ) -> u64 {
  let mut value: u64 = 0;
  for i in 0..seed.len() {
    value += u64::pow(27, 256_u32 - i as u32 - 1_u32) * (1 + seed.as_bytes()[i] as u64)
  }
  value
}
