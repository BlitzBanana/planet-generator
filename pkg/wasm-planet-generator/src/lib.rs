#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate delaunator;
extern crate voronoi;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use rand::Rng;
use std::panic;
use std::convert::From;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct Point(f64, f64);

impl From<Point> for delaunator::Point {
  fn from(point: Point) -> Self {
    delaunator::Point { x: point.0, y: point.1 }
  }
}

#[derive(Serialize)]
pub struct Triangle(Point, Point, Point);

#[derive(Serialize)]
pub struct Grid {
  pub points: Vec<Point>,
  pub triangles: Vec<Triangle>,
  pub polygons: Vec<Vec<Point>>
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

fn perturb_points(_seed: String, points: Vec<Point>, spacing: f64, chaos: f64) -> Vec<Point> {
  let mut rng = rand::thread_rng();

  points
    .iter()
    .map(|point| Point (
      perturb_point_coord(point.0, spacing, chaos, rng.gen::<f64>()),
      perturb_point_coord(point.1, spacing, chaos, rng.gen::<f64>())
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
  let mut chunks = result.triangles.chunks_exact(3);
  let mut triangles = Vec::new();

  loop {
    let current = chunks.next();
    if current == None {
      break;
    }
    let chunk = current.unwrap();
    let p1x = points[chunk[0]].0;
    let p1y = points[chunk[0]].1;
    let p2x = points[chunk[1]].0;
    let p2y = points[chunk[1]].1;
    let p3x = points[chunk[2]].0;
    let p3y = points[chunk[2]].1;
    triangles.push(Triangle(
      Point(p1x, p1y),
      Point(p2x, p2y),
      Point(p3x, p3y)
    ))
  }

  triangles
}

fn voronoi(_points: &Vec<Point>) -> Vec<Vec<Point>> {
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

#[wasm_bindgen(js_name = generateGrid)]
pub fn _generate_grid(seed: String, width: f64, height: f64, spacing: f64, chaos: f64) -> JsValue {
  let grid = generate_grid(seed, width, height, spacing, chaos);

  JsValue::from_serde(&grid).unwrap()
}
