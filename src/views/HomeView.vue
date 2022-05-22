<template>
  <div class="home">
    <h1 class="applicationTitle">{{ applicationName }}</h1>
  </div>
</template>

<script lang="ts">
import { Vue } from 'vue-class-component'
import { getName, getVersion } from "@tauri-apps/api/app";

export default class HomeView extends Vue {
  applicationName: String = ''

  async beforeMount () {
    this.applicationName = await getName()
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
    .applicationTitle {
      margin-bottom: 0;
    }
</style>
