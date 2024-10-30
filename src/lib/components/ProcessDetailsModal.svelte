<script lang="ts">
  import Modal from "./Modal.svelte";
  import { formatStatus } from "$lib/utils/processStatus";

  interface Process {
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

  export let show = false;
  export let process: Process | null = null;
  export let onClose: () => void;

  function formatMemory(bytes: number) {
    return (bytes / (1024 * 1024)).toFixed(1) + " MB";
  }
</script>

<Modal {show} title="Process Details" maxWidth="500px" {onClose}>
  {#if process}
    <div class="process-details">
      <div class="detail-row">
        <span class="detail-label">Name:</span>
        <span class="detail-value">{process.name}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">PID:</span>
        <span class="detail-value">{process.pid}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">Parent PID:</span>
        <span class="detail-value">{process.ppid}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">User:</span>
        <span class="detail-value">{process.user}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">Status:</span>
        <span class="detail-value">
          {@html formatStatus(process.status)}
        </span>
      </div>
      <div class="detail-row">
        <span class="detail-label">CPU Usage:</span>
        <span class="detail-value">{process.cpu_usage.toFixed(1)}%</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">Memory Usage:</span>
        <span class="detail-value">{formatMemory(process.memory_usage)}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">Command:</span>
        <span class="detail-value command">{process.command}</span>
      </div>
    </div>
  {/if}
</Modal>

<style>
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
</style>
