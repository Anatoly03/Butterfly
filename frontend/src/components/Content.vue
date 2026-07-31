<script setup lang="ts">
import { ref } from "vue";
import type { ChatProfile, ChatMessage } from "../types/chat";

withDefaults(
  defineProps<{ profile?: ChatProfile; messages?: ChatMessage[] }>(),
  { profile: undefined, messages: () => [] },
);

const emit = defineEmits<{ (e: "send", text: string): void }>();

const draft = ref("");

function send() {
  const text = draft.value.trim();
  if (!text) return;
  emit("send", text);
  draft.value = "";
}
</script>

<template>
  <div class="grid grid-rows-[auto_minmax(0,1fr)_auto] min-h-0">
    <!-- Profile -->
    <header
      v-if="profile"
      class="flex items-center gap-3 px-5 py-3 bg-surface border-b border-line"
    >
      <img :src="profile.avatar" alt="" class="w-10 h-10 rounded-full" />
      <div class="flex flex-col leading-tight">
        <span class="font-semibold">{{ profile.name }}</span>
        <span v-if="profile.status" class="text-sm text-muted">{{ profile.status }}</span>
      </div>
    </header>

    <!-- Messages -->
    <div class="min-h-0 overflow-auto flex flex-col gap-2 px-5 py-4">
      <div
        v-for="(message, index) in messages"
        :key="message.id ?? index"
        class="flex flex-col max-w-[70%]"
        :class="message.mine ? 'self-end items-end' : 'items-start'"
      >
        <p
          class="m-0 px-3 py-2 rounded-2xl border"
          :class="message.mine
            ? 'bg-brand border-brand text-white'
            : 'bg-surface border-line'"
        >
          {{ message.text }}
        </p>
        <span v-if="message.time" class="mt-0.5 text-xs text-muted">{{ message.time }}</span>
      </div>
    </div>

    <!-- Input -->
    <form
      class="flex gap-2 px-5 py-3 bg-surface border-t border-line"
      @submit.prevent="send"
    >
      <input
        v-model="draft"
        type="text"
        class="flex-1 px-4 py-2.5 border border-line rounded-full outline-none focus:border-brand"
        placeholder="Type a message…"
        aria-label="Message"
      />
      <button
        type="submit"
        class="px-5 py-2.5 rounded-full bg-brand text-white font-semibold cursor-pointer hover:bg-brand-dark"
      >
        Send
      </button>
    </form>
  </div>
</template>
