<script setup lang="ts">
type PropsType = {
  rank?: number;
  value: string;
};

const { rank, value } = defineProps<PropsType>();

const emits = defineEmits<{
  (e: 'on-select'): void;
}>();

const handleSelect = () => {
  emits('on-select');
};
</script>
<template>
  <div class="my-4 box btn-orange relative" @click="handleSelect">
    <div>{{ value }}</div>
    <div
      v-if="rank"
      class="absolute w-6 h-6 -top-3 -right-3 rounded-full bg-purple-600"
    >
      <div class="text-center font-medium text-white">{{ rank }}</div>
    </div>
  </div>
</template>
