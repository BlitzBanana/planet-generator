import { generateGrid } from './grid'
import { fromGrid } from './map'
import { GenerateOptions, Map, Cell } from './interfaces'

export { GenerateOptions, Map, Cell }
export const generate = async (options: GenerateOptions): Promise<Map> => {
  const grid = await generateGrid(options)
  return fromGrid(grid, options)
}
