<script setup lang="ts">
import { AppPage } from '../router/page';
</script>
<template>
  <div class="flex flex-col justify-center items-center h-full">
    <h1 class="text-center my-2">Welcome to Ranker</h1>
    <div class="my-12 flex flex-col justify-center">
      <router-link :to="AppPage.Create" class="box btn-orange my-2">
        <button>Create New Poll</button>
      </router-link>
      <router-link :to="AppPage.Join" class="box btn-purple my-2">
        <button>Join Existing Poll</button>
      </router-link>
    </div>
  </div>
</template>
