import { ref } from "vue";
import type { ChatProfile, ChatMessage, SidebarUser } from "../types/chat";

/**
 * Central messenger state + actions.
 *
 * Seed data lives here (not in App) so components stay dumb and the app root
 * stays clean. When a real backend arrives, swap the seeds below for fetches
 * and the components don't change.
 */

const users = ref<SidebarUser[]>([
  { id: 1, name: "Alice", avatar: "https://i.pravatar.cc/64?img=1" },
  { id: 2, name: "Bob", avatar: "https://i.pravatar.cc/64?img=2" },
  { id: 3, name: "Charlie", avatar: "https://i.pravatar.cc/64?img=3" },
]);

const profile = ref<ChatProfile>({
  avatar: "https://i.pravatar.cc/64?img=1",
  name: "Alice",
  status: "Online",
});

const messages = ref<ChatMessage[]>([
  { id: 1, text: "Hey, how are you?", mine: false, time: "09:24" },
  { id: 2, text: "Doing great — working on Butterfly 🦋", mine: true, time: "09:25" },
  { id: 3, text: "Nice! Can't wait to try it.", mine: false, time: "09:26" },
]);

export function useMessenger() {
  function sendMessage(text: string) {
    messages.value.push({
      id: Date.now(),
      text,
      mine: true,
      time: new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
    });
  }

  return { users, profile, messages, sendMessage };
}
