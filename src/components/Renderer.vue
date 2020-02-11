<template>
  <v-container ref="container" class="pa-0" fill-height fluid>
    <canvas ref="canvas" />
  </v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import { Delaunay } from 'd3-delaunay'
import { Point, Grid } from '../generator'

enum Colors {
  SEA = '#223f6b',
  SEA_LOW = '#69c0b8',
  SAND = '#ffdc73',
  GRASS = '#09af12',
  ROCK = '#aaaaaa',
  SNOW = '#f9fafc'
}

const getColor = (elevation: number): string => {
  if (elevation < -0.4) return Colors.SEA
  if (elevation < 0) return Colors.SEA_LOW
  if (elevation < 0.04) return Colors.SAND
  if (elevation < 0.18) return Colors.GRASS
  if (elevation < 0.34) return Colors.ROCK
  return Colors.SNOW
}

export default Vue.extend({
  name: 'Renderer',
  props: {
    grid: {
      type: Object,
      default: () => null
    }
  },
  data: () => ({
    canvas: null as HTMLCanvasElement | null,
    context: null as CanvasRenderingContext2D | null,
    map: null as Grid | null
  }),
  methods: {
    render() {
      if (!this.canvas) return
      if (!this.context) return
      if (!this.map) return

      const { width, height } = this.canvas
      const { points, elevation } = this.map
      const delaunay = Delaunay.from(points)
      const voronoi = delaunay.voronoi([0, 0, width, height])

      this.context.fillStyle = Colors.SEA
      this.context.fillRect(0, 0, width, height)

      const canvasTemp = document.createElement('canvas')
      canvasTemp.width = width
      canvasTemp.height = height

      let i = 0
      for (const cell of voronoi.cellPolygons()) {
        const polygon = cell.map(p => [p[0], p[1]] as Point)
        this.renderCell(canvasTemp, polygon, elevation[i++])
      }

      this.context.drawImage(canvasTemp, 0, 0)
    },
    renderCell(canvas: HTMLCanvasElement, cell: Point[], elevation: number) {
      if (cell.length < 3) return

      const context = canvas.getContext('2d', { alpha: false })
      const color = getColor(elevation)

      if (!context) {
        throw new Error('Unable to retreive canvas 2d context')
      }

      // if (color === Colors.SEA) return

      context.fillStyle = color

      context.beginPath()
      context.moveTo(cell[0][0], cell[0][1])

      for (let i = 1; i < cell.length; i++) {
        context.lineTo(cell[i][0], cell[i][1])
      }

      context.closePath()
      context.fill()
    }
  },
  watch: {
    grid: {
      immediate: true,
      handler(value) {
        this.map = value
      }
    },
    map: {
      immediate: true,
      handler() {
        this.render()
      }
    }
  },
  mounted() {
    const container = this.$refs.container as Element
    const canvas = (this.canvas = this.$refs.canvas as HTMLCanvasElement)
    const context = (this.context = this.canvas.getContext('2d', {
      alpha: false
    }))

    if (!context) {
      throw new Error('Unable to retreive canvas 2d context')
    }

    const handleResize = () => {
      canvas.width = container.clientWidth
      canvas.height = container.clientHeight
      context.fillStyle = '#fff'
      context.fillRect(0, 0, canvas.width, canvas.height)
      context.font = '50px Fira Code'
      context.fillStyle = '#e54f47'
      context.textAlign = 'center'
      context.fillText('Click on generate', canvas.width / 2, canvas.height / 2)
      this.render()
    }

    window.onresize = handleResize
    handleResize()
  }
})
</script>
