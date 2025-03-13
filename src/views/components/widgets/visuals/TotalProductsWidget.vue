<script setup lang="ts">
import {
    ArchiveBoxIcon,
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
          <div class="stat-figure ">
            <archive-box-icon class="h-6" />
          </div>
          <div class="stat-title">Total Products</div>
          <div class="stat-value text-[24px]">{{analyticsData.totalProducts}}</div>
        </div>
      </div>
</template>