<script setup lang="ts"></script>

<template>
  <div
    class="page mobile-height max-w-screen-sm mx-auto py-8 px-4 overflow-y-auto"
  >
    <router-view v-slot="{ Component, route }">
      <transition name="scale" mode="out-in">
        <component :is="Component" :key="route.path" />
      </transition>
    </router-view>
  </div>
</template>

<style scoped></style>
