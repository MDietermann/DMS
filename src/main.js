import {createRouter, createWebHistory} from 'vue-router'
import 'bootstrap-vue-3/dist/bootstrap-vue-3.css'
import BootstrapVue3 from 'bootstrap-vue-3'
import 'bootstrap/dist/css/bootstrap.css'
import { createPinia } from 'pinia'
import { routes } from './routes'
import { createApp } from 'vue'
import App from './App.vue'
import './style.css'


const router = createRouter({
    history: createWebHistory(),
    routes
})

createApp(App)
    .use(router)
    .use(createPinia())
    .use(BootstrapVue3)
    .mount('#app')
