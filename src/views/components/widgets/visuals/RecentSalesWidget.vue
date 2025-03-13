<script setup lang="ts">
import { ListBulletIcon, ArrowPathRoundedSquareIcon } from '@heroicons/vue/24/solid';
import {
  useAnalytics
} from '../../../../composable/useAnalytics';
import { ref, watch } from 'vue';

const {
  recentSalesData,
  loadRecentSales
} = useAnalytics();

const selectedRange = ref(0);

watch(selectedRange, () => {
  loadRecentSales(selectedRange.value);
});

</script>
<template>
    <div class="tg-widget">
        <div class="tg-widget-h">
            <div>
                Recent Sales
            </div>
            <div class="flex">
                <div @click="loadRecentSales(selectedRange)" class="tg-widget-btn mr-2 tooltip tooltip-left" data-tip="Refresh">
                    <arrow-path-rounded-square-icon class="tg-widget-btn-icon" />
                </div>
                <div class="dropdown dropdown-end tooltip tooltip-left" data-tip="Limit By">
                    <div tabindex="0" role="button" class="tg-widget-btn bg-transparent">
                        <list-bullet-icon class="tg-widget-btn-icon" />
                    </div>
                    <ul tabindex="0"
                        class="dropdown-content text-[#fff8ea] menu bg-[#1f1619] mt-[3px] rounded-box z-[1] w-30 shadow-lg">
                        <li><a @click="selectedRange = 5">5</a></li>
                        <li><a @click="selectedRange = 10">10</a></li>
                        <li><a @click="selectedRange = 25">25</a></li>
                        <li><a @click="selectedRange = 50">50</a></li>

                    </ul>
                </div>
            </div>
        </div>
        <div class="tg-table-container">
            <table class="tg-table">
                <thead>
                    <tr>
                        <td>Sales #</td>
                        <td>Customer</td>
                        <td>Total Quantity</td>
                        <td>Total Amount</td>
                        <td>Transaction Date</td>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="sale in recentSalesData" :key="sale.Sales_ID">
                        <td>{{sale.Sales_ID}}</td>
                        <td>{{sale.Customer_Name}}</td>
                        <td>{{sale.Total_Quantity}}</td>
                        <td>₱ {{sale.Total_Sales_Amount }}</td>
                        <td>{{new Date(sale.Sales_Date).toLocaleString('default', { month: 'long', day: 'numeric', year: 'numeric', hour: 'numeric', minute: 'numeric', hour12: true })}}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>