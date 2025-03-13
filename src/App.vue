<script setup lang="ts">
import { useRouter } from 'vue-router';
import { onMounted } from 'vue';


const router = useRouter();
onMounted(() => {
  router.push('/');
});
</script>

<template>
  <main class="app-container">
    <router-view />
  </main>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Roboto:ital,wdth,wght@0,75..100,100..900;1,75..100,100..900&display=swap');
@import "tailwindcss";
@plugin "daisyui";

body {
  overflow: hidden;
  font-family: "Roboto";
  padding: 0;
  margin: 0;
}

/*Custom CSS*/
thead td {
  position: sticky;
  top: 0;
  padding: 13px 14px 13px 20px;
  font-size: 14px;
  font-weight: 700;

}

tr {
  line-height: 15px;
}

tbody td {
  height: 10px;
  font-size: 14px;
  padding: 12px 0 12px 20px;
  color:rgb(48, 42, 42);
  border-top: 1px solid rgb(124, 90, 90);
  font-variation-settings:
    "wdth" 75;
}
tbody td:last-child {
  padding-right: 12px;
}

/*
Modify DaisyUI or unsa pa ba na
*/
.stats{
  background: #fff8ea;
}

.stat-figure{
  padding:10px;
  background-color:#f5e3db;
  border-radius:100px;
  box-shadow: inset 0 2px 3px rgb(0 0 0 / 0.1);
}

.stat-title, .stat-desc {
  color:#5a5353;
}

.stat-value {
  color:#332c2cfd;
}

/*Final CSS*/

.tg-header {
  display: flex;
  flex-direction: column;
  gap:0 !important;
  color:#363531;
}

.tg-header > p:nth-child(1) {
  font-size: 22px;
  font-weight: 500;
}

.tg-header > p:nth-child(2) {
  font-size: 12px;
  font-weight: light;
}
.tg-widget-c {
  border-radius: 7px;
  background-color:#997f7f;
  padding: 10px 6px 0px 6px;
  box-shadow: inset 0 1px 8px rgb(0 0 0 / 0.2);


}
.tg-table-container {
  width: 100%;
  height: 100%;
  overflow: auto;
  max-height: 600px;
  padding: 6px;
  border-radius: 7px;
}

.tg-main-table{
  max-height: none !important;
}
.tg-table {
  flex-grow: 1;
  border-collapse: collapse;
  width: 100%;
}
/*Custom Widget */
.tg-widget {
  flex: 1 1 auto ;
  overflow-x: hidden;
  user-select: none;
  display: flex;
  flex-direction: column;
  border-radius: 13px;
  /*background-color: #744e4e;*/
  background: #faf3e3;

  color:#353431;

  padding: 13px 20px 13px 20px;
  box-shadow: var(--shadow-md);
}

.tg-widget-h {
  display: flex;
  margin-bottom: 10px;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  font-variation-settings:
    'wdth' 75;
}

.tg-widget-h > div:nth-child(1) {
  margin-right:30px;
}
.tg-widget-btn {
  background-color: transparent;
  border: 1px solid rgba(43, 39, 39, 0.404);
  border-radius: 10px;
  transition: all 300ms;
}

.tg-widget-btn:hover {
  background-color: rgba(43, 39, 39, 0.404);
}

.tg-widget-btn-icon {
  height: 20px;
  width: 20px;
  margin: 6px;
}

.tg-modal-delete{
  width: 100%;
  background: #86131342;
  display:flex;
  justify-content: end;
  padding: 8px 12px 8px 12px;
}

.fieldset {
  color:black;
  border:none;
  background-color: #fff9ed
}
.fieldset div input,.fieldset div select {
  border:1px solid rgb(163, 138, 138);
  background-color: #ffffffb2;
}
.fieldset p{
  color:#2b2926;
}
/*Temporary Modal*/
.saveinfo {
  border-color: rgb(117, 117, 117);
  color: white;
  background-color: rgb(77, 74, 74);
}

.modal-box {
  color: rgb(8, 1, 1);
  font-size: 14px;
  background: #dfcec8 /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
}
.modal-operation {
  background: wheat;
}
.modal-box select,
.modal-box input {
  border: transparent;
  background-color: #ffffff;

}
.btn {
  border:rgb(131, 114, 114) solid 1px !important;
}
.badge{
  border-color:rgb(88, 70, 70);
}
</style>