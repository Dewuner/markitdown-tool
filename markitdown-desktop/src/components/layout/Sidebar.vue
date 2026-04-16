<script setup lang="ts">
import { PanelLeftClose, PanelLeftOpen, Trash2 } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { useAppStore } from '@/lib/composables/useAppStore';

const { state, toggleSidebar, selectHistory, removeFromHistory, selectedHistoryId } = useAppStore();

function formatDate(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleDateString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function formatSize(bytes: number | null): string {
  if (bytes === null) return '';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function statusVariant(status: string): 'default' | 'secondary' | 'destructive' {
  if (status === 'completed') return 'default';
  if (status === 'error') return 'destructive';
  return 'secondary';
}
</script>

<template>
  <aside
    class="flex flex-col border-r border-zinc-200 bg-zinc-50 transition-all duration-200"
    :class="state.sidebarCollapsed ? 'w-12' : 'w-64'"
  >
    <div class="flex h-10 items-center justify-between px-2">
      <span v-if="!state.sidebarCollapsed" class="px-2 text-xs font-medium text-zinc-500">
        历史记录
      </span>
      <Button variant="ghost" size="icon" class="h-7 w-7" @click="toggleSidebar">
        <PanelLeftClose v-if="!state.sidebarCollapsed" class="h-4 w-4" />
        <PanelLeftOpen v-else class="h-4 w-4" />
      </Button>
    </div>
    <Separator />
    <ScrollArea v-if="!state.sidebarCollapsed" class="flex-1">
      <div class="space-y-1 p-2">
        <div
          v-for="entry in state.history"
          :key="entry.id"
          class="group cursor-pointer rounded-md p-2 transition-colors hover:bg-zinc-100"
          :class="selectedHistoryId === entry.id ? 'bg-zinc-100' : ''"
          @click="selectHistory(entry.id)"
        >
          <div class="flex items-start justify-between gap-1">
            <div class="min-w-0 flex-1">
              <p class="truncate text-xs font-medium text-zinc-900">
                {{ entry.filename }}
              </p>
              <p class="mt-0.5 text-[10px] text-zinc-400">
                {{ formatDate(entry.created_at) }}
                <span v-if="entry.file_size" class="ml-1">{{ formatSize(entry.file_size) }}</span>
              </p>
            </div>
            <Badge :variant="statusVariant(entry.status)" class="shrink-0 text-[10px]">
              {{ entry.status === 'completed' ? '完成' : entry.status === 'error' ? '失败' : '处理中' }}
            </Badge>
          </div>
          <Button
            variant="ghost"
            size="icon"
            class="mt-1 hidden h-5 w-5 text-zinc-400 hover:text-red-500 group-hover:flex"
            @click.stop="removeFromHistory(entry.id)"
          >
            <Trash2 class="h-3 w-3" />
          </Button>
        </div>
        <div v-if="state.history.length === 0" class="py-8 text-center text-xs text-zinc-400">
          暂无转换记录
        </div>
      </div>
    </ScrollArea>
  </aside>
</template>
