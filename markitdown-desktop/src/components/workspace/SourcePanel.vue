<script setup lang="ts">
import { FileText, HardDrive, Calendar, FileType } from 'lucide-vue-next';
import { Separator } from '@/components/ui/separator';
import { ScrollArea } from '@/components/ui/scroll-area';
import { useAppStore } from '@/lib/composables/useAppStore';

const { state } = useAppStore();

function formatSize(bytes: number | null): string {
  if (bytes === null) return '未知';
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function fileExtension(filename: string): string {
  const parts = filename.split('.');
  return parts.length > 1 ? parts.pop()!.toUpperCase() : '未知';
}
</script>

<template>
  <ScrollArea class="h-full">
    <div class="p-4" v-if="state.currentConversion">
      <h3 class="mb-4 text-xs font-semibold uppercase tracking-wider text-zinc-500">
        源文件信息
      </h3>

      <div class="space-y-3">
        <div class="flex items-start gap-2">
          <FileText class="mt-0.5 h-3.5 w-3.5 shrink-0 text-zinc-400" />
          <div>
            <p class="text-[10px] text-zinc-400">文件名</p>
            <p class="text-xs font-medium text-zinc-900 break-all">
              {{ state.currentConversion.filename }}
            </p>
          </div>
        </div>

        <Separator />

        <div class="flex items-start gap-2">
          <FileType class="mt-0.5 h-3.5 w-3.5 shrink-0 text-zinc-400" />
          <div>
            <p class="text-[10px] text-zinc-400">格式</p>
            <p class="text-xs font-medium text-zinc-900">
              {{ fileExtension(state.currentConversion.filename) }}
            </p>
          </div>
        </div>

        <Separator />

        <div class="flex items-start gap-2">
          <HardDrive class="mt-0.5 h-3.5 w-3.5 shrink-0 text-zinc-400" />
          <div>
            <p class="text-[10px] text-zinc-400">大小</p>
            <p class="text-xs font-medium text-zinc-900">
              {{ formatSize(state.currentConversion.file_size) }}
            </p>
          </div>
        </div>

        <Separator />

        <div class="flex items-start gap-2">
          <Calendar class="mt-0.5 h-3.5 w-3.5 shrink-0 text-zinc-400" />
          <div>
            <p class="text-[10px] text-zinc-400">状态</p>
            <p class="text-xs font-medium" :class="state.currentConversion.status === 'completed'
              ? 'text-zinc-900' : 'text-red-500'">
              {{ state.currentConversion.status === 'completed' ? '转换完成' : '转换失败' }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </ScrollArea>
</template>
