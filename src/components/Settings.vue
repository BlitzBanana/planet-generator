<template>
  <v-card color="transparent" flat>
    <SettingsOptions v-model="options" />
    <v-card-actions>
      <v-spacer />
      <v-btn small :loading="generating" @click="generate">Generate</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import Vue from 'vue'
import SettingsOptions from './SettingsOptions.vue'
import { GenerateOptions, generate } from '../generator'

const randomSeed = () =>
  Math.random()
    .toString(36)
    .substr(7)

export default Vue.extend({
  name: 'Settings',
  components: { SettingsOptions },
  props: {
    width: {
      type: Number,
      required: true
    },
    height: {
      type: Number,
      required: true
    }
  },
  data() {
    return {
      generating: false,
      options: {
        seed: randomSeed(),
        width: 0,
        height: 0,
        space: 30,
        chaos: 0.5,
        noise: 1
      } as GenerateOptions
    }
  },
  methods: {
    async generate() {
      this.generating = true
      const start = window.performance.now()
      const grid = await generate(this.options)
      const end = window.performance.now()
      this.generating = false
      this.$emit('generated', grid)
      console.log('Generated in ', end - start, 'ms')
    }
  },
  watch: {
    height: {
      immediate: true,
      handler(height) {
        this.options.height = height
      }
    },
    width: {
      immediate: true,
      handler(width) {
        this.options.width = width
      }
    }
  }
})
</script>
