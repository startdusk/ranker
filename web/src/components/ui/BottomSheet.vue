<script setup lang="ts">
type PropsType = {
  isOpen: boolean;
  onClose?: () => void;
};

const { isOpen = false, onClose } = defineProps<PropsType>();
</script>
<template>
  <Transition name="slide-fade" :duration="3000">
    <div
      v-if="isOpen"
      class="absolute left-0 right-0 max-w-screen-sm bg-gray-50 bottom-0 z-10 overflow-y-hidden top-16 flex flex-col"
    >
      <div class="sticky top-0 flex justify-end flex-grow-0">
        <v-icon
          name="md-cancel"
          class="mr-2 mt-2 fill-current text-orange-700 cursor-pointer hover:opacity-80"
          @click="onClose"
        />
      </div>
      <div class="relative overflow-y-hidden bg-gray-50 flex-grow">
        <div class="absolute top-0 bottom-0 left-0 right-0 overflow-y-auto">
          <slot></slot>
        </div>
      </div>
    </div>
  </Transition>
</template>
<style scoped>
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.8s cubic-bezier(1, 0.5, 0.8, 1);
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}
</style>
