<template>
  <v-container ref="container" class="pa-0" fill-height fluid></v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import * as PIXI from 'pixi.js'
import { Point, Triangle, Polygon, Grid } from '../generator'

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
      graphics: null as PIXI.Graphics | null
    }
  },
  computed: {
    _grid(): Grid | null {
      return this.grid ? this.grid as Grid : null
    }
  },
  methods: {
    render() {
      if (!this.container) return
      if (!this.app) return
      if (!this.graphics) return
      if (!this._grid) return

      const { points, triangles, polygons } = this._grid

      this.graphics.clear()
      
      points.map((point: Point) => this.renderPoint(point))
      triangles.map((triangle: Triangle) => {
        return this.renderTriangle([
          points[triangle[0]],
          points[triangle[1]],
          points[triangle[2]]
        ])
      })
      polygons.map((polygon: Polygon) => {
        return this.renderPolygon(polygon.map(i => points[i]))
      })
    },
    renderPoint(point: Point) {
      if (!this.graphics) return

      const [x, y] = point

      this.graphics.lineStyle(0)
      this.graphics.beginFill(0xb40135, 0.8)
      this.graphics.drawCircle(x, y, 2)
      this.graphics.endFill()
    },
    renderTriangle(triangle: [Point, Point, Point]) {
      if (!this.graphics) return

      const [[x1, y1], [x2, y2], [x3, y3]] = triangle

      this.graphics.lineStyle(1, 0xb40135, 0.2)
      this.graphics.moveTo(x1, y1)
      this.graphics.lineTo(x2, y2)
      this.graphics.lineTo(x3, y3)
      this.graphics.closePath()
    },
    renderPolygon(polygon: Point[]) {
      if (!this.graphics) return
      if (polygon.length < 3) return

      const [first, ...points] = polygon

      this.graphics.lineStyle(1, 0xb40135)
      this.graphics.moveTo(first[0], first[1])

      for (const [x, y] of points) {
        this.graphics.lineTo(x, y)
      }

      this.graphics.closePath()
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
    const graphics = (this.graphics = new PIXI.Graphics())

    container.appendChild(app.view)
    app.stage.addChild(graphics)
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
