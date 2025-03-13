<script setup lang="ts">
import {
  UserGroupIcon,
} from '@heroicons/vue/24/solid';
import {
  useAnalytics
} from '../../../../composable/useAnalytics';

const {
  analyticsData
} = useAnalytics();

</script>

<template>
<div class="stats shadow-md flex-auto">
    <div class="stat">
      <div class="stat-figure">
        <user-group-icon class="h-6" />
      </div>
      <div class="stat-title ">Total Customers</div>
      <div class="stat-value  text-[24px]">{{ analyticsData.totalCustomers }} Customers</div>
    </div>
  </div>
</template>