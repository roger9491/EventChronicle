<template>
  <n-space vertical>
    <n-form ref="formRef" inline :label-width="80">
      <!-- 搜索提示 -->
      <!-- 輸入組件 -->
      <n-auto-complete v-model:value="title" placeholder="標籤" :input-props="{
        autocomplete: 'disabled'
      }" :options="computedOptions" />

      <!-- 時間範圍選擇 -->
      <n-date-picker v-mode:value="range" type="datetimerange" @confirm="onConfirm" :default-time="defaultTime" />
    </n-form>
    <n-button round type="primary" @click="recordEvent(range, title)">
      送出
    </n-button>

    <n-calendar v-model:value="value">
      <template v-slot:default="{ year, month, date }">
        <template v-if="isHaveEvent(year, month, date)">
          <div v-for="event in eventDataObject[year][month][date]">
            <div> {{ event.title }}</div>
          </div>
        </template>
      </template>
    </n-calendar>
  </n-space>
</template>

<script lang="ts">
import { reactive } from 'vue';
export default {
  // 頁面變數
  data() {
    return {
      range: [],  // 紀錄事件的時間範圍
      defaultTime: [],   // 默認時間
      title: "",  // 紀錄事件的名稱
      labelList: ["java", "k8s", "cs162"],   // 已記錄得事件
      value: null,
      originalEventData: {},  // 原始事件的數據
      eventDataObject: reactive({} as {
        [year: number]: {
          [month: number]: {
            [date: number]: any[]
          }
        },
      }),

    }
  },
  created() {
    // 在這裡做一些事情，例如從服務器獲取數據

    // 獲取原始數據

    // 轉換成對象格式
    this.eventDataObject = {
      2023: {
        5: {
          21: [
            {
              startTime: 1,
              title: "java"
            }
          ]
        }
      }
    }

  },
  computed: {
    computedOptions() {
      let label = []
      for (let i = 0; i < this.labelList.length; i++) {
        // title 符合前綴
        if (this.labelList[i].startsWith(this.title)) {
          label.push(this.labelList[i])
        }
      }
      return label
    },
    // 計算當前時間為默認時間
    defaultTime(): string[] {
      return ['00:00:00', this.getCurrentTimeWithZeroSeconds()];
    },
    // isHaveEvent(year: number, month: number, date: number): boolean {
    //   if (year in this.eventDataObject) {
    //     if (month in this.eventDataObject[year]) {
    //       if (date in this.eventDataObject[year][month]) {
    //         return date in this.eventDataObject[year][month];
    //       }
    //     }
    //   }
    //   return false
    // },
  },
  // 事件
  methods: {
    onConfirm(range: any) {
      this.range = range

      // test
      console.log(this.range)
      console.log(range)
    },
    /* 
      紀錄事件
      1. 該變當前頁面數據
      2. 發送數據給rust並記錄成json格式
    */
    recordEvent(range: never[], title: string) {
      console.log("recordEvent", range, title)
      // 該變當前頁面數據
      // eventDataObject 添加數據
      let startDate = new Date(range[0])
      let endDate = new Date(range[1])
      for (let year = startDate.getFullYear(); year <= endDate.getFullYear(); year++) {
        for (let month = startDate.getMonth(); month <= endDate.getMonth(); month++) {
          for (let date = startDate.getDate(); date <= endDate.getDate(); date++) {
            if (!this.eventDataObject[year]) {
              this.eventDataObject[year] =  reactive({})
            }
            if (!this.eventDataObject[year][month]) {
              this.eventDataObject[year][month] = reactive({})
            }
            this.eventDataObject[year][month][date] = [range, title]
          }
        }
      }

      console.log(this.eventDataObject)
      console.log(this.range, this.title)
    },
    getCurrentTimeWithZeroSeconds(): string {
      const now = new Date();
      return `${now.getHours()}:${now.getMinutes()}:00`;
    },
    handleDateClick(year: number, month: number, date: number) {
      console.log(`Clicked on date: ${year}-${month}-${date}`);
      // 在這裡添加你的事件處理邏輯
    },
    // 判斷紀錄事件
    isHaveEvent(year: number, month: number, date: number) {

      if (year in this.eventDataObject) {
        if (month in this.eventDataObject[year]) {
          if (date in this.eventDataObject[year][month]) {
            return date in this.eventDataObject[year][month];
          }
        }

        return false
      }
    },

  }
}

</script>