// Create a new types file to centralize interfaces
export interface Process {
  pid: number;
  ppid: number;
  name: string;
  cpu_usage: number;
  memory_usage: number;
  status: string;
  user: string;
  command: string;
  threads?: number;
}

export interface SystemStats {
  cpu_usage: number[];
  memory_total: number;
  memory_used: number;
  memory_free: number;
  memory_cached: number;
  uptime: number;
  load_avg: [number, number, number];
}

export interface Column {
  id: keyof Process;
  label: string;
  visible: boolean;
  required?: boolean;
  format?: (value: any) => string;
}