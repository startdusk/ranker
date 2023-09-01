<script setup lang="ts">
import { ref, watchEffect } from 'vue';

type PropsType = {
  min: number;
  max: number;
  step: number;
  initial: number;
  onChange: (val: number) => void;
};

let { min, max, step, initial, onChange } = defineProps<PropsType>();

if (initial < min || initial > max) {
  console.warn(
    `'initial' = ${initial} must in the rang eof ${min} and ${max}. Setting a default initial value`
  );
  const steps = (max - min) / step;
  initial = min + Math.floor(steps);
}

const current = ref(initial);

watchEffect(() => {
  onChange(current.value);
});

const minusDisable = () => {
  return current.value - step < min;
};

const plusDisable = () => {
  return current.value + step > max;
};

const handleMinusClick = () => {
  current.value -= step;
};

const handlePlusClick = () => {
  current.value += step;
};
</script>
<template>
  <div class="flex justify-between items-center">
    <button
      type="button"
      class="btn-round btn-round-orange"
      :disabled="minusDisable()"
      @click="handleMinusClick"
    >
      -
    </button>
    <div class="text-2xl font-bold">{{ current }}</div>
    <button
      type="button"
      class="btn-round btn-round-orange"
      :disabled="plusDisable()"
      @click="handlePlusClick"
    >
      +
    </button>
  </div>
</template>
