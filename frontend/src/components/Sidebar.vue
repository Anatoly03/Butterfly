<script setup lang="ts">
import SidebarUser from "./SidebarUser.vue";
import SidebarSearch from "./SidebarSearch.vue";

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
    <SidebarSearch />

    <SidebarUser
      v-for="user in users"
      :key="user.id ?? user.name"
      :user="user"
    />
  </div>
</template>
