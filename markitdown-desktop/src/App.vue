<script setup lang="ts">
import { onMounted } from 'vue';
import MainView from '@/views/MainView.vue';
import { useAppStore } from '@/lib/composables/useAppStore';
import { getHistory } from '@/lib/tauri';

const { setHistory } = useAppStore();

onMounted(async () => {
  try {
    const result = await getHistory();
    if (result.success && result.data) {
      setHistory(result.data);
    }
  } catch {
    // DB not ready yet, ignore
  }
});
</script>

<template>
  <MainView />
</template>
