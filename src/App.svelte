<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import {
    faThumbtack,
    faInfoCircle,
    faXmark,
  } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  interface Process {
    pid: number;
    ppid: number;
    name: string;
    cpu_usage: number;
    memory_usage: number;
    status:
      | "Running"
      | "Sleeping"
      | "Idle"
      | "Zombie"
      | "Unknown"
      | "Stop"
      | string;
    user: string;
    command: string;
    threads?: number; // Optional field
  }

  interface SystemStats {
    cpu_usage: number[];
    memory_total: number;
    memory_used: number;
    uptime: number;
    load_avg: [number, number, number];
  }

  interface Column {
    id: keyof Process;
    label: string;
    visible: boolean;
    required?: boolean;
    format?: (value: any) => string;
  }

  let processes: Process[] = [];
  let systemStats: SystemStats | null = null;
  let intervalId: number;
  let error: string | null = null;
  let searchTerm = "";

  interface ProcessStatus {
    label: string;
    emoji: string;
    color: string;
  }

  const statusMap: Record<string, ProcessStatus> = {
    Running: {
      label: "Running",
      emoji: "🏃",
      color: "var(--green)",
    },
    Sleeping: {
      label: "Sleeping",
      emoji: "😴",
      color: "var(--blue)",
    },
    Idle: {
      label: "Idle",
      emoji: "⌛",
      color: "var(--overlay0)",
    },
    Zombie: {
      label: "Zombie",
      emoji: "🧟",
      color: "var(--red)",
    },
    Stop: {
      label: "Stopped",
      emoji: "⛔",
      color: "var(--yellow)",
    },
    Unknown: {
      label: "Unknown",
      emoji: "🤔",
      color: "var(--overlay0)",
    },
  };

  function formatStatus(status: string) {
    const processStatus = statusMap[status] || statusMap.Unknown;
    return `<span class="status-badge" style="--status-color: ${processStatus.color}">
      <span class="status-emoji">${processStatus.emoji}</span>
      ${processStatus.label}
    </span>`;
  }

  let columns: Column[] = [
    { id: "name", label: "Process Name", visible: true, required: true },
    { id: "pid", label: "PID", visible: true, required: true },
    { id: "status", label: "Status", visible: true, format: formatStatus },
    { id: "user", label: "User", visible: true },
    {
      id: "cpu_usage",
      label: "CPU %",
      visible: true,
      format: (v) => v.toFixed(1) + "%",
    },
    {
      id: "memory_usage",
      label: "Memory",
      visible: true,
      format: formatMemory,
    },
    { id: "threads", label: "Threads", visible: false },
    { id: "command", label: "Command", visible: false },
    { id: "ppid", label: "Parent PID", visible: false },
  ];

  let showColumnMenu = false;

  let currentPage = 1;
  let itemsPerPage = 50;
  let itemsPerPageOptions = [25, 50, 100, 250, 500];

  $: totalPages = Math.ceil(filteredProcesses.length / itemsPerPage);
  $: paginatedProcesses = filteredProcesses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage,
  );

  $: {
    // Reset to first page when filtering or changing items per page
    if (searchTerm || itemsPerPage) {
      currentPage = 1;
    }
  }

  function changePage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }

  async function getProcesses() {
    try {
      processes = await invoke<Process[]>("get_processes");
      error = null;
    } catch (e) {
      error = e.toString();
    }
  }

  async function killProcess(pid: number) {
    try {
      const success = await invoke<boolean>("kill_process", { pid });
      if (success) {
        await getProcesses();
      }
    } catch (e) {
      error = e.toString();
    }
  }

  function formatMemory(bytes: number) {
    return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  }

  async function getSystemStats() {
    try {
      systemStats = await invoke<SystemStats>("get_system_stats");
    } catch (e) {
      console.error("Failed to get system stats:", e);
    }
  }

  function formatUptime(seconds: number): string {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${days}d ${hours}h ${minutes}m`;
  }

  function formatMemorySize(bytes: number): string {
    const gb = bytes / (1024 * 1024 * 1024);
    return `${gb.toFixed(1)} GB`;
  }

  function formatMemoryPercentage(): string {
    if (!systemStats) return "0%";
    return (
      ((systemStats.memory_used / systemStats.memory_total) * 100).toFixed(1) +
      "%"
    );
  }

  function getUsageClass(percentage: number): string {
    if (percentage >= 90) return "critical";
    if (percentage >= 60) return "high";
    if (percentage >= 30) return "medium";
    return "low";
  }

  $: memoryPercentage = systemStats
    ? (systemStats.memory_used / systemStats.memory_total) * 100
    : 0;

  let isLoading = true;

  onMount(async () => {
    try {
      await Promise.all([getProcesses(), getSystemStats()]);
    } finally {
      isLoading = false;
    }

    intervalId = setInterval(() => {
      getProcesses();
      getSystemStats();
    }, 2000);
  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
  });

  $: filteredProcesses = processes.filter((process) =>
    process.name.toLowerCase().includes(searchTerm.toLowerCase()),
  );

  let sortConfig = {
    field: "cpu_usage" as keyof Process,
    direction: "desc" as "asc" | "desc",
  };

  function toggleSort(field: keyof Process) {
    if (sortConfig.field === field) {
      sortConfig.direction = sortConfig.direction === "asc" ? "desc" : "asc";
    } else {
      sortConfig.field = field;
      sortConfig.direction = "desc";
    }
  }

  function getSortIndicator(field: keyof Process) {
    if (sortConfig.field !== field) return "↕";
    return sortConfig.direction === "asc" ? "↑" : "↓";
  }

  $: sortedProcesses = filteredProcesses.sort((a, b) => {
    const aPin = pinnedProcesses.has(a.pid);
    const bPin = pinnedProcesses.has(b.pid);
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

  $: paginatedProcesses = sortedProcesses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage,
  );

  let pinnedProcesses: Set<number> = new Set();
  let selectedProcess: Process | null = null;
  let showModal = false;

  function togglePin(pid: number) {
    if (pinnedProcesses.has(pid)) {
      pinnedProcesses.delete(pid);
    } else {
      pinnedProcesses.add(pid);
    }
    pinnedProcesses = pinnedProcesses; // Trigger reactivity
  }

  function showProcessDetails(process: Process) {
    selectedProcess = process;
    showInfoModal = true;
  }

  let showConfirmModal = false;
  let processToKill: Process | null = null;
  let isKilling = false;

  function confirmKillProcess(process: Process) {
    processToKill = process;
    showConfirmModal = true;
  }

  async function handleConfirmKill() {
    if (processToKill) {
      isKilling = true;
      try {
        await killProcess(processToKill.pid);
      } finally {
        isKilling = false;
        showConfirmModal = false;
        processToKill = null;
      }
    }
  }

  function getProcessSymbol(processType: string) {
    switch (processType) {
      case "system":
        return "⚙️";
      case "kernel":
        return "🔲";
      default:
        return "📟";
    }
  }

  let showInfoModal = false;
</script>

{#if isLoading}
  <div class="loading-container">
    <div class="loading-content">
      <div class="spinner"></div>
      <span class="loading-text">Loading processes...</span>
    </div>
  </div>
{:else}
  <!-- Your existing app content -->
  <main>
    <div class="stats-bar">
      {#if systemStats}
        <div class="stats-grid">
          <div class="stat-box">
            <div class="stat-title">CPU Usage</div>
            <div class="cpu-bars">
              {#each systemStats.cpu_usage as usage, i}
                <div class="cpu-bar">
                  <span class="cpu-label">CPU {i}</span>
                  <div class="progress-bar">
                    <div
                      class="progress-fill {getUsageClass(usage)}"
                      style="width: {usage}%"
                    ></div>
                  </div>
                  <span class="cpu-value">{usage.toFixed(1)}%</span>
                </div>
              {/each}
            </div>
          </div>

          <div class="stat-box">
            <div class="stat-title">Memory</div>
            <div class="memory-info">
              <div class="progress-bar">
                <div
                  class="progress-fill {getUsageClass(memoryPercentage)}"
                  style="width: {memoryPercentage}%"
                ></div>
              </div>
              <div class="memory-text">
                {formatMemorySize(systemStats.memory_used)} / {formatMemorySize(
                  systemStats.memory_total,
                )}
                ({formatMemoryPercentage()})
              </div>
            </div>
          </div>

          <div class="stat-box">
            <div class="stat-title">System Info</div>
            <div class="system-info">
              <div>Uptime: {formatUptime(systemStats.uptime)}</div>
              <div>
                Load Avg: {systemStats.load_avg
                  .map((l) => l.toFixed(2))
                  .join(", ")}
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <div class="toolbar">
      <div class="toolbar-content">
        <div class="search-box">
          <div class="search-input-wrapper">
            <input
              type="text"
              placeholder="Search processes"
              bind:value={searchTerm}
              class="search-input"
            />
            {#if searchTerm}
              <button
                class="btn-clear"
                on:click={() => (searchTerm = "")}
                title="Clear search"
              >
                Clear
              </button>
            {/if}
          </div>
        </div>
        <div class="toolbar-spacer"></div>
        <div class="pagination-controls">
          <select
            class="select-input"
            bind:value={itemsPerPage}
            aria-label="Items per page"
          >
            {#each itemsPerPageOptions as option}
              <option value={option}>{option} per page</option>
            {/each}
          </select>
          <div class="pagination">
            <button
              class="btn-page"
              disabled={currentPage === 1}
              on:click={() => changePage(1)}
            >
              ««
            </button>
            <button
              class="btn-page"
              disabled={currentPage === 1}
              on:click={() => changePage(currentPage - 1)}
            >
              «
            </button>
            <span class="page-info">
              Page {currentPage} of {totalPages}
              <span class="results-info">
                ({filteredProcesses.length} processes)
              </span>
            </span>
            <button
              class="btn-page"
              disabled={currentPage === totalPages}
              on:click={() => changePage(currentPage + 1)}
            >
              »
            </button>
            <button
              class="btn-page"
              disabled={currentPage === totalPages}
              on:click={() => changePage(totalPages)}
            >
              »»
            </button>
          </div>
        </div>
        <div class="column-toggle">
          <button
            class="btn-toggle"
            on:click={() => (showColumnMenu = !showColumnMenu)}
            aria-label="Toggle columns"
          >
            Columns
            <span class="icon">{showColumnMenu ? "▼" : "▶"}</span>
          </button>

          {#if showColumnMenu}
            <div
              class="column-menu"
              on:mouseleave={() => (showColumnMenu = false)}
            >
              {#each columns as column}
                <label class="column-option">
                  <input
                    type="checkbox"
                    bind:checked={column.visible}
                    disabled={column.required}
                  />
                  <span>{column.label}</span>
                </label>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    </div>

    {#if error}
      <div class="alert">{error}</div>
    {/if}

    <div class="table-container">
      <table>
        <thead>
          <tr>
            {#each columns.filter((col) => col.visible) as column}
              <th class="sortable" on:click={() => toggleSort(column.id)}>
                <div class="th-content">
                  {column.label}
                  <span
                    class="sort-indicator"
                    class:active={sortConfig.field === column.id}
                  >
                    {getSortIndicator(column.id)}
                  </span>
                </div>
              </th>
            {/each}
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each paginatedProcesses as process (process.pid)}
            <tr
              class:high-usage={process.cpu_usage > 50 ||
                (process.memory_usage / systemStats?.memory_total || 0) > 0.1}
              class:pinned={pinnedProcesses.has(process.pid)}
            >
              {#each columns.filter((col) => col.visible) as column}
                <td class="truncate">
                  {#if column.format}
                    {@html column.format(process[column.id])}
                  {:else}
                    {process[column.id]}
                  {/if}
                </td>
              {/each}
              <td class="col-actions">
                <div class="action-buttons">
                  <button
                    class="btn-action pin-btn"
                    class:pinned={pinnedProcesses.has(process.pid)}
                    on:click={() => togglePin(process.pid)}
                    title={pinnedProcesses.has(process.pid) ? "Unpin" : "Pin"}
                  >
                    <Fa icon={faThumbtack} />
                  </button>
                  <button
                    class="btn-action info-btn"
                    on:click={() => showProcessDetails(process)}
                    title="Show Details"
                  >
                    <Fa icon={faInfoCircle} />
                  </button>
                  <button
                    class="btn-action kill-btn"
                    on:click={() => confirmKillProcess(process)}
                    title="End Process"
                  >
                    <Fa icon={faXmark} />
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </main>
{/if}

{#if showConfirmModal && processToKill}
  <div class="modal-backdrop" on:click={() => (showConfirmModal = false)}>
    <div class="modal confirm-modal" on:click|stopPropagation>
      <div class="modal-header">
        <h2>Confirm Action</h2>
        <button
          class="btn-close"
          on:click={() => (showConfirmModal = false)}
          disabled={isKilling}>×</button
        >
      </div>
      <div class="modal-content">
        <p class="confirm-message">
          Are you sure you want to end this process?
        </p>
        <div class="process-info">
          <span class="process-name">{processToKill.name}</span>
          <span class="process-pid">(PID: {processToKill.pid})</span>
        </div>
        <div class="confirm-actions">
          <button
            class="btn-secondary"
            on:click={() => (showConfirmModal = false)}
            disabled={isKilling}
          >
            Cancel
          </button>
          <button
            class="btn-danger"
            on:click={handleConfirmKill}
            disabled={isKilling}
          >
            {#if isKilling}
              <div class="spinner"></div>
              <span>Ending...</span>
            {:else}
              End Process
            {/if}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

{#if showInfoModal && selectedProcess}
  <div class="modal-backdrop" on:click={() => (showInfoModal = false)}>
    <div class="modal info-modal" on:click|stopPropagation>
      <div class="modal-header">
        <h2>Process Details</h2>
        <button class="btn-close" on:click={() => (showInfoModal = false)}
          >×</button
        >
      </div>
      <div class="modal-content">
        <div class="process-details">
          <div class="detail-row">
            <span class="detail-label">Name:</span>
            <span class="detail-value">{selectedProcess.name}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">PID:</span>
            <span class="detail-value">{selectedProcess.pid}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Parent PID:</span>
            <span class="detail-value">{selectedProcess.ppid}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">User:</span>
            <span class="detail-value">{selectedProcess.user}</span>
          </div>
          <div class="detail-row">
            <span class="detail-label">Status:</span>
            <span class="detail-value">
              {@html formatStatus(selectedProcess.status)}
            </span>
          </div>
          <div class="detail-row">
            <span class="detail-label">CPU Usage:</span>
            <span class="detail-value"
              >{selectedProcess.cpu_usage.toFixed(1)}%</span
            >
          </div>
          <div class="detail-row">
            <span class="detail-label">Memory Usage:</span>
            <span class="detail-value"
              >{formatMemory(selectedProcess.memory_usage)}</span
            >
          </div>
          <div class="detail-row">
            <span class="detail-label">Command:</span>
            <span class="detail-value command">{selectedProcess.command}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  :global(:root) {
    /* Catppuccin Mocha */
    --base: #1e1e2e;
    --mantle: #181825;
    --crust: #11111b;
    --text: #cdd6f4;
    --subtext0: #a6adc8;
    --subtext1: #bac2de;
    --surface0: #313244;
    --surface1: #45475a;
    --surface2: #585b70;
    --overlay0: #6c7086;
    --overlay1: #7f849c;
    --blue: #89b4fa;
    --lavender: #b4befe;
    --sapphire: #74c7ec;
    --sky: #89dceb;
    --red: #f38ba8;
    --maroon: #eba0ac;
    --peach: #fab387;
    --yellow: #f9e2af;
    --green: #a6e3a1;
    --teal: #94e2d5;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, sans-serif;
    background-color: var(--base);
    color: var(--text);
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
    user-select: none;
  }

  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    min-width: min-content;
  }

  .toolbar {
    padding: 8px;
    border-bottom: 1px solid var(--surface0);
    background-color: var(--mantle);
  }

  .search-box {
    max-width: 240px;
  }

  .search-input {
    width: 100%;
    padding: 6px 12px;
    border: 1px solid var(--surface0);
    border-radius: 6px;
    font-size: 13px;
    background-color: var(--crust);
    color: var(--text);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--blue);
    box-shadow: 0 0 0 2px var(--blue-alpha);
  }

  .stats-bar {
    background-color: var(--mantle);
    border-bottom: 1px solid var(--surface0);
    padding: 12px;
  }

  .stat-box {
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 8px;
    padding: 12px;
  }

  .stat-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--subtext0);
    margin-bottom: 8px;
  }

  .progress-bar {
    flex: 1;
    height: 8px;
    background-color: var(--surface0);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.2s ease;
  }

  .table-container {
    flex: 1;
    overflow-x: auto;
    overflow-y: auto;
  }

  table {
    width: max-content;
    min-width: 100%;
    table-layout: fixed;
    border-collapse: collapse;
    font-size: 13px;
  }

  th {
    position: sticky;
    top: 0;
    background: var(--mantle);
    text-align: left;
    padding: 8px 12px;
    font-weight: 500;
    color: var(--subtext0);
    border-bottom: 1px solid var(--surface0);
    transition: background-color 0.2s ease;
    z-index: 3;
  }

  td {
    padding: 6px 12px;
    border-bottom: 1px solid var(--surface0);
    color: var(--text);
    z-index: 1;
  }

  tr:hover {
    background-color: var(--surface0);
  }

  .command {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      monospace;
    font-size: 12px;
    color: var(--subtext1);
  }

  .btn-danger {
    padding: 8px 16px;
    font-size: 13px;
    color: var(--base);
    background: var(--red);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .btn-danger:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .loading-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid var(--base);
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .cpu-label,
  .cpu-value,
  .memory-text,
  .system-info {
    color: var(--subtext0);
  }

  /* Custom scrollbar */
  ::-webkit-scrollbar {
    width: 10px;
    height: 10px;
  }

  ::-webkit-scrollbar-track {
    background: var(--mantle);
  }

  ::-webkit-scrollbar-thumb {
    background: var(--surface1);
    border-radius: 5px;
    border: 2px solid var(--mantle);
  }

  ::-webkit-scrollbar-thumb:hover {
    background: var(--surface2);
  }

  .alert {
    margin: 8px;
    padding: 8px 12px;
    background-color: var(--surface0);
    border: 1px solid var(--red);
    border-radius: 6px;
    color: var(--red);
    font-size: 13px;
  }

  /* CPU usage colors based on percentage */
  .progress-fill.low {
    background-color: var(--green);
  }

  .progress-fill.medium {
    background-color: var(--yellow);
  }

  .progress-fill.high {
    background-color: var(--peach);
  }

  .progress-fill.critical {
    background-color: var(--red);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
    padding: 12px;
    min-width: max-content;
  }

  .cpu-bars {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 8px;
  }

  .cpu-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
  }

  .cpu-label {
    width: 45px;
    color: var(--subtext0);
  }

  .cpu-value {
    width: 45px;
    text-align: right;
    color: var(--subtext0);
  }

  .memory-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .memory-text {
    font-size: 12px;
    color: var(--subtext0);
  }

  .system-info {
    font-size: 12px;
    color: var(--subtext0);
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .toolbar-content {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 12px;
    min-width: max-content;
  }

  .toolbar-spacer {
    flex: 1;
  }

  .search-box {
    width: 240px;
  }

  .column-toggle {
    position: relative;
  }

  .btn-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-toggle:hover {
    background: var(--surface1);
  }

  .icon {
    font-size: 10px;
    color: var(--subtext0);
  }

  .column-menu {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    padding: 8px;
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    z-index: 100;
  }

  .column-option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    font-size: 12px;
    color: var(--text);
    cursor: pointer;
    border-radius: 4px;
  }

  .column-option:hover {
    background: var(--surface0);
  }

  .column-option input[type="checkbox"] {
    appearance: none;
    width: 16px;
    height: 16px;
    border: 1px solid var(--surface2);
    border-radius: 4px;
    background: var(--mantle);
    cursor: pointer;
    position: relative;
  }

  .column-option input[type="checkbox"]:checked {
    background: var(--blue);
    border-color: var(--blue);
  }

  .column-option input[type="checkbox"]:checked::after {
    content: "✓";
    position: absolute;
    color: var(--base);
    font-size: 12px;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .column-option input[type="checkbox"]:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .pagination-controls {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .select-input {
    padding: 6px 12px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    appearance: none;
    padding-right: 24px;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23cdd6f4' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
  }

  .select-input:hover {
    background-color: var(--surface1);
  }

  .select-input:focus {
    outline: none;
    border-color: var(--blue);
  }

  .pagination {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .btn-page {
    padding: 6px 10px;
    font-size: 12px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-page:hover:not(:disabled) {
    background: var(--surface1);
  }

  .btn-page:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .page-info {
    font-size: 12px;
    color: var(--subtext0);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .results-info {
    color: var(--overlay0);
  }

  /* Style the select dropdown */
  .select-input option {
    background: var(--base);
    color: var(--text);
    padding: 8px;
  }

  .sortable {
    cursor: pointer;
    user-select: none;
  }

  .th-content {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .sort-indicator {
    color: var(--overlay0);
    font-size: 12px;
    opacity: 0.5;
    transition: all 0.2s ease;
  }

  .sort-indicator.active {
    color: var(--blue);
    opacity: 1;
  }

  .sortable:hover .sort-indicator {
    opacity: 1;
  }

  .high-usage {
    background-color: color-mix(in srgb, var(--red) 10%, transparent);
  }

  .high-usage:hover {
    background-color: color-mix(in srgb, var(--red) 15%, transparent);
  }

  th:hover {
    background-color: var(--surface0);
  }

  /* Add transition to table rows */
  tr {
    transition: background-color 0.2s ease;
  }

  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 500;
    background-color: color-mix(in srgb, var(--status-color) 15%, transparent);
    color: var(--status-color);
    border: 1px solid color-mix(in srgb, var(--status-color) 30%, transparent);
  }

  :global(.status-emoji) {
    font-size: 14px;
    line-height: 1;
  }

  /* Optional: Add a subtle hover effect */
  :global(.status-badge:hover) {
    background-color: color-mix(in srgb, var(--status-color) 20%, transparent);
  }

  /* Column widths */
  .col-process-name {
    width: 200px;
    min-width: 200px;
  }
  .col-pid {
    width: 80px;
    min-width: 80px;
  }
  .col-status {
    width: 120px;
    min-width: 120px;
  }
  .col-user {
    width: 100px;
    min-width: 100px;
  }
  .col-cpu {
    width: 80px;
    min-width: 80px;
  }
  .col-memory {
    width: 100px;
    min-width: 100px;
  }
  .col-threads {
    width: 80px;
    min-width: 80px;
  }
  .col-command {
    width: 300px;
    min-width: 300px;
  }
  .col-actions {
    width: 120px;
    min-width: 120px;
  }

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 0;
  }

  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: var(--base);
    border-radius: 8px;
    width: 600px;
    max-width: 90vw;
    max-height: 90vh;
    overflow: auto;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    /* Remove any transform properties if they exist */
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
    border-bottom: 1px solid var(--surface0);
  }

  .modal-header h2 {
    margin: 0;
    font-size: 18px;
    color: var(--text);
  }

  .btn-close {
    background: transparent;
    border: none;
    color: var(--overlay0);
    font-size: 24px;
    cursor: pointer;
    padding: 4px 8px;
  }

  .btn-close:hover {
    color: var(--text);
  }

  .modal-content {
    padding: 16px;
  }

  .detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 16px;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .detail-item.full-width {
    grid-column: 1 / -1;
  }

  .detail-label {
    font-size: 12px;
    color: var(--overlay0);
  }

  .detail-value {
    font-size: 14px;
    color: var(--text);
  }

  .detail-value.command {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      monospace;
    font-size: 12px;
    padding: 8px;
    background: var(--mantle);
    border-radius: 4px;
    overflow-x: auto;
  }

  tr.pinned {
    background-color: color-mix(in srgb, var(--blue) 10%, transparent);
  }

  tr.pinned:hover {
    background-color: color-mix(in srgb, var(--blue) 15%, transparent);
  }

  .action-buttons {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .btn-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: none;
    cursor: pointer;
    background: transparent;
    border-radius: 6px;
    width: 28px;
    height: 28px;
    transition: all 0.2s ease;
    position: relative;
    overflow: hidden;
  }

  .btn-action::before {
    content: "";
    position: absolute;
    inset: 0;
    opacity: 0.1;
    transition: opacity 0.2s ease;
  }

  .btn-action:hover::before {
    opacity: 0.15;
  }

  .btn-icon {
    font-size: 14px;
    line-height: 1;
    position: relative;
    z-index: 1;
  }

  .btn-text {
    font-size: 12px;
    font-weight: 500;
    padding: 4px 8px;
    position: relative;
    z-index: 1;
  }

  /* Pin button */
  .pin-btn {
    color: var(--sapphire);
  }

  .pin-btn::before {
    background: var(--sapphire);
  }

  .pin-btn.pinned {
    color: var(--blue);
    transform: rotate(45deg);
  }

  .pin-btn.pinned::before {
    background: var(--blue);
    opacity: 0.15;
  }

  /* Info button */
  .info-btn {
    color: var(--lavender);
  }

  .info-btn::before {
    background: var(--lavender);
  }

  /* Kill button */
  .kill-btn {
    color: var(--red);
    border: 1px solid color-mix(in srgb, var(--red) 30%, transparent);
  }

  .kill-btn::before {
    background: var(--red);
  }

  .kill-btn:hover {
    color: var(--base);
    background: var(--red);
  }

  .kill-btn:hover::before {
    opacity: 1;
  }

  /* Add subtle glow effect on hover */
  .btn-action:hover {
    box-shadow: 0 0 12px color-mix(in srgb, currentColor 20%, transparent);
  }

  /* Optional: Add active state */
  .btn-action:active {
    transform: translateY(1px);
  }

  .pin-btn.pinned:active {
    transform: rotate(45deg) translateY(1px);
  }

  /* Optional: Add focus state */
  .btn-action:focus {
    outline: none;
    box-shadow: 0 0 0 2px color-mix(in srgb, currentColor 30%, transparent);
  }

  /* Optional: Disable hover effects when button is disabled */
  .btn-action:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-action:disabled:hover {
    transform: none;
    box-shadow: none;
  }

  .btn-action:disabled::before {
    display: none;
  }

  /* Add transition for pin rotation */
  .pin-btn {
    transition: all 0.2s ease;
  }

  /* Update the actions column styles */
  .col-actions {
    position: sticky;
    right: 0;
    z-index: 2;
    background: var(--base);
    border-left: 1px solid var(--surface0);
  }

  /* Add styles for the actions header */
  th:last-child {
    position: sticky;
    right: 0;
    z-index: 4;
    background: var(--mantle);
    border-left: 1px solid var(--surface0);
  }

  /* Regular cells */
  td {
    z-index: 1;
  }

  .confirm-modal {
    max-width: 400px;
  }

  .confirm-message {
    color: var(--text);
    margin: 0 0 16px 0;
    font-size: 14px;
  }

  .process-info {
    background: var(--mantle);
    padding: 12px;
    border-radius: 6px;
    margin-bottom: 24px;
  }

  .process-name {
    color: var(--text);
    font-weight: 500;
    font-size: 14px;
  }

  .process-pid {
    color: var(--subtext0);
    font-size: 12px;
    margin-left: 8px;
  }

  .confirm-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  .btn-secondary {
    padding: 8px 16px;
    font-size: 13px;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-secondary:hover {
    background: var(--surface1);
  }

  .btn-danger {
    padding: 8px 16px;
    font-size: 13px;
    color: var(--base);
    background: var(--red);
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .btn-danger:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-danger:hover {
    background: color-mix(in srgb, var(--red) 90%, white);
  }

  /* Optional: Add animation for the modal */
  .confirm-modal {
    animation: slideIn 0.2s ease;
  }

  @keyframes slideIn {
    from {
      transform: translate(-50%, -60%);
      opacity: 0;
    }
    to {
      transform: translate(-50%, -50%);
      opacity: 1;
    }
  }

  /* Optional: Add focus styles for better accessibility */
  .btn-secondary:focus,
  .btn-danger:focus {
    outline: none;
    box-shadow: 0 0 0 2px var(--blue);
  }

  /* Optional: Add active state */
  .btn-secondary:active,
  .btn-danger:active {
    transform: translateY(1px);
  }

  .loading-container {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--base);
  }

  .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--surface0);
    border-top-color: var(--blue);
    border-radius: 50%;
    animation: spin 1s ease-in-out infinite;
  }

  .loading-text {
    color: var(--subtext0);
    font-size: 14px;
    font-weight: 500;
    letter-spacing: 0.5px;
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }

  /* Optional: Add a subtle gradient background */
  .loading-container {
    background: linear-gradient(135deg, var(--base) 0%, var(--mantle) 100%);
  }

  /* Optional: Add a subtle shadow to the spinner */
  .spinner {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  /* Optional: Add a subtle text shadow */
  .loading-text {
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  .process-cell {
    padding: 6px 12px !important;
  }

  .process-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .process-icon {
    font-size: 14px;
    line-height: 1;
    display: inline-flex;
    align-items: center;
  }

  .process-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Optional: Add colors for different process types */
  .process-icon[data-type="system"] {
    color: var(--sapphire);
  }

  .process-icon[data-type="kernel"] {
    color: var(--red);
  }

  .process-icon[data-type="background"] {
    color: var(--overlay0);
  }

  .info-modal {
    max-width: 500px;
  }

  .process-details {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .detail-row {
    display: flex;
    gap: 12px;
    padding: 8px;
    border-radius: 4px;
  }

  .detail-row:nth-child(odd) {
    background: var(--surface0);
  }

  .detail-label {
    flex: 0 0 120px;
    color: var(--subtext0);
    font-weight: 500;
  }

  .detail-value {
    flex: 1;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      monospace;
    word-break: break-all;
  }

  .detail-value.command {
    font-size: 12px;
    color: var(--subtext1);
  }

  .btn-action {
    padding: 4px 8px;
    background: transparent;
    border: 1px solid var(--surface1);
    border-radius: 4px;
    color: var(--text);
    cursor: pointer;
    transition: all 0.2s ease;
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .btn-action:hover {
    background: var(--surface0);
  }

  .btn-action.info-btn:hover {
    border-color: var(--blue);
    color: var(--blue);
  }

  .action-buttons {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid var(--base);
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  .btn-danger {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .btn-secondary:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .search-input {
    width: 240px;
    padding: 6px 12px;
    border: 1px solid var(--surface0);
    border-radius: 6px;
    font-size: 13px;
    background-color: var(--crust);
    color: var(--text);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--blue);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--blue) 25%, transparent);
  }

  .btn-clear {
    padding: 6px 12px;
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    color: var(--text);
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .btn-clear:hover {
    background: var(--surface1);
    border-color: var(--surface2);
  }
</style>
