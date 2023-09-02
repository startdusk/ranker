<script setup lang="ts">
import { useRouter } from "vue-router";

import { usePollStore } from "./stores/PollStore";
import { AppPage } from "./router/page";

const pollStore = usePollStore();
const router = useRouter();

pollStore.$subscribe((_mutate, _state) => {
  console.log(pollStore.me);
  console.log(pollStore.poll);
  if (pollStore.me?.id && pollStore.poll && !pollStore.poll.hasStarted) {
    router.push(AppPage.WaitingRoom);
  }

  if (pollStore.me?.id && pollStore.poll?.hasStarted) {
    router.push(AppPage.Voting);
  }

  if (pollStore.me?.id && pollStore.hasVoted) {
    router.push(AppPage.Results);
  }
});
</script>
<template>
  <div
    class="page mobile-height max-w-screen-sm mx-auto py-8 px-4 overflow-y-auto"
  >
    <router-view v-slot="{ Component, route }">
      <transition name="scale" mode="out-in" :duration="300">
        <component :is="Component" :key="route.path" />
      </transition>
    </router-view>
  </div>
</template>
