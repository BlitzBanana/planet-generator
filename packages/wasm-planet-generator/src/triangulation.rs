

extern crate serde_derive;
extern crate delaunator;

use std::convert::From;
use crate::map::Point;

impl From<&Point> for delaunator::Point {
  fn from(point: &Point) -> Self {
    delaunator::Point { x: point.0, y: point.1 }
  }
}

#[derive(Serialize)]
pub struct Triangulation {
  pub triangles: Vec<usize>,
  pub halfedges: Vec<usize>,
  pub hull: Vec<usize>
}

pub fn triangulate(points: &Vec<Point>) -> Triangulation {
  let delaunay_points = points
    .iter()
    .map(|p| delaunator::Point::from(p))
    .collect::<Vec<delaunator::Point>>();
  
  let result = delaunator::triangulate(&delaunay_points).unwrap();

  Triangulation {
    triangles: result.triangles,
    halfedges: result.halfedges,
    hull: result.hull
  }
}
