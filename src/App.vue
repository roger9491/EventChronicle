


<template>
  <n-config-provider :theme="darkTheme">
    <n-message-provider>
      <EventChronicle></EventChronicle>
      <!-- <router-view></router-view> -->
    </n-message-provider>
  </n-config-provider>
</template>





<script lang="ts">
// import HelloWorld from './components/HelloWorld.vue'
import  darkTheme  from 'naive-ui';
import EventChronicle from './components/EventChronicle.vue';

export default {
    data() {
        return {
            darkTheme,
            value: null
        };
    },
    methods: {
        handleDateClick(year: number, month: number, date: number) {
            console.log(`Clicked on date: ${year}-${month}-${date}`);
            // 在這裡添加你的事件處理邏輯
        }
    },
  
}

</script>