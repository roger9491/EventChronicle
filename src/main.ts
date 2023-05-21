import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { invoke } from '@tauri-apps/api'


// 調用 rs 指令
invoke('greet', { name: 'tauri' })
    .then((response) => console.log(response))

createApp(App).mount('#app')
