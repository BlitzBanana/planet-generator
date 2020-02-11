declare module 'worker-loader!*' {
  class WebpackWorker extends Worker {
    constructor()
  }

  export = WebpackWorker
}

declare module '*.wasm' {
  export = WebAssembly.Module
}
