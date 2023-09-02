<script setup lang="ts">
type PropsType = {
  rank?: number;
  value: string;
  onSelect: () => void;
};

const { rank, value, onSelect } = defineProps<PropsType>();
</script>
<template>
  <div class="my-4 box btn-orange relative" @click="() => onSelect()">
    <div>{{ value }}</div>
    <div
      v-if="rank"
      class="absolute w-6 h-6 -top-3 -right-3 rounded-full bg-purple-600"
    >
      <div class="text-center font-medium text-white">{{ rank }}</div>
    </div>
  </div>
</template>
