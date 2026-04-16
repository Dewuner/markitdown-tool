<script setup lang="ts">
import { useAppStore } from '@/lib/composables/useAppStore';
import FileDropZone from '@/components/conversion/FileDropZone.vue';
import ConversionProgress from '@/components/conversion/ConversionProgress.vue';
import SplitPaneWorkspace from '@/components/workspace/SplitPaneWorkspace.vue';

const { state } = useAppStore();
</script>

<template>
  <main class="flex flex-1 flex-col overflow-hidden">
    <ConversionProgress v-if="state.status === 'converting'" />
    <FileDropZone v-if="!state.currentConversion && state.status === 'idle'" />
    <SplitPaneWorkspace v-else-if="state.currentConversion" />
  </main>
</template>
