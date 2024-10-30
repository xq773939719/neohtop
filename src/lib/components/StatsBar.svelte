<script lang="ts">
  import Fa from "svelte-fa";
  import {
    faMicrochip,
    faMemory,
    faServer,
  } from "@fortawesome/free-solid-svg-icons";
  import type { SystemStats } from "$lib/types";
  import {
    formatUptime,
    formatMemorySize,
    formatPercentage,
    getUsageClass,
  } from "$lib/utils";

  export let systemStats: SystemStats | null = null;

  $: memoryPercentage = systemStats
    ? (systemStats.memory_used / systemStats.memory_total) * 100
    : 0;
</script>

<div class="stats-bar">
  {#if systemStats}
    <div class="stats-grid">
      <div class="stat-box">
        <div class="stat-header">
          <div class="stat-title">
            <Fa icon={faMicrochip} />
            <span>CPU Usage</span>
          </div>
        </div>
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
        <div class="stat-header">
          <div class="stat-title">
            <Fa icon={faMemory} />
            <span>Memory</span>
          </div>
        </div>
        <div class="memory-info">
          <div class="progress-bar">
            <div
              class="progress-fill {getUsageClass(memoryPercentage)}"
              style="width: {memoryPercentage}%"
            ></div>
          </div>
          <div class="memory-details">
            <div class="memory-row">
              <span>Total:</span>
              <span>{formatMemorySize(systemStats.memory_total)}</span>
            </div>
            <div class="memory-row">
              <span>Used:</span>
              <span
                >{formatMemorySize(systemStats.memory_used)} ({formatPercentage(
                  memoryPercentage,
                )})</span
              >
            </div>
            <div class="memory-row">
              <span>Free:</span>
              <span>{formatMemorySize(systemStats.memory_free)}</span>
            </div>
            <div class="memory-row">
              <span>Cached:</span>
              <span>{formatMemorySize(systemStats.memory_cached)}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="stat-box">
        <div class="stat-header">
          <div class="stat-title">
            <Fa icon={faServer} />
            <span>System Info</span>
          </div>
        </div>
        <div class="system-info">
          <div class="info-row">
            <span>Uptime:</span>
            <span>{formatUptime(systemStats.uptime)}</span>
          </div>
          <div class="info-row">
            <span>Load Average:</span>
            <span>{systemStats.load_avg[0].toFixed(2)} (1m)</span>
          </div>
          <div class="info-row">
            <span></span>
            <span>{systemStats.load_avg[1].toFixed(2)} (5m)</span>
          </div>
          <div class="info-row">
            <span></span>
            <span>{systemStats.load_avg[2].toFixed(2)} (15m)</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .stats-bar {
    background-color: var(--mantle);
    border-bottom: 1px solid var(--surface0);
    padding: 12px;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
    padding: 12px;
    min-width: max-content;
  }

  .stat-box {
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 8px;
    padding: 16px;
  }

  .stat-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--surface0);
  }

  .stat-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: 0.5px;
  }

  .stat-title :global(svg) {
    width: 14px;
    height: 14px;
    color: var(--subtext0);
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

  .memory-details {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-top: 8px;
  }

  .memory-row,
  .info-row {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: var(--subtext0);
  }

  .memory-row span:first-child,
  .info-row span:first-child {
    color: var(--subtext1);
    min-width: 80px;
  }
</style>
