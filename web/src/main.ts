import { createApp } from "vue";

import { addIcons, OhVueIcon } from "oh-vue-icons";
import {
  MdContentcopy,
  MdPeopleOutlined,
  BiPencilSquare,
  MdCancel,
} from "oh-vue-icons/icons";
import { createPinia } from "pinia";

import router from "./router";

import App from "./App.vue";
import "./style.css";

addIcons(MdContentcopy, MdPeopleOutlined, BiPencilSquare, MdCancel);

const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.component("v-icon", OhVueIcon);
app.use(router);
app.mount("#app");
