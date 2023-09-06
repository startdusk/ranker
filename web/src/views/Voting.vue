<script setup lang="ts">
import { ref } from 'vue';
import ConfirmationDialog from '../components/ui/ConfirmationDialog.vue';
import RankedCheckBox from '../components/ui/RankedCheckBox.vue';
import { usePollStore } from '../stores/PollStore';

const pollStore = usePollStore();
const rankings = ref<string[]>([]);
const confirmCancel = ref(false);
const confirmVotes = ref(false);

const toggleNomination = (id: string) => {
  const position = rankings.value.findIndex((ranking) => ranking === id);
  const hasVotesRemaining =
    (pollStore.poll?.votesPerVoter || 0) - rankings.value.length > 0;

  if (position < 0 && hasVotesRemaining) {
    rankings.value = [...rankings.value, id];
  } else {
    rankings.value = [
      ...rankings.value.slice(0, position),
      ...rankings.value.slice(position + 1, rankings.value.length),
    ];
  }
};

const getRank = (id: string) => {
  const position = rankings.value.findIndex((ranking) => ranking === id);
  return position < 0 ? undefined : position + 1;
};

const handleSubmitRanking = () => {
  pollStore.submitRankings(rankings.value);
};
</script>
<template>
  <div
    class="mx-auto flex flex-col w-full justify-between items-center h-full max-w-sm"
  >
    <div class="w-full">
      <h1 class="text-center">Voting Page</h1>
    </div>
    <div class="w-full">
      <template v-if="pollStore.poll">
        <div class="text-center text-xl font-semibold mb-6">
          Select Your Top {{ pollStore.poll?.votesPerVoter }} Choices
        </div>
        <div class="text-center text-lg font-semibold mb-6 text-indigo-700">
          {{ pollStore.poll.votesPerVoter - rankings.length }} Votes remaining
        </div>
      </template>
      <div class="px-2">
        <RankedCheckBox
          v-for="[nominationId, nomination] in Object.entries(
            pollStore.poll?.nominations || {}
          )"
          :key="nominationId"
          :value="nomination.text"
          :rank="getRank(nominationId)"
          @on-select="
            () => {
              toggleNomination(nominationId);
            }
          "
        />
      </div>
    </div>
    <div class="mx-auto flex flex-col items-center">
      <button
        :disabled="rankings.length < (pollStore.poll?.votesPerVoter ?? 100)"
        class="box btn-purple my-2 w-36"
        @click="confirmVotes = true"
      >
        Submit Votes
      </button>
      <ConfirmationDialog
        message="You cannot change your vote after submitting"
        :show-dialog="confirmVotes"
        @on-cancel="
          () => {
            confirmVotes = false;
          }
        "
        @on-confirm="handleSubmitRanking"
      />
      <template v-if="pollStore.isAdmin">
        <button class="box btn-orange my-2 w-36" @click="confirmCancel = true">
          Cancel Poll
        </button>
        <ConfirmationDialog
          message="This will cancel the poll and remove all users"
          :show-dialog="confirmCancel"
          @on-cancel="
            () => {
              confirmCancel = false;
            }
          "
          @on-confirm="() => pollStore.cancelPoll()"
        />
      </template>
    </div>
  </div>
</template>
