<script setup lang="ts">
import copy from "copy-to-clipboard";

import { usePollStore } from "../stores/PollStore";

import ColorizeText from "../components/ui/ColorizeText.vue";
import ComfirmationDialog from "../components/ui/ConfirmationDialog.vue";
import { ref } from "vue";

const showConfirmation = ref(false);

const pollStore = usePollStore();

const copyToClipboard = () => {
  copy(pollStore.poll?.id!);
};

const handleStartVote = () => {
  pollStore.startVote();
};

const leavePoll = () => {
  showConfirmation.value = true;
};

const cancelLeavePoll = () => {
  showConfirmation.value = false;
};

const comfirmLeavePoll = () => {
  showConfirmation.value = false;
};
</script>
<template>
  <div class="flex flex-col w-full justify-between items-center h-full">
    <div>
      <h2 class="text-center">Poll Topic</h2>
      <p class="italic text-center mb-4">{{ pollStore.poll?.topic }}</p>
      <h2 class="text-center">Poll ID</h2>
      <h3 class="text-center mb-2">Click to copy!</h3>
      <div
        @click="copyToClipboard"
        class="mb-4 flex justify-center align-middle cursor-pointer"
      >
        <div class="font-extrabold text-center mr-2">
          <ColorizeText :text="pollStore.poll?.id!" />
        </div>
        <v-icon name="md-contentcopy" />
      </div>
    </div>

    <div class="flex justify-center">
      <button class="box btn-orange mx-2 pulsate">
        <v-icon name="md-people-outlined" />
        <span>{{ pollStore.participantCount }}</span>
      </button>
      <button class="box btn-purple mx-x pulsate">
        <v-icon name="bi-pencil-square" />
        <span>{{ pollStore.nominationCount }}</span>
      </button>
    </div>

    <div class="flex flex-col justify-center">
      <template v-if="pollStore.isAdmin">
        <div class="my-2 italic">
          {{ pollStore.poll?.votesPerVoter }} Nominations Required to Start!
        </div>
        <button
          class="box btn-orange my-2"
          :disabled="!pollStore.canStartVote"
          @click="handleStartVote"
        >
          Start Voting
        </button>
      </template>
      <template v-else>
        <div class="my-2 italic">
          Waiting for Admin,
          <span class="font-semibold"> {{ pollStore.participant }} </span>
          , to start the voting.
        </div>
      </template>

      <button class="box btn-purple my-2" @click="leavePoll">Leave Poll</button>
      <ComfirmationDialog
        message="You'll be kicked out of the poll"
        :showDialog="showConfirmation"
        :onCancel="cancelLeavePoll"
        :onConfirm="comfirmLeavePoll"
      />
    </div>
  </div>
</template>
