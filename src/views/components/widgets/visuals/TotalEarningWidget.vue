<script setup lang="ts">
import {
  BanknotesIcon,
} from '@heroicons/vue/24/solid';
import {
  useAnalytics
} from '../../../../composable/useAnalytics';

const {
  analyticsData
} = useAnalytics();

</script>

<template>
<div class="stats  shadow-md flex-auto">
    <div class="stat">
      <div class="stat-figure">
        <banknotes-icon class="h-6" />
      </div>
      <div class="stat-title">Total Earnings</div>
      <div class="stat-value text-[24px]">PHP {{ analyticsData.totalEarnings.TotalAmount ?? 0}}</div>
      <div class="stat-desc">
        {{ new Date(analyticsData.totalEarnings.StartDate).toLocaleString('default', { month: 'long', day: 'numeric', year: 'numeric' }) }} 
        - {{ new Date(analyticsData.totalEarnings.EndDate).toLocaleString('default', { month: 'long', day: 'numeric', year: 'numeric' }) }}</div>
    </div>
  </div>
</template>