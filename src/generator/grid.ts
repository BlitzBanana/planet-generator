import { Grid, GenerateOptions } from './interfaces'

// import PromiseWorker from 'promise-worker'
// import GridWorker from 'worker-loader!./grid-worker'

// const worker = new PromiseWorker(new GridWorker())

const generator = import('wasm-planet-generator')

export const generateGrid = async (options: GenerateOptions): Promise<Grid> => {
  const start = window.performance.now()
  // const grid = await worker.postMessage(options).then(grid => grid as Grid)
  const grid = await generator.then(wasm => {
    return wasm.generateGrid(
      options.seed,
      options.width,
      options.height,
      options.space,
      options.chaos
    )
  })
  const end = window.performance.now()
  console.log('Generated in ', end - start, 'ms')
  return grid
}
