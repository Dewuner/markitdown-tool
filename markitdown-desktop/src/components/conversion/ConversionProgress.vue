<script setup lang="ts">
import { Progress } from '@/components/ui/progress';
import { useAppStore } from '@/lib/composables/useAppStore';

const { state } = useAppStore();
</script>

<template>
  <div class="flex items-center gap-3 border-b border-zinc-200 bg-white px-4 py-2">
    <span class="text-xs text-zinc-500">转换中...</span>
    <Progress
      :model-value="state.batchProgress.total > 0
        ? (state.batchProgress.current / state.batchProgress.total) * 100
        : 0"
      class="h-1.5 flex-1"
    />
    <span class="text-xs text-zinc-400">
      {{ state.batchProgress.current }} / {{ state.batchProgress.total }}
    </span>
  </div>
</template>
