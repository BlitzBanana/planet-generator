const generator = import('wasm-planet-generator')

// Represents two map coords
export type Point = [number, number]

export interface Grid {
  points: Point[]
  elevation: number[]
  cells: Point[][]
  triangulation: {
    triangles: number[],
    halfedges: number[],
    hull: number[]
  }
}

export interface GenerateOptions {
  seed: string
  width: number
  height: number
  space: number
  chaos: number
}

export const generate = async (options: GenerateOptions): Promise<Grid> => {
  return generator.then(
    wasm =>
      wasm.generateGrid(
        options.seed,
        options.width,
        options.height,
        options.space,
        options.chaos
      ) as Grid
  )
}
