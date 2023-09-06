<script setup lang="ts">
import { ref } from 'vue';

import { Nominations } from '../poll-types';
import BottomSheet from './ui/BottomSheet.vue';

type PropsType = {
  isOpen: boolean;
  title?: string;
  nominations?: Nominations;
  userId?: string;
  isAdmin: boolean;
};

const {
  isOpen,
  title,
  nominations = {},
  userId,
  isAdmin,
} = defineProps<PropsType>();

const emits = defineEmits<{
  (e: 'on-close'): void;
  (e: 'on-submit-nomination', nomination: string): void;
  (e: 'on-remove-nomination', nominationId: string): void;
}>();

const nominationText = ref('');

const handleSubmitNomination = () => {
  emits('on-submit-nomination', nominationText.value);
  nominationText.value = '';
};

const handleRemoveNomination = (nominationId: string) => {
  emits('on-remove-nomination', nominationId);
};

const getBoxStyle = (id: string): string => {
  return id === userId
    ? 'bg-orange-100 flex-row'
    : 'bg-gray-100 flex-row-reverse';
};

const handleClose = () => {
  emits('on-close');
};
</script>
<template>
  <BottomSheet :is-open="isOpen" @on-close="handleClose">
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
              @click="handleRemoveNomination"
            />
          </div>
        </div>
      </div>
    </div>
  </BottomSheet>
</template>
