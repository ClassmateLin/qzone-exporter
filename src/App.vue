<script setup lang="ts">

import { onMounted } from 'vue';
import { createWin, getWin } from "./window/actions";
import Windows from './window'
import { Event } from '@tauri-apps/api/event';
import router from './router';
import EventName from './constant/event';
import { get_value, save_store, set_value } from './store'
import { invoke } from '@tauri-apps/api';

onMounted(async () => {
  await set_value('cookie', null)
  await save_store()
  const cookie = await get_value('cookie')
  console.log(cookie)
  if (cookie) {

    const res = await invoke('get_user_info', { 'ck': cookie })
    console.log(res)
    if (res) {
      router.push('layout')
      return
    }
  }

  const win_lable = 'login';
  const window = new Windows()
  await window.listen()

  await createWin({
    label: win_lable,
    url: '#login',
    title: "QQ空间登录"
  });

  window.getWin('main')?.once(win_lable + EventName.CREATED_SUCCESS_SUFFIX, async () => {
    console.log('sss');
    const login_win = await getWin('login')
    await login_win?.listen(EventName.LOGIN_SUCCESS, async (event: Event<any>) => {
      window.getWin('main')?.emit(EventName.LOGIN_SUCCESS, event.payload)
      await login_win.close()
    })
  })

  window.getWin('main')?.once(EventName.LOGIN_SUCCESS, async (event: Event<any>) => {
    console.log('主窗口监听登录成功: ', event)
    router.push('layout')
  })


})
</script>

<template>
  <router-view></router-view>
</template>