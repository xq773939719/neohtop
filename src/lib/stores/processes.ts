import { writable, derived } from "svelte/store";
import type { Process, SystemStats } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

interface ProcessStore {
  processes: Process[];
  systemStats: SystemStats | null;
  error: string | null;
  isLoading: boolean;
  searchTerm: string;
  currentPage: number;
  pinnedProcesses: Set<string>;
  selectedProcess: Process | null;
  showInfoModal: boolean;
  showConfirmModal: boolean;
  processToKill: Process | null;
  isKilling: boolean;
  isFrozen: boolean;
  selectedProcessPid: number | null;
  sortConfig: {
    field: keyof Process;
    direction: "asc" | "desc";
  };
}

const initialState: ProcessStore = {
  processes: [],
  systemStats: null,
  error: null,
  isLoading: true,
  searchTerm: "",
  currentPage: 1,
  pinnedProcesses: new Set(),
  selectedProcess: null,
  showInfoModal: false,
  showConfirmModal: false,
  processToKill: null,
  isKilling: false,
  isFrozen: false,
  selectedProcessPid: null,
  sortConfig: {
    field: "cpu_usage",
    direction: "desc",
  },
};

function createProcessStore() {
  const { subscribe, set, update } = writable<ProcessStore>(initialState);

  return {
    subscribe,
    set,
    update,

    setIsLoading: (isLoading: boolean) =>
      update((state) => ({ ...state, isLoading })),

    async getProcesses() {
      try {
        const result = await invoke<[Process[], SystemStats]>("get_processes");
        update((state) => ({
          ...state,
          processes: result[0],
          systemStats: result[1],
          error: null,
        }));
      } catch (e: unknown) {
        update((state) => ({
          ...state,
          error: e instanceof Error ? e.message : String(e),
        }));
      }
    },

    async killProcess(pid: number) {
      try {
        const success = await invoke<boolean>("kill_process", { pid });
        if (success) {
          await this.getProcesses();
        }
      } catch (e: unknown) {
        update((state) => ({
          ...state,
          error: e instanceof Error ? e.message : String(e),
        }));
      }
    },

    toggleSort(field: keyof Process) {
      update((state) => ({
        ...state,
        sortConfig: {
          field,
          direction:
            state.sortConfig.field === field
              ? state.sortConfig.direction === "asc"
                ? "desc"
                : "asc"
              : "desc",
        },
      }));
    },

    togglePin(command: string) {
      update((state) => {
        const newPinnedProcesses = new Set(state.pinnedProcesses);
        if (newPinnedProcesses.has(command)) {
          newPinnedProcesses.delete(command);
        } else {
          newPinnedProcesses.add(command);
        }
        return { ...state, pinnedProcesses: newPinnedProcesses };
      });
    },

    setSearchTerm: (searchTerm: string) =>
      update((state) => ({ ...state, searchTerm, currentPage: 1 })),
    setIsFrozen: (isFrozen: boolean) =>
      update((state) => ({ ...state, isFrozen })),
    setCurrentPage: (currentPage: number) =>
      update((state) => ({ ...state, currentPage })),

    showProcessDetails(process: Process) {
      update((state) => ({
        ...state,
        selectedProcessPid: process.pid,
        selectedProcess: process,
        showInfoModal: true,
      }));
    },

    closeProcessDetails() {
      update((state) => ({
        ...state,
        showInfoModal: false,
        selectedProcess: null,
        selectedProcessPid: null,
      }));
    },

    confirmKillProcess(process: Process) {
      update((state) => ({
        ...state,
        processToKill: process,
        showConfirmModal: true,
      }));
    },

    closeConfirmKill() {
      update((state) => ({
        ...state,
        showConfirmModal: false,
        processToKill: null,
      }));
    },

    async handleConfirmKill() {
      update((state) => ({ ...state, isKilling: true }));

      try {
        const pid = this.getState().processToKill?.pid;
        if (pid) {
          await this.killProcess(pid);
        }
      } finally {
        update((state) => ({
          ...state,
          isKilling: false,
          showConfirmModal: false,
          processToKill: null,
        }));
      }
    },

    // Helper to get current state
    getState() {
      let currentState: ProcessStore | undefined;
      subscribe((state) => {
        currentState = state;
      })();
      return currentState!;
    },
  };
}

export const processStore = createProcessStore();
