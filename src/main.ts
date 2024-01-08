import { createApp } from "vue";
import "./styles.css";
import router from './router.ts'
import App from "./App.vue";
import Vue3Toasity from 'vue3-toastify';
import 'vue3-toastify/dist/index.css';

const app = createApp(App)
app.use(router)
app.use(Vue3Toasity, {
    position: "bottom-right",
    autoClose: 2000
})
app.mount("#app")