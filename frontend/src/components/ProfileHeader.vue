<script setup lang="ts">
import type { ChatProfile } from "../types/chat";

defineProps<{ profile: ChatProfile }>();
</script>

<template>
  <header class="flex items-center gap-3 px-5 py-3 bg-surface border-b border-line">
    <img :src="profile.avatar" alt="" class="w-10 h-10 rounded-full" />
    <div class="flex flex-col leading-tight">
      <span class="font-semibold">{{ profile.name }}</span>
      <span v-if="profile.status" class="text-sm text-muted">{{ profile.status }}</span>
    </div>
  </header>
</template>
