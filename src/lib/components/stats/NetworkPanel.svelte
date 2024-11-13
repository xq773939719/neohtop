<script lang="ts">
  import { faNetworkWired } from "@fortawesome/free-solid-svg-icons";
  import PanelHeader from "./PanelHeader.svelte";
  import StatItem from "./StatItem.svelte";

  export let networkRxBytes: number;
  export let networkTxBytes: number;

  function formatBytes(bytes: number): string {
    const units = ["B", "KB", "MB", "GB", "TB"];
    let value = bytes;
    let unitIndex = 0;

    while (value >= 1024 && unitIndex < units.length - 1) {
      value /= 1024;
      unitIndex++;
    }

    return `${value.toFixed(1)} ${units[unitIndex]}/s`;
  }
</script>

<div class="stat-panel">
  <PanelHeader icon={faNetworkWired} title="Network I/O" />
  <div class="network-stats">
    <StatItem label="↓ Receiving" value={formatBytes(networkRxBytes)} />
    <StatItem label="↑ Sending" value={formatBytes(networkTxBytes)} />
  </div>
</div>

<style>
  .stat-panel {
    flex: 0.8;
    min-width: 125px;
    background-color: var(--mantle);
    border-radius: 6px;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
  }

  .network-stats {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }
</style>
