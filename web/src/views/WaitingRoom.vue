<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { useRouter } from "vue-router";
import copy from "copy-to-clipboard";

import { usePollStore } from "../stores/PollStore";

import ColorizeText from "../components/ui/ColorizeText.vue";
import ComfirmationDialog from "../components/ui/ConfirmationDialog.vue";
import ParticipantList from "../components/ParticipantList.vue";
import NominationForm from "../components/NominationForm.vue";

import { AppPage } from "../router/page";

const showConfirmation = ref(false);
const isConfirmationOpen = ref(false);
const confirmationMessage = ref("");
const participantToRemove = ref<string>();
const isNominationFormOpen = ref(false);
const isParticipantListOpen = ref(false);

const router = useRouter();
const pollStore = usePollStore();

watchEffect(() => {
  if (!pollStore.poll) {
    router.push(AppPage.Welcome);
    return;
  }
});

const copyToClipboard = () => {
  copy(pollStore.poll?.id!);
};

const handleStartVote = () => {
  pollStore.startVote();
};

const cancelLeavePoll = () => {
  showConfirmation.value = false;
};

const confirmLeavePoll = () => {
  pollStore.startVote();
};

const submitRemoveParticipant = () => {
  participantToRemove.value &&
    pollStore.removeParticipant(participantToRemove.value);
  isConfirmationOpen.value = false;
};

const closeParticipantList = () => {
  isParticipantListOpen.value = false;
};

const confirmRemoveParticipant = (id: string) => {
  confirmationMessage.value = `Remove "${pollStore.poll?.participants[id]}" from poll?`;
  participantToRemove.value = id;
  isConfirmationOpen.value = true;
};
</script>
<template>
  <div v-if="pollStore.poll">
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
        <button
          class="box btn-orange mx-2 pulsate"
          @click="isParticipantListOpen = true"
        >
          <v-icon name="md-people-outlined" />
          <span>{{ pollStore.participantCount }}</span>
        </button>
        <button
          class="box btn-purple mx-x pulsate"
          @click="isNominationFormOpen = true"
        >
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
            <span class="font-semibold">
              {{ " " + pollStore.participant }}
            </span>
            , to start the voting.
          </div>
        </template>

        <button class="box btn-purple my-2" @click="showConfirmation = true">
          Leave Poll
        </button>
        <ComfirmationDialog
          message="You'll be kicked out of the poll"
          :show-dialog="showConfirmation"
          :on-cancel="cancelLeavePoll"
          :on-confirm="confirmLeavePoll"
        />
      </div>
    </div>
    <ParticipantList
      :is-open="isParticipantListOpen"
      :on-close="closeParticipantList"
      :participants="pollStore.poll?.participants"
      :is-admin="pollStore.isAdmin || false"
      :user-id="pollStore.me?.id"
      :on-remove-participant="confirmRemoveParticipant"
    />
    <NominationForm
      :title="pollStore.poll.topic"
      :is-open="isNominationFormOpen"
      :on-close="() => (isNominationFormOpen = false)"
      :nominations="pollStore.poll.nominations"
      :user-id="pollStore.me?.id"
      :is-admin="pollStore.isAdmin || false"
      :on-submit-nomination="(text) => pollStore.nominate(text)"
      :on-remove-nomination="
        (nominationId) => pollStore.removeNomination(nominationId)
      "
    />
    <ComfirmationDialog
      :message="confirmationMessage"
      :show-dialog="isConfirmationOpen"
      :on-cancel="() => (isConfirmationOpen = false)"
      :on-confirm="submitRemoveParticipant"
    />
  </div>
</template>
