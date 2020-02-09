<template>
  <v-app>
    <v-navigation-drawer app clipped permanent>
      <v-card-title>
        <v-img class="mr-2" src="./assets/logo.png" max-width="50" />
        Planet Generator
      </v-card-title>
      <Settings :width="width" :height="height" @generated="g => (grid = g)" />
    </v-navigation-drawer>

    <v-content>
      <Renderer ref="renderer" :grid="grid" />
    </v-content>
  </v-app>
</template>

<script lang="ts">
import Vue from 'vue'
import Settings from './components/Settings.vue'
import Renderer from './components/Renderer.vue'
import { Grid } from './generator'

export default Vue.extend({
  name: 'App',
  components: {
    Settings,
    Renderer
  },
  data: () => ({
    grid: null as Grid | null,
    width: 0,
    height: 0
  }),
  methods: {
    handleResize() {
      const renderer = this.$refs.renderer as Vue
      this.width = renderer.$el.clientWidth
      this.height = renderer.$el.clientHeight
    }
  },
  created() {
    this.$vuetify.theme.dark = true
  },
  mounted() {
    Vue.nextTick(() => this.handleResize())
    window.onresize = () => this.handleResize()
  }
})
</script>
