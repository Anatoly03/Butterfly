<script setup lang="ts">
import SidebarUser from "./SidebarUser.vue";

type User = {
  id?: string | number;
  avatar: string;
  name: string;
};

withDefaults(defineProps<{ users?: User[] }>(), {
  users: () => [],
});
</script>

<template>
  <div class="sidebar">
    <slot name="sidebar"></slot>

    <SidebarUser
      v-for="user in users"
      :key="user.id ?? user.name"
      :user="user"
    />
  </div>
</template>
