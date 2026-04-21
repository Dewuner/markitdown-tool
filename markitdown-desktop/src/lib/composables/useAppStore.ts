import { reactive, ref } from 'vue';
import type { HistoryEntry, ConversionData, AppStatus } from '@/lib/types';
import { deleteHistory } from '@/lib/tauri';

interface AppState {
  status: AppStatus;
  currentConversion: ConversionData | null;
  history: HistoryEntry[];
  sidebarCollapsed: boolean;
  errorMessage: string | null;
  batchProgress: { current: number; total: number };
}

const state = reactive<AppState>({
  status: 'idle',
  currentConversion: null,
  history: [],
  sidebarCollapsed: false,
  errorMessage: null,
  batchProgress: { current: 0, total: 0 },
});

const selectedHistoryId = ref<number | null>(null);

export function useAppStore() {
  function setStatus(status: AppStatus) {
    state.status = status;
  }

  function setCurrentConversion(data: ConversionData | null) {
    state.currentConversion = data;
  }

  function setHistory(entries: HistoryEntry[]) {
    state.history = entries;
  }

  function addToHistory(entry: HistoryEntry) {
    state.history.unshift(entry);
  }

  function removeFromHistory(id: number) {
    deleteHistory(id).then((result) => {
      if (result.success) {
        state.history = state.history.filter((e) => e.id !== id);
      }
    });
  }

  function toggleSidebar() {
    state.sidebarCollapsed = !state.sidebarCollapsed;
  }

  function setError(msg: string | null) {
    state.errorMessage = msg;
  }

  function setBatchProgress(current: number, total: number) {
    state.batchProgress = { current, total };
  }

  function selectHistory(id: number | null) {
    selectedHistoryId.value = id;
    if (id !== null) {
      const entry = state.history.find((e) => e.id === id);
      if (entry) {
        state.currentConversion = {
          filename: entry.filename,
          source_path: entry.source_path,
          output_path: entry.output_path,
          markdown_content: entry.markdown_content,
          image_paths: entry.image_paths ? JSON.parse(entry.image_paths) : null,
          file_size: entry.file_size,
          status: entry.status,
        };
      }
    }
  }

  return {
    state,
    selectedHistoryId,
    setStatus,
    setCurrentConversion,
    setHistory,
    addToHistory,
    removeFromHistory,
    toggleSidebar,
    setError,
    setBatchProgress,
    selectHistory,
  };
}
