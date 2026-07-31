<script setup lang="ts">
import Sidebar from "./Sidebar.vue";
import Content from "./Content.vue";
import ProfileHeader from "./ProfileHeader.vue";
import type { SidebarUser, ChatProfile, ChatMessage } from "../types/chat";

withDefaults(
  defineProps<{
    me?: ChatProfile;
    users?: SidebarUser[];
    profile?: ChatProfile;
    messages?: ChatMessage[];
  }>(),
  { me: undefined, users: () => [], profile: undefined, messages: () => [] },
);

defineEmits<{ (e: "send", text: string): void; (e: "settings"): void }>();
</script>

<template>
  <main class="messenger-layout">
    <aside class="messenger-sidebar" aria-label="Messenger sidebar">
      <ProfileHeader v-if="me" :profile="me" />

      <section class="messenger-sidebar_content">
        <Sidebar :users="users" />
      </section>

      <footer class="flex items-center h-16 px-3 border-t border-line bg-surface">
        <button
          type="button"
          class="w-full flex items-center gap-2 p-2 rounded-lg hover:bg-gray-100 cursor-pointer"
          @click="$emit('settings')"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="w-5 h-5 text-muted"
            aria-hidden="true"
          >
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
          <span>Settings</span>
        </button>
      </footer>
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
