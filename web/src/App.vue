<script setup lang="ts">
import { watchEffect } from "vue";
import Loader from "./components/ui/Loader.vue";
import { usePollStore } from "./stores/PollStore";
import { sleep } from "./utils";

const state = usePollStore();
watchEffect(async () => {
  console.log("App useEffect - check token and send to proper page");

  state.startLoading();
  const accessToken = localStorage.getItem("accessToken");
  await sleep(1000);
  // if there's not access token, we'll be shown the default
  // state.currentPage of AppPage.Welcome
  if (!accessToken) {
    state.stopLoading();
    return;
  }
});
</script>

<template>
  <Loader :isLoading="state.isLoading" color="orange" :width="120" />
  <div
    class="page mobile-height max-w-screen-sm mx-auto py-8 px-4 overflow-y-auto"
  >
    <router-view v-slot="{ Component, route }">
      <transition name="scale" mode="out-in">
        <component :is="Component" :key="route.path" />
      </transition>
    </router-view>
  </div>
</template>

<style scoped></style>
