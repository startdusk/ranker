<script setup lang="ts">
import { ref, watchEffect } from "vue";

type PropsType = {
  type?: "standard" | "error";
  title?: string;
  message: string;
  show: boolean;
  autoCloseDuration?: number;
};

const snackBarStyles = {
  standard: "bg-gray-100 bg-opacity-50",
  error: "bg-red-600 text-white",
};

const emits = defineEmits<{
  (e: "on-close"): void;
}>();

const {
  type = "standard",
  title,
  message,
  show,
  autoCloseDuration,
} = defineProps<PropsType>();
const outerStyles = ref(snackBarStyles[type]);
const showSnackBar = ref(false);

const handleCloseSnackBar = () => {
  showSnackBar.value = false;
  emits("on-close");
};

watchEffect(() => {
  if (show) {
    showSnackBar.value = true;
  }

  autoCloseDuration &&
    setTimeout(() => handleCloseSnackBar(), autoCloseDuration);
});
</script>
<template>
  <transition name="slide-fade" :duration="300">
    <div
      class="relative shadow-md py-2 mb-1 z-50 rounded-b-md text-center w-full sm:w-1/2 top-0 left-0 right-0 mx-auto bg-opacity-100"
      :class="outerStyles"
    >
      <div class="absolute top-0 right-0">
        <v-icon
          name="md-cancel"
          class="fill-current mr-1 mt-1 cursor-pointer hover:opacity-80"
          @click="handleCloseSnackBar"
        />
      </div>
      <div class="mt-4 mx-8 mb-2">
        <h3 v-if="title" class="font-semibold">{{ title }}</h3>
        <div class="text-sm font-light italic">{{ message }}</div>
      </div>
    </div>
  </transition>
</template>

<style scoped>
.slide-fade-enter-active {
  transition: all 0.8s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-leave-active {
  transition: all 0.3s ease-out;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}
</style>
