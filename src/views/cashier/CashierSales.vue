<script setup lang="ts">
</script>

<template>
  <main class="flex text-nowrap flex-col gap-[16px] p-[26px]">
    <div class="tg-header">
      <p>Sales</p>
      <p>{{ new Date().toLocaleString('default', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }) }}</p>
    </div>    <!--Sections-->
    <div class="flex flex-wrap justify-start gap-[20px]">
      <!--Main Section - Product-->
      <sales-widget class="w-full h-full" />
    </div>
  </main>
</template>