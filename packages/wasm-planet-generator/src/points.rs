
extern crate rand;

use rand::{Rng, SeedableRng, rngs::StdRng};
use crate::map::Point;

pub fn generate_points(width: f64, height: f64, spacing: f64) -> Vec<Point> {
  let mut points = Vec::new();
  let count_width = (width / spacing) as i32;
  let count_height = (height / spacing) as i32;

  for cols in 1..count_width {
    for rows in 1..count_height {
      let x = (cols as f64) * spacing;
      let y = (rows as f64) * spacing;
      points.push(Point (x, y));
    }
  }

  points
}

pub fn perturb_points(seed: u64, points: Vec<Point>, spacing: f64, chaos: f64) -> Vec<Point> {
  let mut random: StdRng = SeedableRng::seed_from_u64(seed);

  points
    .iter()
    .map(|point| Point (
      perturb_point_coord(point.0, spacing, chaos, random.gen::<f64>()),
      perturb_point_coord(point.1, spacing, chaos, random.gen::<f64>())
    ))
    .collect::<Vec<Point>>()
}

fn perturb_point_coord(value: f64, spacing: f64, chaos: f64, random: f64) -> f64 {
  let perturbation: f64 = random - 0.5_f64;
  let influence: f64 = perturbation / (1_f64 / chaos);
  value + influence * spacing
}
