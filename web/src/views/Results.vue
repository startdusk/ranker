<script setup lang="ts">
import { ref } from "vue";

import ConfirmationDialog from "../components/ui/ConfirmationDialog.vue";
import ResultCard from "../components/ui/ResultCard.vue";
import { usePollStore } from "../stores/PollStore";

const pollStore = usePollStore();
const isConfirmationOpen = ref(false);
const isLeavePollOpen = ref(false);
</script>
<template>
  <div>
    <div
      class="mx-auto flex flex-col w-full justify-between items-center h-full max-w-sm"
    >
      <div class="w-full">
        <h1 class="text-center mt-12 mb-4">Results</h1>
        <ResultCard
          v-if="pollStore.poll?.results.length"
          :results="pollStore.poll?.results"
        />
        <p v-else class="text-center text-xl">
          <span class="text-orange-600">{{ pollStore.rankingsCount }} of </span>
          <span class="text-purple-600">{{ pollStore.participantCount }} </span>
          participants have voted
        </p>
      </div>
      <div class="flex flex-col justify-center">
        <button
          v-if="pollStore.isAdmin && !pollStore.poll?.results.length"
          class="box btn-orange my-2"
          @click="isConfirmationOpen = true"
        >
          End Poll
        </button>
        <div
          v-if="pollStore.isAdmin && !pollStore.poll?.results.length"
          class="my-2 italic"
        >
          Waiting for Admin,
          <span class="font-semibold">
            {{ pollStore.poll?.participants[pollStore.poll?.adminId] }}
          </span>
          , to finalize the poll.
        </div>
        <button
          v-if="!!pollStore.poll?.results.length"
          class="box btn-purple my-2"
          @click="isLeavePollOpen = true"
        >
          Leave Poll
        </button>
      </div>
    </div>
    <ConfirmationDialog
      v-if="pollStore.isAdmin"
      message="Are you sure close the poll and calculate the results?"
      :show-dialog="isConfirmationOpen"
      :on-cancel="
        () => {
          isConfirmationOpen = false;
        }
      "
      :on-confirm="
        () => {
          pollStore.closePoll();
          isConfirmationOpen = false;
        }
      "
    />
    <ConfirmationDialog
      v-if="isLeavePollOpen"
      message="You'll lose ya results. Dat alright?"
      :show-dialog="isLeavePollOpen"
      :on-cancel="
        () => {
          isLeavePollOpen = false;
        }
      "
      :on-confirm="
        () => {
          pollStore.startOver();
        }
      "
    />
  </div>
</template>
