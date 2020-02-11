import registerPromiseWorker from 'promise-worker/register'

const generator = import('wasm-planet-generator')

registerPromiseWorker(options => {
  return generator.then(wasm =>
    wasm.generateGrid(
      options.seed,
      options.width,
      options.height,
      options.space,
      options.chaos
    )
  )
})
