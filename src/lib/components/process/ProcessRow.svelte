<script lang="ts">
  import type { Process, Column } from "$lib/types";
  import ProcessCell from "./cells/ProcessCell.svelte";
  import ActionButtons from "./ActionButtons.svelte";

  export let process: Process;
  export let columns: Column[];
  export let isPinned: boolean;
  export let isHighUsage: boolean;
  export let onTogglePin: (command: string) => void;
  export let onShowDetails: (process: Process) => void;
  export let onKillProcess: (process: Process) => void;
</script>

<tr class:high-usage={isHighUsage} class:pinned={isPinned}>
  {#each columns.filter((col) => col.visible) as column}
    <ProcessCell {process} field={column.id} format={column.format} />
  {/each}
  <td class="col-actions">
    <ActionButtons
      {process}
      {isPinned}
      {onTogglePin}
      {onShowDetails}
      {onKillProcess}
    />
  </td>
</tr>

<style>
  tr:hover {
    background-color: var(--surface0);
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

  .col-actions {
    position: sticky;
    right: 0;
    z-index: 2;
    background: var(--base);
    border-left: 1px solid var(--surface0);
    width: 120px;
    padding: 6px 8px;
  }
</style>
