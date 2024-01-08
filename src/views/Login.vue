<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { ref, onMounted } from 'vue'
import { emit } from '@tauri-apps/api/event'
import EventName from '../constant/event';
import { save_store, set_value } from '../store';
import { toast } from 'vue3-toastify';
type QRCode = {
  qrcode_data: string, // 二维码图片保存路径
  qrsig: string,       // 二维码签名
  ptqrtoken: string,   // 二维码token  path
}

enum QRCodeResultCode {
  Valid = 'valid',     // 二维码未失效
  Expired = 'Expired',   // 二维码已失效
  VERIFYING = ' VERIFYING', // 二维码认证中
  Success = 'Success',   // 二维码认证成功
  Unknown = 'Unknown',   // 二维码状态未知
}

type QRCodeLoginResult = {
  code: QRCodeResultCode,
  msg: string,
  data: string | null,
}

const IsQRCodeLoading = ref(true)

const qrcode_data = ref("")

const status_msg = ref("");

onMounted(async () => {
  try {
    const qrcode: QRCode = await invoke("get_login_qrcode")

    qrcode_data.value = 'data:image/png;base64,' + qrcode.qrcode_data;

    IsQRCodeLoading.value = false

    const interval_id = setInterval(async () => {
      const login_res: QRCodeLoginResult = await invoke("get_login_result", { qrcode: qrcode })
      status_msg.value = login_res.msg
      console.log("登录结果:", login_res)

      if (login_res.code == QRCodeResultCode.Success) {
        console.log("登录成功:", login_res.data)
        await set_value("cookie", login_res.data)
        await save_store()
        await emit(EventName.LOGIN_SUCCESS, {
          name: 'hello'
        })
        clearInterval(interval_id)
      } else if (login_res.code == QRCodeResultCode.Expired || login_res.code == QRCodeResultCode.Unknown) {
        IsQRCodeLoading.value = true
        const qrcode: QRCode = await invoke("get_login_qrcode")
        qrcode_data.value = 'data:image/png;base64,' + qrcode.qrcode_data;
        IsQRCodeLoading.value = false
      }
    }, 2 * 1000)
  } catch (e: any) {
    toast.error(e)
  }


})


</script>

<template>
  <div class="card bg-base">
    <div class="card-body">
      <label class="label card-title justify-center">请使用手机版QQ扫码</label>

      <label v-if="!IsQRCodeLoading" class="label justify-center"><img :src="qrcode_data"></label>
      <label v-else class="label justify-center"><span class="loading loading-spinner loading-lg"></span></label>

      <label class="label justify-center">{{ status_msg }}</label>
    </div>
  </div>
</template>