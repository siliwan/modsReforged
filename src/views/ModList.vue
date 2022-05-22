<template>
  <div>
      <h1>Mods</h1>
      <div class="content">
        <div class="selector">
          <label for="mod-selector">Load from</label>
          <button id="mod-selector" @click="onSelectModFolder($event)">select mod folder</button>
          <span v-if="loadingMods">Mods are being loaded...</span>
        </div>
        <hr>
        <div>
          <div class="filter">
            <label for="filter-name">Filter</label>
            <input type="text" id="filter-name" v-model="filterName">
          </div>
          <br>
          <div class="mods">
            <div  v-if="filteredMods.length > 0">
                <ul class="modlist">
                    <li v-for="(mod, index) in filteredMods" v-bind:key="mod.value.id" class="ls-none" @click="onClicked($event, mod)" :class="{ selected: mod?.isSelected ?? false}">
                        <ModEntry v-model="filteredMods[index]"></ModEntry>
                    </li>
                </ul>
            </div>
            <div v-else>
                <p v-if="filterName.trim() == ''">No mods found</p>
                <p v-else>No mods matching filter "{{ filterName }}"</p>
            </div>
          </div>
          <hr>
          <div>
            <div>
              <input type="text" :value="selectedModIds" readonly>
              <button v-on:click="copyModsToClipBoard">Copy</button>
              <transition name="copyFailed">
                  <span v-show="hasCopyFailed" class="copyFailed">Couldn't copy to clipboard.</span>
              </transition>
            </div>
            <div>
              <p>Selected mods: {{ selectedMods }}</p>
              <button v-on:click="exportMods" :disabled="selectedMods == 0">Export</button>
              <div>
                <textarea v-model="exportJson" class="rawJson" readonly :hidden="exportJson.trim() == ''"></textarea>
              </div>
            </div>
          </div>
          <div class="debug-area">
            <label for="toggleDebug">Show debug information?</label>
            <input type="checkbox" id="toggleDebug" v-model="debug">
            <div class="debug" :hidden="!debug">
                <h3>Raw Json</h3>
                <textarea v-model="modJson" class="rawJson" readonly></textarea>
                <h3>Mods:</h3>
                <li v-for="mod in mods" v-bind:key="mod.value.id" class="ls-none">
                    <input type="checkbox" disabled v-model="mod.isSelected">
                    <span>{{mod.value.name}} ({{ mod.value.id }})</span>
                </li>
            </div>
          </div>
        </div>
      </div>
  </div>
</template>

<script lang="ts">
import { ServerData } from '@/models/ServerData'
import ModEntry from '@/components/ModEntry.vue'
import { Options, Vue } from 'vue-class-component'
import { ISelectable } from '@/models/ISelectable'
import { writeText as writeTextToClipboard } from '@tauri-apps/api/clipboard'
import { confirm, message, open as openDialog } from '@tauri-apps/api/dialog'
import { EOL } from '@tauri-apps/api/os'
import { Invoke } from '@/models/Command'
import { isErr } from '@/models/Result'
import { ServerModEntry } from '@/models/ModEntry'

@Options({
  components: {
    ModEntry
  }
})
export default class ModList extends Vue {
  debug = false

  loadingMods = false;

  mods: ISelectable<ServerData>[] = []
  modJson = '[]';

  exportJson = '';

  filterName = '';

  hasCopyFailed = false;

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  async onSelectModFolder (_: MouseEvent) {
    const selectedDirectory = await openDialog({
      directory: true,
      multiple: false,
      title: 'Select folder with mods'
    })

    if (selectedDirectory == null || Array.isArray(selectedDirectory)) {
      message("Didn't select a directory!")
      return
    }

    this.loadModsFrom(selectedDirectory)
  }

  async loadModsFrom (directory: string) {
    try {
      this.loadingMods = true
      const response = await Invoke<string>('get_mod_list', {
        rawPath: directory
      })

      if (isErr(response)) {
        this.loadingMods = false
        return
      }

      console.log(EOL)

      this.modJson = response.value

      const parsedMods: ServerData[] = JSON.parse(this.modJson)

      // Clear array
      this.mods.length = 0

      parsedMods.forEach(mod => {
        const selectableMod: ISelectable<ServerData> = { isSelected: false, value: mod }
        this.mods.push(selectableMod)
      })

      this.loadingMods = false
    } catch (error) {
      console.error(error)
      this.loadingMods = false
    }
  }

  async onClicked (_: MouseEvent, mod: ISelectable<ServerData>) {
    this.exportJson = ''

    if (mod.isSelected) {
      const dependantMods = this.mods.filter(mod => !mod.isSelected && mod.value.revision.dependencies.includes(mod.value.id))

      if (dependantMods.length === 0) return

      const messageString = `${mod.value.name} has following dependencies:${EOL}${dependantMods.map(dependantMod => EOL + '• ' + dependantMod.value.name + ' (version ' + dependantMod.value.revision.version + ')')}${EOL}${EOL}Do you want to enable them as well?`

      if (await confirm(messageString, `Dependencies of ${mod.value.name}`)) {
        dependantMods.forEach(mod => { mod.isSelected = true })
      }
    }
  }

  async exportMods () {
    if (await confirm(`Do you want to export ${this.selectedMods} mod${this.selectedMods === 1 ? '' : 's'}?`, 'Export')) {
      const exportMods = this.mods.filter(mod => mod.isSelected).map(mod => new ServerModEntry(mod.value))

      this.exportJson = `"mods": ${JSON.stringify(exportMods)}`

      writeTextToClipboard(this.exportJson)
    }
  }

  __copyFailedTimeoutHandle!: number | undefined;

  copyModsToClipBoard () {
    try {
      const modIds = this.selectedModIds.join(',')
      console.info(`Copying following mods to clipboard: ${modIds}`)
      writeTextToClipboard(modIds)
    } catch (error) {
      console.error(error)
      this.hasCopyFailed = true

      clearTimeout(this.__copyFailedTimeoutHandle)
      setTimeout(() => { this.hasCopyFailed = false }, 4000)
    }
  }

  get filteredMods (): ISelectable<ServerData>[] {
    return this.mods.filter(mod => mod.value.name.toLowerCase().includes(this.filterName.toLowerCase()))
  }

  get selectedMods (): number {
    return this.mods.filter(mod => mod.isSelected).length
  }

  get selectedModIds (): string[] {
    return this.mods.filter(mod => mod.isSelected).map(selectedMod => selectedMod.value.id)
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
    label {
      margin-right: 10px;
    }
    .content {
      padding-left: 20px;
    }
    .modlist {
        display: inline-block;
        width: 50%;

        > li {
            border: 1px solid #eee;
            border-left: 4px solid lightgray;
            transition: border-left .4s ease;

            border-radius: 0 16px 16px 0 ;

            &.selected {
                border-left: 4px solid black;
            }

            > :first-child {
                width: 100%;
            }
        }
    }

    .ls-none {
        list-style: none;
    }

    .copyFailed {
        color: red;
    }

    .copyFailed-enter-active, .copyFailed-leave-active {
        transition: opacity 0.25s ease-out;
    }

    .copyFailed-enter, .copyFailed-leave-to {
        opacity: 0;
    }

    .rawJson {
        min-width: 16em;
        min-height: 18em;
    }
</style>
