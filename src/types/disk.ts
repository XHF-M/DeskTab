// 盘位类型定义
export interface DiskDrive {
  driveLetter: string;
  label?: string;
  size?: number;
  freeSpace?: number;
}

export interface DiskState {
  drives: string[];
  selectedDrive: string | null;
  loading: boolean;
  error: string | null;
}