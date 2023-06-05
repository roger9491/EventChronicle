import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { invoke } from '@tauri-apps/api'
import router from './router/router'



createApp(App).use(router).mount('#app')
