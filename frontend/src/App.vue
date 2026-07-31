<script setup lang="ts">
import Base from "./components/Base.vue";
import { useMessenger } from "./composables/useMessenger";

const { me, users, profile, messages, sendMessage, openSettings } = useMessenger();
</script>

<template>
  <Base
    :me="me"
    :users="users"
    :profile="profile"
    :messages="messages"
    @send="sendMessage"
    @settings="openSettings"
  />
</template>
