<script setup lang="ts">
import { ref } from "vue";

import { Nominations } from "../poll-types";
import BottomSheet from "./ui/BottomSheet.vue";

type PropsType = {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  nominations?: Nominations;
  userId?: string;
  isAdmin: boolean;
  onSubmitNomination: (nomination: string) => void;
  onRemoveNomination: (nominationId: string) => void;
};

const {
  isOpen,
  onClose,
  title,
  nominations = {},
  userId,
  isAdmin,
  onSubmitNomination,
  onRemoveNomination,
} = defineProps<PropsType>();

const nominationText = ref("");

const handleSubmitNomination = () => {
  onSubmitNomination(nominationText.value);
  nominationText.value = "";
};

const getBoxStyle = (id: string): string => {
  return id === userId
    ? "bg-orange-100 flex-row"
    : "bg-gray-100 flex-row-reverse";
};
</script>
<template>
  <BottomSheet :is-open="isOpen" :on-close="onClose">
    <div class="flex flex-col px-4 items-center mb-2">
      <h3 class="font-semibold">{{ title }}</h3>
      <div class="w-full my-4">
        <textarea
          :rows="2"
          :maxLength="100"
          class="box info w-full"
          v-model="nominationText"
        />
      </div>
      <button
        class="box btn-purple"
        :disabled="!nominationText.length || nominationText.length > 100"
        @click="handleSubmitNomination"
      >
        Nominate
      </button>

      <h2 class="text-center text-xl my-4 font-medium">Nominations</h2>
      <div class="w-full mb-2">
        <div
          v-for="[nominationId, nomination] in Object.entries(nominations)"
          :key="nominationId"
          class="my-2 flex justify-between items-center p-2 rounded-md"
          :class="`${getBoxStyle(nomination.userId)}`"
        >
          <div>{{ nomination.text }}</div>
          <div v-if="isAdmin" class="ml-2">
            <v-icon
              name="md-cancel"
              class="fill-current cursor-pointer hover:opacity-80"
              @click="onRemoveNomination(nominationId)"
            />
          </div>
        </div>
      </div>
    </div>
  </BottomSheet>
</template>
