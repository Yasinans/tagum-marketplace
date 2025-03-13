<template>
  <main class="dashboard-container">
    <div class="tg-header">
      <p>Dashboard</p>
      <p>{{ new Date().toLocaleString('default', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' }) }}</p>
    </div>
    <div class="dashboard-1">
      <total-earning-widget/>
      <number-sales-widget/>
      <total-products-widget/>
      <total-customers-widget/>
    </div>
    <div class="dashboard-2">
      <sales-trends-widget/>
    </div>
    <div class="dashboard-3">
      <recent-sales-widget/>
    </div>
  </main>
</template>

<style scoped>
.d-tooltip-item {
  height: max-content;
  position: relative;
  flex-direction: column;
  width: min-content;
}

.d-tooltip-item .d-tooltip {
  border-radius: 5px;
  margin-left: -10px;
  margin-top: 2px;
  padding: 5px;
  background-color: rgb(31, 25, 25);
  visibility: hidden;
  position: absolute;
  width: 120px;
  z-index: 1;
}

.d-tooltip-item:hover {
  text-decoration: dashed underline 1px;
  text-underline-offset: 2px;
}

.d-tooltip-item:hover .d-tooltip {
  visibility: visible;
}


.d-table {
  background-color: #5a3939;
  border-collapse: collapse;
  width: 100%;
  box-shadow: inset -12px -8px 40px #421f1f5e;
}

.d-table thead td {
  padding: 13px 14px 13px 12px;
  font-size: 14px;
  color: rgb(245, 245, 245);
  font-weight: 500;
}


.d-table tbody td {
  font-size: 15px;
  padding: 12px 0 12px 14px;
  border-top: 1px solid rgb(124, 90, 90);
  font-variation-settings:
    "wdth" 75;
}

.dashboard-container {
  display: flex;
  text-wrap: nowrap;
  flex-direction: column;
  gap: 20px;
  padding: 26px;
}

.widget {
  user-select: none;
  flex: 1 1;
  color: #FFF;
  border-radius: 13px;
  background-color: #744e4e;
  padding: 20px;
  padding-top: 12px;
  height: max-content;

}

.dashboard-container>div {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-start;
  gap: 20px;

}

.dashboard-2-1,
.dashboard-3-1,
.dashboard-3-2 {
  height: max-content;
}

.dashboard-wHead {
  display: flex;
  margin-bottom: 5px;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  font-variation-settings:
    'wdth' 75;
}


.dashboard-2-1-1>div:nth-of-type(1) {
  font-size: 16px;
  font-weight: 700;
}

.d-button {
  border: 1px solid rgba(43, 39, 39, 0.404);
  border-radius: 10px;
  transition: all 300ms;
}

.d-button:hover {
  background-color: rgba(43, 39, 39, 0.404);
}

.d-button>.icon {
  height: 20px;
  width: 20px;
  margin: 6px;

}
</style>