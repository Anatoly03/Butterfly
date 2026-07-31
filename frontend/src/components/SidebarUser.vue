<script setup lang="ts">
interface User {
  avatar: string;
  name: string;
}

defineProps<{ user: User }>();
</script>

<template>
  <div class="w-full flex items-center gap-2 p-2 hover:bg-gray-100 rounded-lg cursor-pointer">
    <img :src="user.avatar" alt="User Avatar" class="w-8 h-8 rounded-full" />
    <span>{{ user.name }}</span>
  </div>
</template>
