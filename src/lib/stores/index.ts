import { writable } from 'svelte/store';
import { themes, type Theme } from '$lib/styles';

function createThemeStore() {
  // Get initial theme from localStorage or default to catppuccin
  const storedTheme = typeof window !== 'undefined' 
    ? localStorage.getItem('theme') 
    : 'catppuccin';
    
  const { subscribe, set } = writable<Theme>(themes[storedTheme || 'catppuccin']);

  return {
    subscribe,
    setTheme: (themeName: string) => {
      const theme = themes[themeName];
      if (theme) {
        localStorage.setItem('theme', themeName);
        set(theme);
        applyTheme(theme);
      }
    },
    init: () => {
      const theme = themes[storedTheme || 'catppuccin'];
      applyTheme(theme);
    }
  };
}

function applyTheme(theme: Theme) {
  const root = document.documentElement;
  Object.entries(theme.colors).forEach(([key, value]) => {
    root.style.setProperty(`--${key}`, value);
  });
}

export const themeStore = createThemeStore();
