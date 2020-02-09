import * as seedrandom from 'seedrandom'
import { Delaunay, Voronoi } from 'd3-delaunay'

interface RandomPointsOptions {
  width: number
  height: number
  space: number
  chaos: number
}

const randomPoints = (
  seed: string,
  { width, height, space = 10, chaos = 0.5 }: RandomPointsOptions
): [number, number][] => {
  const random = seedrandom.default(seed)
  const countW = Math.floor(width / space)
  const countH = Math.floor(height / space)
  return Array(countW)
    .fill(0)
    .map((_, x) => {
      return Array(countH)
        .fill(0)
        .map((_, y) => {
          return [x * space + space, y * space + space] as [number, number]
        })
    })
    .flat()
    .map(
      ([x, y]) =>
        [
          x + ((random() - 0.5) / (1 / chaos)) * space,
          y + ((random() - 0.5) / (1 / chaos)) * space
        ] as [number, number]
    )
}

export interface GenerateGridOptions extends RandomPointsOptions {
  seed: string
}

export interface Grid {
  delaunay: Delaunay<Delaunay.Point>
  voronoi: Voronoi<Delaunay.Point>
}

export const generateGrid = ({
  seed,
  ...options
}: GenerateGridOptions): Grid => {
  const points = randomPoints(seed, options)
  const delaunay = Delaunay.from(points)
  const voronoi = delaunay.voronoi([0, 0, options.width, options.height])
  return { delaunay, voronoi }
}
