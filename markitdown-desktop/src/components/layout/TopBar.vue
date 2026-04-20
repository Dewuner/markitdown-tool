<script setup lang="ts">
import { Star, Plus } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';
import { useAppStore } from '@/lib/composables/useAppStore';

defineProps<{
  title?: string;
}>();

const { state, setStatus, setCurrentConversion } = useAppStore();

function resetToHome() {
  setCurrentConversion(null);
  setStatus('idle');
}
</script>

<template>
  <header class="flex h-12 items-center justify-between border-b border-zinc-200 bg-white px-4">
    <div class="flex items-center gap-2">
      <Button
        v-if="state.currentConversion"
        variant="ghost"
        size="sm"
        class="h-7 gap-1 text-xs text-zinc-500"
        @click="resetToHome"
      >
        <Plus class="h-3.5 w-3.5" />
        新建转换
      </Button>
      <span class="text-sm font-semibold tracking-tight text-zinc-900">
        {{ title || 'MarkItDown Desktop' }}
      </span>
    </div>
    <TooltipProvider :delay-duration="300">
      <Tooltip>
        <TooltipTrigger as-child>
          <Button variant="ghost" size="icon" class="h-8 w-8 text-zinc-400 hover:text-zinc-900">
            <Star class="h-4 w-4" />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="bottom" class="text-xs">
          AI 智能排版（即将推出）
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  </header>
</template>
