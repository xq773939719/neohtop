<script lang="ts">
  import type { Process } from "$lib/types";
  import ProcessIcon from "../ProcessIcon.svelte";

  export let process: Process;
  export let field: keyof Process;
  export let format: ((value: any) => string) | undefined = undefined;

  function formatValue(value: any): string | undefined {
    if (format) {
      return format(value);
    }
    return String(value);
  }
</script>

<td class="truncate" data-column={field}>
  {#if field === "name"}
    <div class="name-cell">
      <ProcessIcon processName={process.name} />
      <span class="process-name">{process.name}</span>
    </div>
  {:else}
    {formatValue(process[field])}
  {/if}
</td>

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

  .truncate {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 0;
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

  tr.pinned {
    background-color: color-mix(in srgb, var(--blue) 10%, transparent);
  }

  tr.pinned:hover {
    background-color: color-mix(in srgb, var(--blue) 15%, transparent);
  }

  th:last-child {
    width: 120px;
    min-width: 120px;
    max-width: 120px;
  }

  td:last-child {
    width: 120px;
    min-width: 120px;
    max-width: 120px;
    padding: 6px 8px;
  }

  .col-actions {
    position: sticky;
    right: 0;
    z-index: 2;
    background: var(--base);
    border-left: 1px solid var(--surface0);
    width: 120px;
  }

  .action-buttons {
    display: flex;
    gap: 4px;
    align-items: center;
    justify-content: center;
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

  .info-btn {
    color: var(--lavender);
  }

  .info-btn::before {
    background: var(--lavender);
  }

  .kill-btn {
    color: var(--red);
    border: 1px solid color-mix(in srgb, var(--red) 30%, transparent);
  }

  .kill-btn:hover {
    color: var(--base);
    background: var(--red);
  }

  .kill-btn:hover::before {
    opacity: 1;
  }

  .btn-action:hover {
    box-shadow: 0 0 12px color-mix(in srgb, currentColor 20%, transparent);
  }

  .btn-action:active {
    transform: translateY(1px);
  }

  .pin-btn.pinned:active {
    transform: rotate(45deg) translateY(1px);
  }

  .btn-action:focus {
    outline: none;
    box-shadow: 0 0 0 2px color-mix(in srgb, currentColor 30%, transparent);
  }

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

  .process-icon {
    width: 16px;
    height: 16px;
    object-fit: contain;
  }

  .name-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
