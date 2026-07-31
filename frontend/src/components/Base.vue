<script setup lang="ts">
import Sidebar from "./Sidebar.vue";
import Content from "./Content.vue";
import type { SidebarUser, ChatProfile, ChatMessage } from "../types/chat";

withDefaults(
  defineProps<{
    users?: SidebarUser[];
    profile?: ChatProfile;
    messages?: ChatMessage[];
  }>(),
  { users: () => [], profile: undefined, messages: () => [] },
);

defineEmits<{ (e: "send", text: string): void }>();
</script>

<template>
  <main class="messenger-layout">
    <aside class="messenger-sidebar" aria-label="Messenger sidebar">
      <header class="messenger-sidebar_header">
        <slot name="sidebar-header">
          <h1 class="messenger-title">Butterfly</h1>
        </slot>
      </header>

      <section class="messenger-sidebar_content">
        <Sidebar :users="users" />
      </section>
    </aside>

    <section class="messenger-main" aria-label="Messenger main area">
      <Content
        :profile="profile"
        :messages="messages"
        @send="$emit('send', $event)"
      />
    </section>
  </main>
</template>
