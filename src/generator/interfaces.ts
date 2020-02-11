export type Point = [number, number]

export interface Grid {
  points: Point[]
  elevation: number[]
  triangulation: {
    triangles: number[]
    halfedges: number[]
    hull: number[]
  }
  subdivision: {
    corners: Point[]
    vectors: [number, number][]
    cells: number[][]
  }
}

export interface GenerateOptions {
  seed: string
  width: number
  height: number
  space: number
  chaos: number
}

export interface Cell {
  center: Point
  polygon: Point[]
  elevation: number
}

export interface Map {
  cells: Cell[]
}
