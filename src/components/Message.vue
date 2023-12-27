<template>
  <div>
    <button @click="showMessage">显示消息</button>
    <div v-if="show">
      <p>{{ message }}</p>
      <button @click="closeMessage">关闭</button>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  data() {
    return {
      show: false,
      message: ''
    }
  },
  methods: {
    showMessage() {
      // 调用 Tauri 的弹框 API，并设置消息内容
      this.message = '这是一条消息'
      this.show = true
    },
    closeMessage() {
      this.show = false
    }
  }
}
</script>