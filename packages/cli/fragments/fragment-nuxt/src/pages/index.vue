<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>
    <div class="row">
      <span class="logos"
        ><a href="https://nuxt.com" target="_blank"
          ><img
            alt="Nuxt logo"
            src="~/assets/nuxt.svg"
            width="144"
            height="144"
            decoding="async"
            data-nimg="1"
            class="logo nuxt"
            style="color: transparent" /></a
      ></span>
      <span class="logos"
        ><a href="https://tauri.app" target="_blank"
          ><img
            alt="Tauri logo"
            src="~/assets/tauri.svg"
            width="144"
            height="144"
            decoding="async"
            data-nimg="1"
            class="logo tauri"
            style="color: transparent" /></a
      ></span>
      <span class="logos"
        ><a href="https://nuxt.com" target="_blank"
          ><img
            alt="Vue logo"
            src="~/assets/vue.svg"
            width="144"
            height="144"
            decoding="async"
            data-nimg="1"
            class="logo vue"
            style="color: transparent" /></a
      ></span>
    </div>
    <p>Click on the Tauri, Nuxt and Vue logos to learn more.</p>
    <div class="row">
      <input v-model="input" id="greet-input" placeholder="Enter a name..." />
      <button type="button" @click="runCommand">Greet</button>
    </div>
    <p>
      {{ output }}
    </p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      input: "",
      output: "",
    };
  },
  methods: {
    async runCommand() {
      this.output = await this.$tauri.invoke("greet", { name: this.input });
    },
  },
};
</script>

<style>
.logo.nuxt:hover {
  filter: drop-shadow(0 0 2em #108775);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #41b883);
}
</style>
