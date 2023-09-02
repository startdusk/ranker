<script setup lang="ts">
import { watchEffect } from "vue";
import Loader from "./components/ui/Loader.vue";
import Pages from "./Pages.vue";

import { usePollStore } from "./stores/PollStore";
import { getTokenPayload, sleep } from "./utils";

const state = usePollStore();
watchEffect(async () => {
  console.log("App useEffect - check token and send to proper page");

  state.startLoading();
  // const accessToken = localStorage.getItem("accessToken");
  const accessToken = `eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJwb2xsX2lkIjoiNVZNSTJWIiwibmFtZSI6ImNsaWVudDEiLCJzdWIiOiJXNHlVS1ZmSXUwMGliTXdscGlFSE0iLCJjb21wYW55IjoiUmFua2VyIEluYy4iLCJleHAiOjE2OTM2NDYyNTN9.rKOI2GQQUTxJyqh_94K95ReYnSEil2H11kiNL2gsqlY`;
  await sleep(1000);
  // if there's not access token, we'll be shown the default
  // state.currentPage of AppPage.Welcome
  if (!accessToken) {
    state.stopLoading();
    return;
  }
  const { exp: tokenExp } = getTokenPayload(accessToken);

  console.log(`exp`, tokenExp);
  // const currentTimeInSeconds = Date.now() / 1000;
  // // Remove old token
  // // if token is within 10 seconds, we'll prevent
  // // them from connecting (poll will almost be over)
  // // since token duration and poll duration are
  // // approximately at the same time
  // if (tokenExp < currentTimeInSeconds - 10) {
  //   localStorage.removeItem("accessToken");
  //   state.stopLoading();
  //   return;
  // }

  // reconnect to poll
  state.setPollAccessToken(accessToken); // needed for socket.io connection
  // socket initialization on server sends updated poll to the client
  // state.initializeSocket();
  state.stopLoading();
});
</script>

<template>
  <Loader :isLoading="state.isLoading" color="orange" :width="120" />
  <Pages />
</template>

<style scoped></style>
