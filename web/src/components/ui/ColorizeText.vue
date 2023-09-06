<script setup lang="ts">
type PropsType = {
  text: string;
};

const { text } = defineProps<PropsType>();
</script>
<template>
  <template v-for="(val, index) in text.split('')">
    <!-- vue v-if/v-else must be use unique key -->
    <span
      v-if="val.charCodeAt(0) >= 48 && val.charCodeAt(0) <= 57"
      :key="index + '_orange'"
      class="text-orange-600"
      >{{ val }}</span
    >
    <span v-else :key="index + '_indigo'" class="text-indigo-6000">{{
      val
    }}</span>
  </template>
</template>
