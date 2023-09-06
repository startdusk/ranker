<script setup lang="ts">
import { computed, ref } from 'vue';

type PropsType = {
  min: number;
  max: number;
  step: number;
  initial: number;
};

let { min, max, step, initial } = defineProps<PropsType>();
const emits = defineEmits<{
  (e: 'on-change', val: number): void;
}>();

if (initial < min || initial > max) {
  console.warn(
    `'initial' = ${initial} must in the rang eof ${min} and ${max}. Setting a default initial value`
  );
  const steps = (max - min) / step;
  initial = min + Math.floor(steps);
}

const current = ref(initial);

const minusDisable = computed(() => {
  return current.value - step < min;
});

const plusDisable = computed(() => {
  return current.value + step > max;
});

const handleMinusClick = () => {
  current.value -= step;
  emits('on-change', current.value);
};

const handlePlusClick = () => {
  current.value += step;
  emits('on-change', current.value);
};
</script>
<template>
  <div class="flex justify-between items-center">
    <button
      type="button"
      class="btn-round btn-round-orange"
      :disabled="minusDisable"
      @click="handleMinusClick"
    >
      -
    </button>
    <div class="text-2xl font-bold">{{ current }}</div>
    <button
      type="button"
      class="btn-round btn-round-orange"
      :disabled="plusDisable"
      @click="handlePlusClick"
    >
      +
    </button>
  </div>
</template>
