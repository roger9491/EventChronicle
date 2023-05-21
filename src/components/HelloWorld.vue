<template>
  <n-calendar v-model:value="value">
    <template v-slot:date="{ year, month, date }">
      <div @click="handleDateClick(year, month, date)">
        {{ year }}-{{ month }}-{{ date }}
      </div>
    </template>
  </n-calendar>
</template>

<script lang="ts">
export default {
  data() {
    return {
      value: null
    }
  },
  methods: {
    handleDateClick(year: number, month: number, date: number) {
      console.log(`Clicked on date: ${year}-${month}-${date}`);
      // 在這裡添加你的事件處理邏輯
    }
  }
}
</script>
