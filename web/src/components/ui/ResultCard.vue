<script setup lang="ts">
import { DeepReadonly } from "vue";
import { Results } from "../../poll-types";

type PropsType = {
  results: DeepReadonly<Results>;
};

const { results } = defineProps<PropsType>();
</script>
<template>
  <div
    class="grid grid-cols-3 gap-4 pb-2 my-2 border-b-2 border-solid border-purple-70 pr-4"
  >
    <div class="col-span-2 font-semibold">Candidate</div>
    <div class="col-span-1 font-semibold text-right">Score</div>
  </div>
  <div class="divide-y-2 overflow-y-auto pr-4">
    <div
      v-for="result of results"
      :key="result.nominationId"
      class="grid grid-cols-3 gap-4 my-1 items-center"
    >
      <div class="col-span-2">{{ result.nominationText }}</div>
      <div class="col-span-1 text-right">{{ result.score.toFixed(2) }}</div>
    </div>
  </div>
</template>
