<script setup lang="ts">
type PropsType = {
  message: string;
  showDialog: boolean;
};

const { message, showDialog } = defineProps<PropsType>();

const emits = defineEmits<{
  (e: 'on-cancel'): void;
  (e: 'on-confirm'): void;
}>();

const handleCancel = () => {
  emits('on-cancel');
};

const handleConfirm = () => {
  emits('on-confirm');
};
</script>
<template>
  <div
    v-if="showDialog"
    class="z-50 fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full"
  >
    <div
      class="absolute bg-white p-4 rounded-xl drop-shadow-md top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64"
    >
      <div class="text-center font-semibold mb-6">{{ message }}</div>
      <div class="flex justify-around my-2">
        <button class="boxless" @click="handleCancel">Cancel</button>
        <button class="box btn-purple" @click="handleConfirm">Confirm</button>
      </div>
    </div>
  </div>
</template>
