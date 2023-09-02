<script setup lang="ts">
import { ref } from "vue";
import { AppPage } from "../router/page";
import { usePollStore } from "../stores/PollStore";

const pollStore = usePollStore();
const pollId = ref("");
const yourName = ref("");
const apiError = ref("");

const areFieldsInvalid = (): boolean => {
  const pollIdValue = pollId.value.trim();
  if (pollIdValue.length < 6 || pollIdValue.length > 6) {
    return true;
  }
  const yourNameValue = yourName.value.trim();
  if (yourNameValue.length < 1 || yourNameValue.length > 25) {
    return true;
  }

  return false;
};

const handleJoinPollClick = () => {
  pollStore.startLoading();
  console.log("handleJoinPollClick");
  pollStore.stopLoading();
};
</script>
<template>
  <div
    class="flex flex-col w-full justify-around items-stretch h-full mx-auto max-w-sm"
  >
    <div class="mb-12">
      <div class="my-4">
        <h3 class="text-center">Enter Code Provided by &quot;Friend&quot;</h3>
        <div class="text-center w-full">
          <input
            maxlength="6"
            autocapitalize="characters"
            style="text-transform: uppercase"
            class="box info w-full"
            v-model.uppercase="pollId"
          />
        </div>
      </div>
      <div class="my-4">
        <h3 class="text-center">Your Name</h3>
        <div class="text-center w-full">
          <input maxlength="25" v-model="yourName" class="box info w-full" />
        </div>
      </div>

      <p v-if="apiError" class="text-center text-red-600 font-light mt-8">
        {{ apiError }}
      </p>
    </div>
    <div class="my-12 flex flex-col justify-center items-center">
      <button
        :disabled="areFieldsInvalid()"
        @click="handleJoinPollClick"
        class="box btn-orange w-32 my-2"
      >
        Join
      </button>
      <router-link :to="AppPage.Welcome">
        <button class="box btn-purple w-32 my-2">Start Over</button>
      </router-link>
    </div>
  </div>
</template>
