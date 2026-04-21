<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { Upload, FileText } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { useAppStore } from '@/lib/composables/useAppStore';
import { convertFile, batchConvert, openFileDialog, getHistory } from '@/lib/tauri';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

const { state, setStatus, setCurrentConversion, setError, setBatchProgress, setHistory } =
  useAppStore();

const isDragging = ref(false);
const unlisteners: UnlistenFn[] = [];

onMounted(async () => {
  unlisteners.push(
    await listen('tauri://drag-enter', () => {
      isDragging.value = true;
    }),
    await listen('tauri://drag-leave', () => {
      isDragging.value = false;
    }),
    await listen<{ paths: string[] }>('tauri://drag-drop', async (e) => {
      isDragging.value = false;
      const paths = e.payload.paths;
      if (!paths || paths.length === 0) return;

      if (paths.length === 1) {
        await handleSingleFile(paths[0]);
      } else {
        await handleBatchFiles(paths);
      }
    }),
  );
});

onUnmounted(() => {
  unlisteners.forEach((fn) => fn());
});

async function handleSingleFile(filePath: string) {
  setStatus('converting');
  setError(null);
  try {
    const result = await convertFile(filePath);
    if (result.success && result.data) {
      setCurrentConversion(result.data);
      setStatus('completed');
      refreshHistory();
    } else {
      setError(result.error || '转换失败');
      setStatus('error');
    }
  } catch (err) {
    setError(String(err));
    setStatus('error');
  }
}

async function handleBatchFiles(filePaths: string[]) {
  setStatus('converting');
  setBatchProgress(0, filePaths.length);
  setError(null);
  try {
    const result = await batchConvert(filePaths);
    if (result.success && result.data?.data) {
      const completed = result.data.data.results.find((r: { success: boolean }) => r.success);
      if (completed?.data) {
        setCurrentConversion(completed.data);
      }
      setBatchProgress(filePaths.length, filePaths.length);
      setStatus('completed');
      refreshHistory();
    } else {
      setError(result.error || '批量转换失败');
      setStatus('error');
    }
  } catch (err) {
    setError(String(err));
    setStatus('error');
  }
}

async function onBrowseFiles() {
  try {
    const result = await openFileDialog();
    if (result.success && result.data && result.data.length > 0) {
      if (result.data.length === 1) {
        await handleSingleFile(result.data[0]);
      } else {
        await handleBatchFiles(result.data);
      }
    }
  } catch (err) {
    setError(String(err));
  }
}

async function refreshHistory() {
  try {
    const result = await getHistory();
    if (result.success && result.data) {
      setHistory(result.data);
    }
  } catch {
    // ignore
  }
}
</script>

<template>
  <div
    class="flex flex-1 items-center justify-center p-8"
  >
    <div
      class="flex w-full max-w-md flex-col items-center rounded-lg border-2 border-dashed p-12 transition-colors"
      :class="isDragging ? 'border-zinc-900 bg-zinc-50' : 'border-zinc-300'"
    >
      <div
        class="mb-4 flex h-16 w-16 items-center justify-center rounded-full"
        :class="isDragging ? 'bg-zinc-200' : 'bg-zinc-100'"
      >
        <Upload v-if="!isDragging" class="h-8 w-8 text-zinc-400" />
        <FileText v-else class="h-8 w-8 text-zinc-700" />
      </div>
      <p class="mb-1 text-sm font-medium text-zinc-900">
        {{ isDragging ? '释放文件开始转换' : '拖拽文件到此处' }}
      </p>
      <p class="mb-4 text-xs text-zinc-400">
        支持 PDF、DOCX、XLSX、PPTX、HTML
      </p>
      <p v-if="state.status === 'error' && state.error" class="mb-4 text-xs text-red-500">
        {{ state.error }}
      </p>
      <Button variant="outline" size="sm" @click="onBrowseFiles">
        浏览文件
      </Button>
    </div>
  </div>
</template>
