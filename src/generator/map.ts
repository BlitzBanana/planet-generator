import { Delaunay } from 'd3-delaunay'
import { Point, Grid, GenerateOptions, Map, Cell } from './interfaces'

export const fromGrid = (grid: Grid, options: GenerateOptions): Map => {
  const start = window.performance.now()
  const delaunay = Delaunay.from(grid.points)
  const voronoi = delaunay.voronoi([0, 0, options.width, options.height])
  const end = window.performance.now()
  console.log('Triangulated in ', end - start, 'ms')

  const cells = grid.points.map(
    (point, i): Cell => ({
      center: point,
      elevation: grid.elevation[i],
      polygon: voronoi.cellPolygon(i).map(p => p as Point)
    })
  )

  return { cells }
}
