<template>
  <v-container ref="container" class="pa-0" fill-height></v-container>
</template>

<script lang="ts">
import Vue from 'vue'
import * as PIXI from 'pixi.js'

export default Vue.extend({
  name: 'Renderer',
  props: {
    grid: {
      type: Object,
      required: true
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
      if (this.container && this.app && this.graphics) {
        this.graphics.clear()

        if (!this.grid) {
          return
        }

        this.graphics.lineStyle(1, 0xb40135)

        this.grid.delaunay.renderPoints(this.graphics)
        this.grid.voronoi.render(this.graphics)
      }
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
