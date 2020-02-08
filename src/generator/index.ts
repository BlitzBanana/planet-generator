export * from './grid'

export const randomSeed = () =>
  Math.random()
    .toString(36)
    .substr(7)
