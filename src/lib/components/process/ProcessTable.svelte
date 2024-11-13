<script lang="ts">
  import type { Process, Column } from "$lib/types";
  import ProcessIcon from "./ProcessIcon.svelte";
  import TableHeader from "./TableHeader.svelte";
  import ActionButtons from "./ActionButtons.svelte";

  export let processes: Process[];
  export let columns: Column[];
  export let systemStats: { memory_total: number } | null;
  export let sortConfig: { field: keyof Process; direction: "asc" | "desc" };
  export let pinnedProcesses: Set<string>;

  export let onToggleSort: (field: keyof Process) => void;
  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
</script>

<div class="table-container">
  <table>
    <TableHeader {columns} {sortConfig} {onToggleSort} />
    <tbody>
      {#each processes as process (process.pid)}
        <tr
          class:high-usage={process.cpu_usage > 50 ||
            process.memory_usage / (systemStats?.memory_total || 0) > 0.1}
          class:pinned={pinnedProcesses.has(process.command)}
        >
          {#each columns.filter((col) => col.visible) as column}
            <td class="truncate">
              {#if column.id === "name"}
                <div class="name-cell">
                  <ProcessIcon processName={process.name} />
                  <span class="process-name">{process.name}</span>
                </div>
              {:else if column.format}
                {@html column.format(process[column.id])}
              {:else}
                {process[column.id]}
              {/if}
            </td>
          {/each}
          <ActionButtons
            {process}
            isPinned={pinnedProcesses.has(process.command)}
            {onTogglePin}
            {onShowDetails}
            {onKillProcess}
          />
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    flex: 1;
    overflow-x: auto;
    overflow-y: auto;

    /* Scrollbar styles */
    scrollbar-width: thin; /* Firefox */
    scrollbar-color: var(--surface2) var(--mantle); /* Firefox */
  }

  /* Webkit scrollbar styles (Chrome, Safari, Edge) */
  .table-container::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  .table-container::-webkit-scrollbar-track {
    background: var(--mantle);
    border-radius: 4px;
  }

  .table-container::-webkit-scrollbar-thumb {
    background: var(--surface2);
    border-radius: 4px;
    transition: background 0.2s ease;
  }

  .table-container::-webkit-scrollbar-thumb:hover {
    background: var(--surface1);
  }

  .table-container::-webkit-scrollbar-corner {
    background: var(--mantle);
  }

  /* When both scrollbars are present, add some padding to prevent overlap */
  .table-container::-webkit-scrollbar-corner {
    background-color: var(--mantle);
  }

  table {
    width: max-content;
    min-width: 100%;
    table-layout: fixed;
    border-collapse: collapse;
    font-size: 13px;
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

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 0;
  }

  .high-usage {
    background-color: color-mix(in srgb, var(--red) 10%, transparent);
  }

  .high-usage:hover {
    background-color: color-mix(in srgb, var(--red) 15%, transparent);
  }

  tr.pinned {
    background-color: color-mix(in srgb, var(--blue) 10%, transparent);
  }

  tr.pinned:hover {
    background-color: color-mix(in srgb, var(--blue) 15%, transparent);
  }

  td:last-child {
    width: 120px;
    min-width: 120px;
    max-width: 120px;
    padding: 6px 8px;
  }

  .name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
