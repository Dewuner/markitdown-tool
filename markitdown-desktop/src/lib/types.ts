export interface ConversionData {
  filename: string;
  source_path: string;
  output_path: string | null;
  markdown_content: string | null;
  image_paths: string[] | null;
  file_size: number | null;
  status: string;
}

export interface ConversionResponse {
  success: boolean;
  data?: ConversionData;
  error?: string;
}

export interface BatchData {
  results: ConversionResponse[];
}

export interface BatchResponse {
  success: boolean;
  data?: BatchData;
  error?: string;
}

export interface HistoryEntry {
  id: number;
  filename: string;
  source_path: string;
  output_path: string | null;
  status: string;
  error_message: string | null;
  markdown_content: string | null;
  image_paths: string | null;
  file_size: number | null;
  created_at: string;
}

export interface IpcResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export type AppStatus = 'idle' | 'converting' | 'completed' | 'error';
