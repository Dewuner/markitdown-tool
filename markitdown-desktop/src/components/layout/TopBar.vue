<script setup lang="ts">
import { Star } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip';

defineProps<{
  title?: string;
}>();
</script>

<template>
  <header class="flex h-12 items-center justify-between border-b border-zinc-200 bg-white px-4">
    <span class="text-sm font-semibold tracking-tight text-zinc-900">
      {{ title || 'MarkItDown Desktop' }}
    </span>
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
