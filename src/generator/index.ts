const generator = import('wasm-planet-generator')

// Represents two map coords
export type Point = [number, number]

// Represents three points indexes
export type Triangle = [number, number, number]

// Represents multiple points indexes
export type Polygon = number[]

export interface Grid {
  points: Point[]
  triangles: Triangle[]
  polygons: Polygon[]
}

export interface GenerateOptions {
  seed: string
  width: number
  height: number
  space: number
  chaos: number
}

export const generate = async (options: GenerateOptions): Promise<Grid> => {
  return generator
    .then(wasm =>
      wasm.generateGrid(
        options.seed,
        options.width,
        options.height,
        options.space,
        options.chaos
      ) as Grid
    )
}

export class Utils {
  static arePointsAdjacent(grid: Grid, a: number, b: number): boolean {
    return grid.triangles.some(triangle => triangle.includes(a) && triangle.includes(b))
  }
}
