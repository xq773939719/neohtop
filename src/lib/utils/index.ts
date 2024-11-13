import type { Process } from "$lib/types";
import type { SortConfig } from "$lib/types";

export interface ProcessStatus {
  label: string;
  emoji: string;
  color: string;
}

export function formatMemorySize(bytes: number): string {
  const gb = bytes / (1024 * 1024 * 1024);
  return `${gb.toFixed(1)} GB`;
}

export function formatPercentage(value: number): string {
  return `${value.toFixed(1)}%`;
}

export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  return `${days}d ${hours}h ${minutes}m`;
}

export function getUsageClass(percentage: number): string {
  if (percentage >= 90) return "critical";
  if (percentage >= 60) return "high";
  if (percentage >= 30) return "medium";
  return "low";
}

export function formatBytes(bytes: number): string {
  const units = ["B", "KB", "MB", "GB", "TB"];
  let value = bytes;
  let unitIndex = 0;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex++;
  }

  return `${value.toFixed(1)} ${units[unitIndex]}`;
}

export function formatDate(timestamp: number) {
  return new Date(timestamp * 1000).toLocaleString();
}

export function filterProcesses(
  processes: Process[],
  searchTerm: string,
  statusFilter: string,
): Process[] {
  return processes.filter((process) => {
    let matchesSearch = searchTerm.length === 0;
    searchTerm
      .split(",")
      .map((term) => term.trim())
      .forEach((term) => {
        const nameSubstringMatch = process.name
          .toLowerCase()
          .includes(term.toLowerCase());
        const nameRegexMatch = (() => {
          try {
            return new RegExp(term, "i").test(process.name);
          } catch {
            return false;
          }
        })();
        const commandMatch = process.command
          .toLowerCase()
          .includes(term.toLowerCase());
        const pidMatch = process.pid.toString().includes(term);
        matchesSearch ||=
          nameSubstringMatch || nameRegexMatch || commandMatch || pidMatch;
      });

    const matchesStatus =
      statusFilter === "all"
        ? true
        : process.status.toLowerCase() === statusFilter;

    return matchesSearch && matchesStatus;
  });
}

export function sortProcesses(
  processes: Process[],
  sortConfig: SortConfig,
  pinnedProcesses: Set<string>,
): Process[] {
  return [...processes].sort((a, b) => {
    const aPin = pinnedProcesses.has(a.command);
    const bPin = pinnedProcesses.has(b.command);
    if (aPin && !bPin) return -1;
    if (!aPin && bPin) return 1;

    const aValue = a[sortConfig.field];
    const bValue = b[sortConfig.field];
    const direction = sortConfig.direction === "asc" ? 1 : -1;

    if (typeof aValue === "string" && typeof bValue === "string") {
      return direction * aValue.localeCompare(bValue);
    }
    return direction * (Number(aValue) - Number(bValue));
  });
}
