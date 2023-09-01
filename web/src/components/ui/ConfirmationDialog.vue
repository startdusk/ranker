<script setup lang="ts">
type PropsType = {
  message: string;
  showDialog: boolean;
  onCancel: () => void;
  onConfirm: () => void;
};

const { message, showDialog, onCancel, onConfirm } = defineProps<PropsType>();
</script>
<template>
  <template v-if="showDialog">
    <div
      class="z-50 fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full"
    >
      <div
        class="absolute bg-white p-4 rounded-xl drop-shadow-md top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-64"
      >
        <div class="text-center font-semibold mb-6">{{ message }}</div>
        <div class="flex justify-around my-2">
          <button class="boxless" @click="onCancel">Cancel</button>
          <button class="box btn-purple" @click="onConfirm">Confirm</button>
        </div>
      </div>
    </div>
  </template>
</template>
