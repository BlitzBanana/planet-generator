<template>
  <v-form>
    <v-container>
      <v-row>
        <v-col cols="12">
          <v-text-field
            :value="value.seed"
            @input="seed => input({ seed })"
            @click:append="randomizeSeed"
            label="Seed"
            append-icon="mdi-seed"
            required
            dense
            outlined
            hide-details
          ></v-text-field>
        </v-col>

        <v-col cols="12">
          <v-text-field
            :value="value.space"
            @input="space => input({ space: parseInt(space, 10) })"
            label="Spacing"
            type="number"
            append-icon="grid_on"
            min="10"
            max="100"
            step="1"
            required
            dense
            outlined
            hide-details
          ></v-text-field>
        </v-col>

        <v-col cols="12">
          <v-text-field
            :value="value.chaos"
            @input="chaos => input({ chaos })"
            label="Chaos"
            type="number"
            append-icon="mdi-wave"
            min="0"
            max="10"
            step="0.1"
            required
            dense
            outlined
            hide-details
          ></v-text-field>
        </v-col>
      </v-row>
    </v-container>
  </v-form>
</template>

<script lang="ts">
import Vue from 'vue'

const randomSeed = () =>
  Math.random()
    .toString(36)
    .substr(7)

export default Vue.extend({
  name: 'SettingsOptions',
  props: {
    value: {
      type: Object,
      required: true
    }
  },
  methods: {
    randomizeSeed() {
      this.input({
        seed: randomSeed()
      })
    },
    input(changes: any) {
      this.apply({
        ...this.value,
        ...changes
      })
    },
    apply(options: any) {
      this.$emit('input', options)
    }
  }
})
</script>
