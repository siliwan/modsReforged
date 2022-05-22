<template>
  <div v-on:click="onSelect($event)" class="mod">
      <div class="common">
            <p>{{ mod?.name }}</p>
            <p>Version {{ mod.revision.version }}</p>
      </div>
      <div class="downloadStatus">
        <p>Downloaded: {{ mod.revision?.downloaded ? 'Yes' : 'No' }}</p>
      </div>
      <div class="dependencies" v-if="mod.revision.dependencies.length > 0">
        Dependencies:
        <ul>
          <li v-for="dependency in mod.revision.dependencies" v-bind:key="`${dependency}`">
            {{ dependency }}
          </li>
        </ul>
      </div>
      <div class="scenarios" v-if="mod.revision.scenarios.length > 0">
        Scenarios:
        <ul>
          <li v-for="scenario in mod.revision.scenarios" v-bind:key="`${scenario.gameId}${scenario.name}`">
            {{ scenario.name }}
          </li>
        </ul>
      </div>
  </div>
</template>

<script lang="ts">
import { ServerData } from '@/models/ServerData'
import { ISelectable } from '@/models/ISelectable'
import { Options, Vue } from 'vue-class-component'

@Options({
  props: {
    modelValue: { isSelected: Boolean, value: ServerData }
  }
})
export default class ModEntry extends Vue {
  modelValue!: ISelectable<ServerData>;

  get mod () { return this.modelValue.value }

  get isSelected () { return this.modelValue.isSelected }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  onSelect (_: MouseEvent) {
    this.modelValue.isSelected = !this.modelValue.isSelected
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
    .mod {
        display: inline-flex;
        flex-flow: row wrap;
        align-content: flex-start;
        gap: 10px;
    }
</style>
