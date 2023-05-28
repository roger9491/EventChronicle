<template>
  <n-space vertical>
    <n-form ref="formRef" inline :label-width="80">
      <!-- 搜索提示 -->
      <!-- 輸入組件 -->
      <n-auto-complete v-model:value="formTitle" placeholder="標籤" :input-props="{
        autocomplete: 'disabled'
      }" :options="computedOptions" />

      <!-- 時間範圍選擇 -->
      <n-time-picker v-model:value="formStartTime" />
      <n-time-picker v-model:value="formEndTime" />
      <n-date-picker v-model:value="formEventDate" type="date" />
    </n-form>
    <n-button round type="primary" @click="formButton(formTitle, formStartTime, formEndTime, formEventDate)">
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
// invoke 調用 rust函式
import { invoke } from '@tauri-apps/api/tauri';
import { reactive } from 'vue';

export default {
  // 頁面變數
  data() {
    return {
      formStartTime: 0,
      formEndTime: 0,
      formEventDate: 0,  // 表單事件的日期
      formTitle: "",  // 表單事件的名稱

      defaultTime: [],   // 默認時間
      labelList: ["java", "k8s", "cs162"],   // 已記錄得事件
      value: null,
      originalEventData: {},  // 原始事件的數據
      eventDataObject: reactive({} as {
        [year: number]: {
          [month: number]: {
            [date: number]: [{
              startTime: number,
              endTime: number,
              title: string,
            }
            ]
          }
        },
      }),
      key: 0

    }
  },
  created() {
    // 初始化表單數值
    this.initializeForm()

    // 在這裡做一些事情，例如從服務器獲取數據
    console.log("created");

    // 取得原始事件的數據
    invoke("response_event_data")
      .then(res => {
        console.log("asda", res as string);
        // 轉換成對象格式
        let jsonData = JSON.parse(res as string);

        // 獲取到的原始數據添加進頁面數據結構
        for (let event of jsonData) {
          this.originAddEventDataObject(event.startTime, event.endTime, event.title);
        }


      })
      .catch(err => {
        console.log("err", err);
      });


  },
  computed: {
    computedOptions() {
      let label = []
      for (let i = 0; i < this.labelList.length; i++) {
        // title 符合前綴
        if (this.labelList[i].startsWith(this.formTitle)) {
          label.push(this.labelList[i])
        }
      }
      return label
    },
    // 計算當前時間為默認時間
    defaultTime(): string[] {
      return ['00:00:00', this.getCurrentTimeWithZeroSeconds()];
    },
  },
  // 事件
  methods: {
    // 表單送出按鈕
    formButton(title: string, startTimestamp: number, endTimestamp: number, dateTimestamp: number) {
      // format timeStamp
      // 取得指定日起的起始結束時間
      // 建立最後儲存的時間物件
      let finalStartDateTmp = new Date(dateTimestamp)
      let finalEndDateTmp = new Date(dateTimestamp)


      let startDateTmp = new Date(startTimestamp)
      let endDateTmp = new Date(endTimestamp)
      // 設置時間部分
      finalStartDateTmp.setHours(
        startDateTmp.getHours(),
        startDateTmp.getMinutes(),
        startDateTmp.getSeconds(),
        startDateTmp.getMilliseconds()
      )
      finalEndDateTmp.setHours(
        endDateTmp.getHours(),
        endDateTmp.getMinutes(),
        endDateTmp.getSeconds(),
        endDateTmp.getMilliseconds()
      )


      //驗證事件時間有沒有重複



      // recordEvent()
    },
    /* 
      紀錄事件
      1. 該變當前頁面數據
      2. 發送數據給rust並記錄成json格式
    */
    recordEvent(start: Number, end: number, t: string, eventDate: number) {
      console.log("recordEvent", eventDate)
      // 驗證時間有沒有重複



      // 該變當前頁面數據
      // eventDataObject 添加數據
      // this.originAddEventDataObject(start, end, t)
      console.log("recordEvent eventDataObject", this.eventDataObject);

      let obj = {
        startTime: start,
        endTime: end,
        title: t,
      }
      let s = JSON.stringify(obj)

      // test
      console.log(s)

      // 呼叫 rust api
      // 持久化事件到文件
      invoke('EventStoreToFile', { dataJson: s }).then(response => {
        console.log(response) // logs "a response from Rust"
      })




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
            return true
          }
        }
        return false
      }
    },

    /* 
      start: 開始的時間戳
      end: 結束的時間戳
      tit: 事件名稱
    */
    // 原始數據格式並添加到 eventDataObject
    originAddEventDataObject(start: number, end: number, tit: string) {
      // eventDataObject 添加數據
      let startDate = new Date(start)
      let endDate = new Date(end)

      let obj = {
        startTime: start,
        endTime: end,
        title: tit
      }
      for (let year = startDate.getFullYear(); year <= endDate.getFullYear(); year++) {
        // new Date 月份從零開始
        for (let month = startDate.getMonth() + 1; month <= endDate.getMonth() + 1; month++) {
          for (let date = startDate.getDate(); date <= endDate.getDate(); date++) {
            if (!this.eventDataObject[year]) {
              this.eventDataObject[year] = reactive({})
            }
            if (!this.eventDataObject[year][month]) {
              this.eventDataObject[year][month] = reactive({})
            }
            if (!this.eventDataObject[year][month][date]) {
              // 如果沒有 初始化陣列即建立
              this.eventDataObject[year][month][date] = [obj]
            } else {
              //  有初始化陣列即 push
              this.eventDataObject[year][month][date].push(obj)
            }

          }
        }
      }

    },

    // 初始化表單
    initializeForm() {
      // 初始化日期為當前
      this.formEventDate = Date.now();
      // 清空輸入框
      this.formTitle;
    },
    /* 
      前提是必須確保當 年 月 日 有其值
    */
    // 檢查紀錄的時間是否有重複到
    checkEventTimeDuplication(
      year: number, 
      month: number,
      date: number,
      startTimestamp: number,
      endTimestamp: number,
      event: string) {

      // 排序事件
      this.eventDataObject[year][month][date].sort(this.compareByTimeStamp)


      // 檢查 開始結束 時間是否有重疊
      let flag = true; 
      for (let eventInfo of this.eventDataObject[year][month][date]){
        
      }

      if (flag) {
        return true
      }else{
        return false
      }

    },
    compareByTimeStamp(a: any, b: any) {
      if (a.startTime > b.startTime){
        return 1;
      }
      if (a.startTime == b.startTime){
        if (a.endTime > b.endTime){
          return 1;
        }
      }
      return 0
    }
  }
}

</script>