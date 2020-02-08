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
import { randomSeed, generateGrid, GenerateGridOptions } from '../generator'

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
        chaos: 0.5
      } as GenerateGridOptions
    }
  },
  methods: {
    generate() {
      this.generating = true
      const planet = generateGrid(this.options)
      this.generating = false
      this.$emit('generated', planet)
    }
  },
  watch: {
    options: {
      immediate: true,
      deep: true,
      handler() {
        this.generate()
      }
    },
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
