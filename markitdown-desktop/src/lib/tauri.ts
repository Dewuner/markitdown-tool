import { invoke } from '@tauri-apps/api/core';
import type {
  ConversionData,
  BatchResponse,
  HistoryEntry,
  IpcResponse,
} from './types';

export async function convertFile(filePath: string): Promise<IpcResponse<ConversionData>> {
  return invoke<IpcResponse<ConversionData>>('convert_file', { filePath });
}

export async function batchConvert(filePaths: string[]): Promise<IpcResponse<BatchResponse>> {
  return invoke<IpcResponse<BatchResponse>>('batch_convert', { filePaths });
}

export async function getHistory(): Promise<IpcResponse<HistoryEntry[]>> {
  return invoke<IpcResponse<HistoryEntry[]>>('get_history');
}

export async function deleteHistory(id: number): Promise<IpcResponse<void>> {
  return invoke<IpcResponse<void>>('delete_history', { id });
}

export async function openFileDialog(): Promise<IpcResponse<string[]>> {
  return invoke<IpcResponse<string[]>>('open_file_dialog');
}

export async function openFolderDialog(): Promise<IpcResponse<string | null>> {
  return invoke<IpcResponse<string | null>>('open_folder_dialog');
}
