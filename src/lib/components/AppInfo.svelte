<script lang="ts">
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import ThemeSwitcher from "./ThemeSwitcher.svelte";
  import { faInfo } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  let version = "";
  let showInfo = false;

  const ASCII_ART = `
    ███╗   ██╗███████╗ ██████╗ ██╗  ██╗████████╗ ██████╗ ██████╗ 
    ████╗  ██║██╔════╝██╔═══██╗██║  ██║╚══██╔══╝██╔═══██╗██╔══██╗
    ██╔██╗ ██║█████╗  ██║   ██║███████║   ██║   ██║   ██║██████╔╝
    ██║╚██╗██║██╔══╝  ██║   ██║██╔══██║   ██║   ██║   ██║██╔═══╝ 
    ██║ ╚████║███████╗╚██████╔╝██║  ██║   ██║   ╚██████╔╝██║     
    ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝     
  `;

  const APP_INFO = {
    name: "NeoHtop",
    developer: "Abdenasser",
    github: "https://github.com/abdenasser",
    stack: ["Tauri", "Rust", "Svelte", "TypeScript"],
  };

  onMount(async () => {
    version = await getVersion();
  });
</script>

<div class="app-info">
  <ThemeSwitcher />
  <button
    class="info-button"
    on:click={() => (showInfo = !showInfo)}
    aria-label="Toggle app info"
  >
    <span class="icon">
      <Fa icon={faInfo} />
    </span>
  </button>

  {#if showInfo}
    <div class="info-panel" on:mouseleave={() => (showInfo = false)}>
      <div class="info-content">
        <pre class="ascii-art">{ASCII_ART}</pre>
        <div class="details">
          <div class="detail-row">
            <span>NeoHtop v{version}</span>
          </div>
          <div class="detail-row">
            <span class="label">app</span>
            <span class="separator">::</span>
            <span class="value">{APP_INFO.name}</span>
          </div>
          <div class="detail-row">
            <span class="label">developer</span>
            <span class="separator">::</span>
            <span class="value">{APP_INFO.github}</span>
          </div>
          <div class="detail-row">
            <span class="label">stack</span>
            <span class="separator">::</span>
            <span class="value">{APP_INFO.stack.join(", ")}</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .app-info {
    display: flex;
    gap: 8px;
    position: relative;
  }

  .info-button,
  :global(.theme-button) {
    height: 31px;
    padding: 6px 12px;
    font-size: 12px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--text);
    background: var(--surface0);
    border: 1px solid var(--surface1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .info-button:hover,
  :global(.theme-button:hover) {
    background: var(--surface1);
  }

  .icon {
    display: inline-flex;
    align-items: center;
    font-size: 10px;
    color: var(--subtext0);
  }

  .info-panel {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    padding: 16px;
    background: var(--base);
    border: 1px solid var(--surface0);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    z-index: 100;
    min-width: 400px;
  }

  .info-content {
    display: flex;
    gap: 24px;
  }

  .ascii-art {
    font-family: monospace;
    font-size: 8px;
    line-height: 1;
    color: var(--mauve);
    margin: 0;
    padding: 0;
    white-space: pre;
  }

  .details {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px 0;
  }

  .detail-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: monospace;
    font-size: 13px;
  }

  .label {
    color: var(--green);
    min-width: 80px;
  }

  .separator {
    color: var(--subtext0);
  }

  .value {
    color: var(--text);
  }

  .link {
    color: var(--blue);
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .link:hover {
    color: var(--sky);
    text-decoration: underline;
  }

  .detail-row span {
    color: var(--text);
    font-weight: 500;
  }

  a {
    color: var(--blue);
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>
