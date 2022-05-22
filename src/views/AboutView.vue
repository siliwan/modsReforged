<template>
  <div class="about" v-if="hasLoaded">
    <h1 class="applicationTitle">{{ applicationName }}</h1>
    <h2 class="applicationVersion">{{ applicationVersion }}</h2>
    <h3>Powered by Tauri <a href="https://tauri.studio" target="_blank" rel="noopener noreferrer"></a></h3>
    <h4>Running on {{ platform }} {{ arch }} version {{ osVersion }}</h4>
  </div>
</template>

<script lang="ts">
import { Vue } from 'vue-class-component'
import { getName, getVersion, getTauriVersion } from '@tauri-apps/api/app'
import { platform, arch, version as osVersion } from '@tauri-apps/api/os'

export default class AboutView extends Vue {
  hasLoaded = false;
  applicationName = ''
  applicationVersion = ''
  tauriVersion = ''

  platform = ''
  arch = ''
  osVersion = ''

  async beforeMount () {
    await Promise.all(
      [
        getName().then((name) => { this.applicationName = name }),
        getVersion().then((version) => { this.applicationVersion = version }),
        getTauriVersion().then((version) => { this.tauriVersion = version }),
        platform().then((platform) => { this.platform = platform }),
        arch().then((arch) => { this.arch = arch }),
        osVersion().then((version) => { this.osVersion = version })
      ]
    )

    this.hasLoaded = true
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
    .about {
     text-align: center;
    }
    .applicationTitle {
      margin-bottom: 0;
    }

    .applicationVersion {
      margin-top: 0;
    }
</style>
