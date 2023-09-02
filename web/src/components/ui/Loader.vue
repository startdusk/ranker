<script setup lang="ts">
import { reactive, ref } from "vue";

type PropsType = {
  color: "blue" | "orange" | "purple";
  isLoading: boolean;
  width?: number;
};

const colorStyles = {
  blue: "bg-blue",
  orange: "bg-orange",
  purple: "bg-purple",
};

const { color, isLoading, width = 80 } = defineProps<PropsType>();
const colorStyle = ref(colorStyles[color] || "bg-purple");

const d = width / 7;
const dRef = ref(3 * d + "px");
const widthRef = ref(width + "px");
const left1Ref = ref(d + "px");
const left2Ref = ref(3 * d + "px");
const left3Ref = ref(5 * d + "px");

const dotStyles = reactive({
  width: d + "px",
  height: d + "px",
  top: d + "px",
});
</script>
<template>
  <div v-if="isLoading" class="overlay">
    <div class="lds-ellipsis" :style="{ height: dRef, width: widthRef }">
      <div :class="colorStyle" :style="{ ...dotStyles, left: left1Ref }"></div>
      <div :class="colorStyle" :style="{ ...dotStyles, left: left1Ref }"></div>
      <div
        :class="colorStyle"
        :style="{
          ...dotStyles,
          left: left2Ref,
        }"
      ></div>
      <div
        :class="colorStyle"
        :style="{
          ...dotStyles,
          left: left3Ref,
        }"
      ></div>
    </div>
  </div>
</template>

<style scoped>
.overlay {
  height: 100%;
  width: 100%;
  position: absolute;
  z-index: 1;
  left: 0;
  top: 0;
  background-color: rgba(68, 68, 68, 0.2);
  overflow-x: hidden;
  transition: 300ms;
  display: flex;
  justify-content: center;
  align-items: center;
}

.lds-ellipsis {
  display: inline-block;
  position: relative;
  width: 80px;
  height: 80px;
}

.lds-ellipsis div {
  position: absolute;
  border-radius: 50%;
  animation-timing-function: cubic-bezier(0, 1, 1, 0);
}

.lds-ellipsis div:nth-child(1) {
  animation: lds-ellipsis1 0.6s infinite;
}

.lds-ellipsis div:nth-child(2) {
  animation: lds-ellipsis2 0.6s infinite;
}

.lds-ellipsis div:nth-child(3) {
  animation: lds-ellipsis2 0.6s infinite;
}

.lds-ellipsis div:nth-child(4) {
  animation: lds-ellipsis3 0.6s infinite;
}

@keyframes lds-ellipsis1 {
  0% {
    transform: scale(0);
  }
  100% {
    transform: scale(1);
  }
}

@keyframes lds-ellipsis2 {
  0% {
    transform: translate(0, 0);
  }
  100% {
    transform: translate(200%, 0);
  }
}

@keyframes lds-ellipsis3 {
  0% {
    transform: scale(1);
  }
  100% {
    transform: scale(0);
  }
}

.style3 {
  width: v-bind("d");
  height: v-bind("d");
  top: v-bind("d");
}
</style>
