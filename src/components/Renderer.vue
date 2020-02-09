<template>
  <v-container ref="container" class="pa-0" fill-height fluid></v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import * as PIXI from 'pixi.js'

type Point = [number, number]
type Triangle = [Point, Point, Point]
type Polygon = Point[]

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
  methods: {
    render() {
      if (!this.container) return
      if (!this.app) return
      if (!this.graphics) return
      if (!this.grid) return

      this.graphics.clear()
      this.grid.points.map((p: Point) => this.renderPoint(p))
      this.grid.triangles.map((t: Triangle) => this.renderTriangle(t))
      this.grid.polygons.map((p: Polygon) => this.renderPolygon(p))
    },
    renderPoint(point: Point) {
      if (!this.graphics) return

      const [x, y] = point

      this.graphics.lineStyle(0)
      this.graphics.beginFill(0xb40135, 1)
      this.graphics.drawCircle(x, y, 1)
      this.graphics.endFill()
    },
    renderTriangle(triangle: Triangle) {
      if (!this.graphics) return

      const [first, second, third] = triangle

      this.graphics.lineStyle(1, 0xb40135)
      this.graphics.moveTo(first[0], first[1])
      this.graphics.lineTo(second[0], second[1])
      this.graphics.lineTo(third[0], third[1])
      this.graphics.closePath()
    },
    renderPolygon(polygon: Polygon) {
      if (!this.graphics) return

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
