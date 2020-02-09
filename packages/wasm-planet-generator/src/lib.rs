#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate delaunator;
extern crate voronoi;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::panic;
use std::convert::From;
use rand::{Rng, SeedableRng, rngs::StdRng};


// API methods
#[wasm_bindgen(js_name = generateGrid)]
pub fn _generate_grid(seed: String, width: f64, height: f64, spacing: f64, chaos: f64) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let grid = generate_grid(seed, width, height, spacing, chaos);
  JsValue::from_serde(&grid).unwrap()
}


// API types
#[derive(Clone, Serialize)]
pub struct Point(f64, f64);

impl From<&Point> for delaunator::Point {
  fn from(point: &Point) -> Self {
    delaunator::Point { x: point.0, y: point.1 }
  }
}

#[derive(Serialize)]
pub struct Triangle(usize, usize, usize);

#[derive(Serialize)]
pub struct Grid {
  pub points: Vec<Point>,
  pub triangles: Vec<Triangle>,
  pub polygons: Vec<Vec<usize>>
}

fn generate_points(width: f64, height: f64, spacing: f64) -> Vec<Point> {
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

fn rand_from_seed(seed: String) -> StdRng {
  let mut value: u64 = 0;
  
  for i in 0..seed.len() {
    value += u64::pow(27, 256_u32 - i as u32 - 1_u32) * (1 + seed.as_bytes()[i] as u64)
  }

  let rand: StdRng = SeedableRng::seed_from_u64(value);

  rand
}

fn perturb_points(seed: String, points: Vec<Point>, spacing: f64, chaos: f64) -> Vec<Point> {
  let mut random = rand_from_seed(seed);

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

fn delaunay(points: &Vec<Point>) -> Vec<Triangle> {
  let delaunay_points = points
    .iter()
    .map(|p| delaunator::Point::from(p))
    .collect::<Vec<delaunator::Point>>();
  let result = delaunator::triangulate(&delaunay_points).unwrap();
  let triangles = result.triangles
    .chunks_exact(3)
    .map(|chunk| Triangle(chunk[0], chunk[1], chunk[2]))
    .collect::<Vec<Triangle>>();

  triangles
}

fn voronoi(_points: &Vec<Point>) -> Vec<Vec<usize>> {
  vec![]
}

fn generate_grid(seed: String, width: f64, height: f64, spacing: f64, chaos: f64) -> Grid {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  
  let base_points = generate_points(width, height, spacing);
  let points = perturb_points(seed, base_points, spacing, chaos);
  let triangles = delaunay(&points);
  let polygons = voronoi(&points);

  Grid { points, triangles, polygons }
}
