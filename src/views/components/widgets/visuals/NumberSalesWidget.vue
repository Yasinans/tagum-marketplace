<script setup lang="ts">
import {
    PresentationChartLineIcon,
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
            <presentation-chart-line-icon class="h-6" />
          </div>
          <div class="stat-title">Number of Sales</div>
          <div class="stat-value text-[24px]">{{analyticsData.totalSales}}</div>
          <div class="stat-desc"></div>
        </div>
      </div>
</template>