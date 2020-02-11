<template>
  <v-container ref="container" class="pa-0" fill-height fluid>
    <canvas ref="canvas" />
  </v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import { Map, Cell } from '../generator'

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
  if (elevation < 0.19) return Colors.GRASS
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
    map: null as Map | null
  }),
  methods: {
    render() {
      if (!this.canvas) return
      if (!this.context) return
      if (!this.map) return

      const { width, height } = this.canvas
      const { cells } = this.map

      this.context.clearRect(0, 0, width, height)

      const canvasTemp = document.createElement('canvas')
      canvasTemp.width = width
      canvasTemp.height = height

      const start = window.performance.now()
      for (const cell of cells) {
        this.renderCell(canvasTemp, cell)
      }
      this.context.drawImage(canvasTemp, 0, 0)
      const end = window.performance.now()
      console.log('Rendered in ', end - start, 'ms')
    },
    renderCell(canvas: HTMLCanvasElement, cell: Cell) {
      if (!cell) return

      const context = canvas.getContext('2d', { alpha: false })
      const color = getColor(cell.elevation)

      if (!context) {
        throw new Error('Unable to retreive canvas 2d context')
      }

      context.fillStyle = color

      context.beginPath()
      context.moveTo(cell.polygon[0][0], cell.polygon[0][1])

      for (let i = 1; i < cell.polygon.length; i++) {
        context.lineTo(cell.polygon[i][0], cell.polygon[i][1])
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
      this.render()
    }

    window.onresize = handleResize
    handleResize()
  }
})
</script>
