<template>
  <v-container ref="container" class="pa-0" fill-height fluid></v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import * as PIXI from 'pixi.js'
import { Delaunay, Voronoi } from 'd3-delaunay'
import { Point, Grid } from '../generator'

const toChunks = <T extends unknown>(array: T[], size: number): T[][] => {
  const count = Math.ceil(array.length / size)
  return Array(count)
    .fill(0)
    .map((_, i) => i * size)
    .map(start => array.slice(start, start + size))
}

enum Colors {
  SEA = 0x223f6b,
  SEA_LOW = 0x69c0b8,
  SAND = 0xffdc73,
  GRASS = 0x09af12,
  ROCK = 0xaaaaaa,
  SNOW = 0xf9fafc
}

const getColor = (elevation: number): number => {
  if (elevation < -0.65) return Colors.SEA
  if (elevation < 0) return Colors.SEA_LOW
  if (elevation < 0.04) return Colors.SAND
  if (elevation < 0.22) return Colors.GRASS
  if (elevation < 0.30) return Colors.ROCK
  return Colors.SNOW
}

interface GraphicsPool {
  clear: () => void,
  reset: () => void,
  using: (fn: (g: PIXI.Graphics) => void) => void
}

const graphicsPool = (container: PIXI.Container, size = 10, limit = 10000): GraphicsPool => {
  let pool: PIXI.Graphics[] = []
  let usage = 0

  const clear = () => pool.map(g => g.clear())
  const reset = () => {
    usage = 0
    pool.map(g => g.destroy())
    pool = Array(size).fill(0).map(() => new PIXI.Graphics())
    pool.map(g => container.addChild(g))
  }
  const using = (fn: (g: PIXI.Graphics) => void) => {
    const i = Math.floor(++usage / limit)
    const exhausted = i >= size
    if (exhausted) reset()
    if (usage % limit === 0) console.log('graphics_pool', { usage })
    return fn(pool[exhausted ? 0 : i])
  }
  reset()
  return { clear, reset, using }
}

export default Vue.extend({
  name: 'Renderer',
  props: {
    grid: {
      type: Object,
      default: () => null
    }
  },
  data() {
    return {
      container: null as Element | null,
      app: null as PIXI.Application | null,
      graphics: null as GraphicsPool | null
    }
  },
  computed: {
    _grid(): Grid | null {
      return this.grid ? (this.grid as Grid) : null
    }
  },
  methods: {
    render() {
      if (!this.container) return
      if (!this.app) return
      if (!this.graphics) return
      if (!this._grid) return

      const { points, elevation } = this._grid
      const delaunay = Delaunay.from(points)
      const voronoi = delaunay.voronoi([0, 0, this.container.clientWidth, this.container.clientHeight])

      this.graphics.clear()

      // for (const [i1, i2, i3] of toChunks([...delaunay.triangles], 3)) {
      //   this.renderTriangle(this.graphics, [
      //     points[i1],
      //     points[i2],
      //     points[i3]
      //   ])
      // }

      for (const i of points.keys()) {
        const _polygon = voronoi.cellPolygon(i)
        const polygon = _polygon.map(p => [p[0], p[1]] as Point)
        this.renderCell(this.graphics, polygon, elevation[i])
      }
    },
    renderPoint(graphics: GraphicsPool, point: Point, elevation: number) {
      if (!graphics) return
      if (!point) return

      const [x, y] = point
      const color = getColor(elevation)

      graphics.using(g => {
        g.lineStyle(0)
        g.beginFill(color, 1)
        g.drawCircle(x, y, 2)
        g.endFill()
      })
    },
    renderCorner(graphics: GraphicsPool, point: Point, elevation: number) {
      if (!graphics) return
      if (!point) return

      const [x, y] = point

      graphics.using(g => {
        g.lineStyle(0)
        g.beginFill(0xffffff, 1)
        g.drawCircle(x, y, 1)
        g.endFill()
      })
    },
    renderTriangle(graphics: GraphicsPool, triangle: [Point, Point, Point]) {
      if (!graphics) return
      if (!triangle) return

      const flatPoints: number[] = triangle.flat()

      graphics.using(g => {
        g.lineStyle(1, 0xb40135, 0.3)
        g.drawPolygon(flatPoints)
      })
    },
    renderLine(graphics: GraphicsPool, line: [Point, Point]) {
      if (!graphics) return
      if (!line) return

      const [[x1, y1], [x2, y2]] = line

      graphics.using(g => {
        g.lineStyle(1, 0xb40135, 1)
        g.moveTo(x1, y1)
        g.lineTo(x2, y2)
      })
    },
    renderCell(graphics: GraphicsPool, cell: Point[], elevation: number) {
      if (!graphics) return
      if (cell.length < 3) return

      const flatPoints: number[] = cell.flat()
      const color = getColor(elevation)

      graphics.using(g => {
        g.beginFill(color)
        g.lineStyle(1, color, 1)
        g.drawPolygon(flatPoints)
        g.endFill()
      })
    }
  },
  mounted() {
    const container = (this.container = this.$refs.container as Element)
    const app = (this.app = new PIXI.Application({
      antialias: true,
      width: container.clientWidth,
      height: container.clientHeight,
      backgroundColor: 0xffffff
    }))

    this.graphics = graphicsPool(app.stage, 50, 1000)
    container.appendChild(app.view)
  },
  watch: {
    grid: {
      immediate: true,
      handler() {
        Vue.nextTick(() => this.render())
      }
    }
  }
})
</script>
