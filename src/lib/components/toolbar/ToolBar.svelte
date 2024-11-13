<script lang="ts">
  import AppInfo from "../AppInfo.svelte";
  import { statusMap } from "$lib/utils";
  import Fa from "svelte-fa";
  import {
    faPlay,
    faPause,
    faChevronDown,
    faChevronRight,
  } from "@fortawesome/free-solid-svg-icons";
  import { configStore } from "$lib/stores/config";
  import type { AppConfig } from "$lib/types/config";
  import StatusFilter from "./StatusFilter.svelte";
  import SearchBox from "./SearchBox.svelte";
  import RefreshControls from "./RefreshControls.svelte";
  import PaginationControls from "./PaginationControls.svelte";
  export let searchTerm: string;
  export let statusFilter: string = "all";
  export let itemsPerPage: number;
  export let currentPage: number;
  export let totalPages: number;
  export let totalResults: number;
  export let columns: Array<{
    id: string;
    label: string;
    visible: boolean;
    required?: boolean;
  }>;
  export let refreshRate: number;
  export let isFrozen: boolean;

  let showColumnMenu = false;
  const statusOptions = [
    { value: "all", label: "All Statuses" },
    ...Object.values(statusMap).map((status) => ({
      value: status.label,
      label: status.label,
    })),
  ];

  function handleColumnVisibilityChange(columnId: string, visible: boolean) {
    configStore.updateConfig({
      appearance: {
        columnVisibility: {
          ...$configStore.appearance.columnVisibility,
          [columnId]: visible,
        },
      },
    });
  }

  function updateBehaviorConfig(key: keyof AppConfig["behavior"], value: any) {
    configStore.updateConfig({
      behavior: {
        ...$configStore.behavior,
        [key]: value,
      },
    });
  }
</script>

<div class="toolbar">
  <div class="toolbar-content">
    <SearchBox bind:searchTerm />
    <StatusFilter bind:statusFilter />

    <div class="toolbar-spacer"></div>

    <PaginationControls
      bind:itemsPerPage
      bind:currentPage
      {totalPages}
      {totalResults}
    />
    <div class="toolbar-spacer"></div>

    <div class="column-toggle">
      <button
        class="btn-toggle"
        on:click={() => (showColumnMenu = !showColumnMenu)}
        aria-label="Toggle columns"
      >
        Columns
        <span class="icon">
          {#if showColumnMenu}
            <Fa icon={faChevronDown} />
          {:else}
            <Fa icon={faChevronRight} />
          {/if}
        </span>
      </button>

      {#if showColumnMenu}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="column-menu" on:mouseleave={() => (showColumnMenu = false)}>
          {#each columns as column}
            <label class="column-option">
              <input
                type="checkbox"
                checked={column.visible}
                disabled={column.required}
                on:change={(e) =>
                  handleColumnVisibilityChange(
                    column.id,
                    e.currentTarget.checked,
                  )}
              />
              <span>{column.label}</span>
            </label>
          {/each}
        </div>
      {/if}
    </div>

    <RefreshControls bind:refreshRate bind:isFrozen />

    <AppInfo />
  </div>
</div>

<style>
  .toolbar {
    padding: 8px;
    border-bottom: 1px solid var(--surface0);
    background-color: var(--mantle);
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

  .search-input-wrapper {
    position: relative;
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .search-input {
    width: 240px;
    height: 28px;
    padding: 0 12px;
    padding-right: 70px;
    border: 1px solid var(--surface1);
    border-radius: 6px;
    font-size: 12px;
    background-color: var(--surface0);
    color: var(--text);
    transition: all 0.2s ease;
  }

  .btn-clear {
    position: absolute;
    right: 4px;
    top: 50%;
    transform: translateY(-50%);
    padding: 4px 8px;
    font-size: 11px;
    color: var(--subtext0);
    background: var(--surface1);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-clear:hover {
    background: var(--surface2);
    color: var(--text);
  }

  .search-input::-webkit-search-cancel-button {
    -webkit-appearance: none;
    height: 14px;
    width: 14px;
    margin-right: 4px;
    background: var(--overlay0);
    -webkit-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cline x1='18' y1='6' x2='6' y2='18'%3E%3C/line%3E%3Cline x1='6' y1='6' x2='18' y2='18'%3E%3C/line%3E%3C/svg%3E")
      no-repeat 50% 50%;
    mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='currentColor' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cline x1='18' y1='6' x2='6' y2='18'%3E%3C/line%3E%3Cline x1='6' y1='6' x2='18' y2='18'%3E%3C/line%3E%3C/svg%3E")
      no-repeat 50% 50%;
    cursor: pointer;
  }

  .search-input::-webkit-search-cancel-button:hover {
    background: var(--text);
  }

  .search-input:hover {
    background-color: var(--surface1);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--blue);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--blue) 25%, transparent);
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
    max-height: 300px;
    overflow-y: auto;
    min-width: 200px;
  }

  .column-menu::-webkit-scrollbar {
    width: 8px;
  }

  .column-menu::-webkit-scrollbar-track {
    background: var(--mantle);
    border-radius: 4px;
  }

  .column-menu::-webkit-scrollbar-thumb {
    background: var(--surface2);
    border-radius: 4px;
  }

  .column-menu::-webkit-scrollbar-thumb:hover {
    background: var(--surface1);
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

  .refresh-controls {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .refresh-controls :global(svg) {
    font-size: 14px;
    color: var(--subtext0);
  }

  .btn-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: var(--surface0);
    color: var(--text);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-action:hover {
    background: var(--surface1);
  }

  .btn-action.frozen {
    background: var(--blue);
    color: var(--base);
  }

  .select-input {
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--surface1);
    border-radius: 6px;
    background: var(--surface0);
    color: var(--text);
    font-size: 13px;
    cursor: pointer;
  }

  .select-input:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
</style>
