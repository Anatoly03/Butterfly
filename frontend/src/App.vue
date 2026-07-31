<script setup lang="ts">
import { ref } from "vue";
import Base from "./components/Base.vue";
// import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // greetMsg.value = await invoke("greet", { name: name.value });
    console.log('Greet:', name.value);
}
</script>

<template>
  <Base />
</template>
