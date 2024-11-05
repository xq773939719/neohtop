<script lang="ts">
  import AppInfo from "./AppInfo.svelte";
  import { statusMap } from "$lib/utils";
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

  const itemsPerPageOptions = [15, 25, 50, 100, 250, 500];
  let showColumnMenu = false;
  const statusOptions = [
    { value: "all", label: "All Statuses" },
    ...Object.values(statusMap).map((status) => ({
      value: status.label,
      label: status.label,
    })),
  ];

  const refreshRateOptions = [
    { value: 1000, label: "1s" },
    { value: 2000, label: "2s" },
    { value: 5000, label: "5s" },
    { value: 10000, label: "10s" },
    { value: 30000, label: "30s" },
  ];

  function changePage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }
</script>

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
    <div class="toolbar-group">
      <select bind:value={statusFilter} class="select-input">
        {#each statusOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
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
          <span class="results-info">({totalResults} processes)</span>
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
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="column-menu" on:mouseleave={() => (showColumnMenu = false)}>
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

    <div class="toolbar-group">
      <div class="refresh-controls">
        <select
          class="select-input"
          bind:value={refreshRate}
          disabled={isFrozen}
        >
          {#each refreshRateOptions as option}
            <option value={option.value}>
              {option.label}
            </option>
          {/each}
        </select>
        <button
          class="btn-action"
          class:frozen={isFrozen}
          on:click={() => (isFrozen = !isFrozen)}
          title={isFrozen ? "Resume Updates" : "Pause Updates"}
        >
          {#if isFrozen}
            ▶
          {:else}
            ⏸
          {/if}
        </button>
      </div>
    </div>

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
    padding: 6px 12px;
    border: 1px solid var(--surface1);
    border-radius: 6px;
    font-size: 12px;
    background-color: var(--surface0);
    color: var(--text);
    transition: all 0.2s ease;
  }

  .search-input:hover {
    background-color: var(--surface1);
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
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .btn-clear:hover {
    background: var(--surface1);
    border-color: var(--surface2);
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

  .refresh-controls {
    display: flex;
    gap: 8px;
    align-items: center;
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
