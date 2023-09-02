<script setup lang="ts">
import { ref, watchEffect } from "vue";
import copy from "copy-to-clipboard";

import { usePollStore } from "../stores/PollStore";

import ColorizeText from "../components/ui/ColorizeText.vue";
import ComfirmationDialog from "../components/ui/ConfirmationDialog.vue";
import ParticipantList from "../components/ParticipantList.vue";
import { useRouter } from "vue-router";
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
  isConfirmationOpen.value = true;
};

const setIsConfirmationOpen = () => {
  isConfirmationOpen.value = true;
};

const closeParticipantList = () => {
  isParticipantListOpen.value = false;
};

const confirmRemoveParticipant = (id: string) => {
  confirmationMessage.value = `Remove ${pollStore.poll?.participants[id]} from poll?`;
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
          :showDialog="showConfirmation"
          :onCancel="cancelLeavePoll"
          :onConfirm="confirmLeavePoll"
        />
      </div>
    </div>
    <ParticipantList
      :isOpen="isParticipantListOpen"
      :onClose="closeParticipantList"
      :participants="pollStore.poll?.participants"
      :isAdmin="pollStore.isAdmin || false"
      :userId="pollStore.me?.id"
      :onRemoveParticipant="confirmRemoveParticipant"
    />
    <ComfirmationDialog
      :message="confirmationMessage"
      :showDialog="isConfirmationOpen"
      :onCancel="submitRemoveParticipant"
      :onConfirm="setIsConfirmationOpen"
    />
  </div>
</template>
