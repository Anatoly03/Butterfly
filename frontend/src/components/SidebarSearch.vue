<script setup lang="ts">
import { MagnifyingGlassIcon, XMarkIcon } from "@heroicons/vue/24/outline";

const query = defineModel<string>({ default: "" });
</script>

<template>
    <div class="relative w-75 mb-4">
        <MagnifyingGlassIcon
            class="absolute left-3 top-1/2 -translate-y-1/2 size-5 text-muted pointer-events-none"
        />
        <input
            v-model="query"
            type="text"
            placeholder="Search..."
            class="w-full py-2 pl-10 pr-10 rounded-[60px] border border-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
            v-if="query"
            type="button"
            aria-label="Clear search"
            class="absolute right-2 top-1/2 -translate-y-1/2 grid place-items-center size-6 rounded-full text-muted cursor-pointer hover:bg-gray-100 hover:text-ink outline-none focus-visible:ring-2 focus-visible:ring-blue-500"
            @click="query = ''"
        >
            <XMarkIcon class="size-6" />
        </button>
    </div>
</template>
