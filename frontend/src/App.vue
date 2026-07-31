<script setup lang="ts">
import Base from "./components/Base.vue";
import { useMessenger } from "./composables/useMessenger";

const { users, profile, messages, sendMessage } = useMessenger();
</script>

<template>
  <Base
    :users="users"
    :profile="profile"
    :messages="messages"
    @send="sendMessage"
  />
</template>
