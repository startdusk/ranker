<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";

import { usePollStore } from "../stores/PollStore";

import CountSelector from "../components/ui/CountSelector.vue";
import { AppPage } from "../router/page";
import { Poll } from "../poll-types";
import { sleep } from "../utils";

const pollStore = usePollStore();
const router = useRouter();

const pollTopic = ref("");
const maxVotes = ref(3);
const enterName = ref("");
const apiError = ref("");

const onChange = (val: number) => {
  maxVotes.value = val;
};

const areFieldsInvalid = (): boolean => {
  if (pollTopic.value.length < 1 || pollTopic.value.length > 100) {
    return true;
  }
  if (maxVotes.value < 1 || maxVotes.value > 3) {
    return true;
  }
  if (enterName.value.length < 1 || enterName.value.length > 25) {
    return true;
  }

  return false;
};

const handleCreatePoll = async () => {
  pollStore.startLoading();
  const poll: Poll = {
    id: "123456",
    topic: pollTopic.value,
    votesPerVoter: 0,
    participants: {
      "my-id": "this is my participanis",
      "user-id1": "user 1 post a participants",
      "user-id2": "user 2 post a participants",
    },
    adminId: "my-id",
    nominations: {},
    rankings: {},
    results: [],
    hasStarted: false,
  };
  pollStore.initializePoll(poll);
  await sleep(3000);
  router.push(AppPage.WaitingRoom);
  pollStore.stopLoading();
};
</script>
<template>
  <div
    class="flex flex-col w-full justify-around items-stretch h-full mx-auto max-w-sm"
  >
    <div class="mb-12">
      <h3 class="text-center">Enter Poll Topic</h3>
      <div class="text-center w-full">
        <input maxlength="100" class="box info w-full" v-model="pollTopic" />
      </div>
      <h3 class="text-center mt-4 mb-2">Votes Per Participant</h3>
      <div class="w-48 mx-auto my-4">
        <CountSelector
          :min="1"
          :max="5"
          :initial="3"
          :step="1"
          :onChange="onChange"
        />
      </div>
      <div class="mb-12">
        <h3 class="text-center">Enter Name</h3>
        <div class="text-center w-full">
          <input maxlength="25" class="box info w-full" v-model="enterName" />
        </div>
      </div>
      <p v-if="apiError" class="text-center text-red-600 font-light mt-8">
        {{ apiError }}
      </p>
    </div>
    <div class="flex flex-col justify-center items-center">
      <button
        class="box btn-orange w-32 my-2"
        :disabled="areFieldsInvalid()"
        @click="handleCreatePoll"
      >
        Create
      </button>
      <router-link :to="AppPage.Welcome">
        <button class="box btn-purple w-32 my-2">Start Over</button>
      </router-link>
    </div>
  </div>
</template>
