<script setup lang="ts">
import BottomSheet from "./ui/BottomSheet.vue";
import { Participants } from "../poll-types";

type PropsType = {
  participants?: Participants;
  userId?: string;
  isAdmin: boolean;
  onRemoveParticipant: (id: string) => void;
  isOpen: boolean;
  onClose: () => void;
};

const {
  isOpen,
  onClose,
  participants = {},
  onRemoveParticipant,
  userId,
  isAdmin,
} = defineProps<PropsType>();
</script>
<template>
  <BottomSheet :is-open="isOpen" :on-close="onClose">
    <div class="px-8 flex flex-wrap justify-center mb-2">
      <div
        v-for="[id, participant] in Object.entries(participants)"
        :key="id"
        class="mx-1 my-1 p-4 shadow-xl bg-white flex justify-between items-center rounded-md"
      >
        <span class="ml-2 mr-1 text-indigo-700 text-xl text-center">
          {{ participant }}
        </span>
        <span
          v-if="isAdmin && userId !== id"
          @click="onRemoveParticipant(id)"
          class="ml-1 mr-2 cursor-pointer"
        >
          <v-icon
            name="md-cancel"
            class="fill-current text-black align-middle"
          />
        </span>
      </div>
    </div>
  </BottomSheet>
</template>
