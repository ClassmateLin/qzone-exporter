<script setup lang="ts">
import { shell } from "@tauri-apps/api"
import { RouterLink, RouterView } from "vue-router";
const onClickQZone = () => {
  shell.open('https://github.com/ClassmateLin/qzone-exporter')
}
</script>

<template>
  <div class="navbar bg-base-100">
    <div class="navbar-start">
      <a class="btn btn-ghost text-xl avatar c" @click="onClickQZone">qzone-exporter</a>
    </div>
    <div class="navbar-center hidden lg:flex">
      <ul class="menu menu-horizontal px-1">
        <li>
          <RouterLink to="/photo">相册</RouterLink>
        </li>
        <li>
          <RouterLink to="/task">任务</RouterLink>
        </li>
      </ul>
    </div>
    <div class="navbar-end">
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
          <div class="w-10 rounded-full">
            <img alt="Tailwind CSS Navbar component" src="https://avatars.githubusercontent.com/u/53087849?s=96&v=4" />
          </div>
        </div>
        <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
          <li><a>设置</a></li>
          <li><a>退出</a></li>
        </ul>
      </div>
    </div>
  </div>
  <div class="flex">
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" :key="$route.name" v-if="$route.meta.keepAlive" />
      </keep-alive>
      <component :is="Component" :key="$route.name" v-if="!$route.meta.keepAlive" />
    </router-view>
  </div></template>
<style scoped></style>
