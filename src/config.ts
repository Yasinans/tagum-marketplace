import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

interface Config {
  api_url: string;
}
//export const API_BASE_URL = ref("http://localhost:3000");

//EXAMPLE API test ignore
export const API_BASE_URL = ref("https://tagum-marketplace-api-web-production.up.railway.app");

export async function loadConfig() {
  try {
    const config: Config = JSON.parse(await invoke("get_config"));
    API_BASE_URL.value = config.api_url; 
    console.log("Loaded API URL:", API_BASE_URL.value);
  } catch (error) {
    console.error("Failed to load config:", error);
  }
}

