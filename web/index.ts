import { createApp } from "vue";
import * as VueRouter from 'vue-router'
import App from "./App.vue";
import Home from "./Home.vue";
import SignIn from "./SignIn.vue";

const routes = [
  { path: '/', component: Home },
  { path: '/signin', component: SignIn },
];
const router = VueRouter.createRouter({
  history: VueRouter.createWebHashHistory(),
  routes,
});
const app = createApp(App);
app.use(router);
app.mount("#app");
