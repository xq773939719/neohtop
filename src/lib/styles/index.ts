export interface Theme {
  name: string;
  label: string;
  colors: {
    base: string;
    mantle: string;
    crust: string;
    text: string;
    subtext0: string;
    subtext1: string;
    surface0: string;
    surface1: string;
    surface2: string;
    overlay0: string;
    overlay1: string;
    blue: string;
    lavender: string;
    sapphire: string;
    sky: string;
    red: string;
    maroon: string;
    peach: string;
    yellow: string;
    green: string;
    teal: string;
  };
}

export const themes: Record<string, Theme> = {
  catppuccin: {
    name: 'catppuccin',
    label: 'Catppuccin Mocha',
    colors: {
      base: '#1e1e2e',
      mantle: '#181825',
      crust: '#11111b',
      text: '#cdd6f4',
      subtext0: '#a6adc8',
      subtext1: '#bac2de',
      surface0: '#313244',
      surface1: '#45475a',
      surface2: '#585b70',
      overlay0: '#6c7086',
      overlay1: '#7f849c',
      blue: '#89b4fa',
      lavender: '#b4befe',
      sapphire: '#74c7ec',
      sky: '#89dceb',
      red: '#f38ba8',
      maroon: '#eba0ac',
      peach: '#fab387',
      yellow: '#f9e2af',
      green: '#a6e3a1',
      teal: '#94e2d5',
    },
  },
  dracula: {
    name: 'dracula',
    label: 'Dracula',
    colors: {
      base: '#282a36',
      mantle: '#1e1f29',
      crust: '#191a21',
      text: '#f8f8f2',
      subtext0: '#bfbfbf',
      subtext1: '#e6e6e6',
      surface0: '#44475a',
      surface1: '#6272a4',
      surface2: '#7970a9',
      overlay0: '#6272a4',
      overlay1: '#7970a9',
      blue: '#8be9fd',
      lavender: '#bd93f9',
      sapphire: '#62d6e8',
      sky: '#89ddff',
      red: '#ff5555',
      maroon: '#ff6e6e',
      peach: '#ffb86c',
      yellow: '#f1fa8c',
      green: '#50fa7b',
      teal: '#8be9fd',
    },
  },
  monokaiPro: {
    name: 'monokaiPro',
    label: 'Monokai Pro',
    colors: {
      base: '#2d2a2e',
      mantle: '#221f22',
      crust: '#1b1b1b',
      text: '#fcfcfa',
      subtext0: '#939293',
      subtext1: '#c1c0c0',
      surface0: '#403e41',
      surface1: '#565457',
      surface2: '#69676c',
      overlay0: '#727072',
      overlay1: '#848486',
      blue: '#78dce8',
      lavender: '#ab9df2',
      sapphire: '#66d9ef',
      sky: '#78dce8',
      red: '#ff6188',
      maroon: '#ff6188',
      peach: '#fc9867',
      yellow: '#ffd866',
      green: '#a9dc76',
      teal: '#78dce8',
    },
  },
  tokyoNight: {
    name: 'tokyoNight',
    label: 'Tokyo Night',
    colors: {
      base: '#1a1b26',
      mantle: '#16161e',
      crust: '#13131a',
      text: '#a9b1d6',
      subtext0: '#9aa5ce',
      subtext1: '#b4f9f8',
      surface0: '#232433',
      surface1: '#2a2b3d',
      surface2: '#32344a',
      overlay0: '#565f89',
      overlay1: '#6b7089',
      blue: '#7aa2f7',
      lavender: '#bb9af7',
      sapphire: '#7dcfff',
      sky: '#7dcfff',
      red: '#f7768e',
      maroon: '#ff9e64',
      peach: '#ff9e64',
      yellow: '#e0af68',
      green: '#9ece6a',
      teal: '#2ac3de',
    },
  },
  gruvbox: {
    name: 'gruvbox',
    label: 'Gruvbox Dark',
    colors: {
      base: '#282828',
      mantle: '#1d2021',
      crust: '#1b1b1b',
      text: '#ebdbb2',
      subtext0: '#a89984',
      subtext1: '#bdae93',
      surface0: '#3c3836',
      surface1: '#504945',
      surface2: '#665c54',
      overlay0: '#7c6f64',
      overlay1: '#928374',
      blue: '#83a598',
      lavender: '#d3869b',
      sapphire: '#83a598',
      sky: '#8ec07c',
      red: '#fb4934',
      maroon: '#cc241d',
      peach: '#fe8019',
      yellow: '#fabd2f',
      green: '#b8bb26',
      teal: '#8ec07c',
    },
  },
  nord: {
    name: 'nord',
    label: 'Nord',
    colors: {
      base: '#2e3440',
      mantle: '#272c36',
      crust: '#242933',
      text: '#eceff4',
      subtext0: '#d8dee9',
      subtext1: '#e5e9f0',
      surface0: '#3b4252',
      surface1: '#434c5e',
      surface2: '#4c566a',
      overlay0: '#616e88',
      overlay1: '#7b88a1',
      blue: '#88c0d0',
      lavender: '#b48ead',
      sapphire: '#81a1c1',
      sky: '#88c0d0',
      red: '#bf616a',
      maroon: '#d08770',
      peach: '#d08770',
      yellow: '#ebcb8b',
      green: '#a3be8c',
      teal: '#8fbcbb',
    },
  },
  oneDark: {
    name: 'oneDark',
    label: 'One Dark',
    colors: {
      base: '#282c34',
      mantle: '#21252b',
      crust: '#1b1f23',
      text: '#abb2bf',
      subtext0: '#828997',
      subtext1: '#9da5b4',
      surface0: '#31353f',
      surface1: '#393f4a',
      surface2: '#4b5263',
      overlay0: '#636d83',
      overlay1: '#767d8d',
      blue: '#61afef',
      lavender: '#c678dd',
      sapphire: '#56b6c2',
      sky: '#56b6c2',
      red: '#e06c75',
      maroon: '#be5046',
      peach: '#d19a66',
      yellow: '#e5c07b',
      green: '#98c379',
      teal: '#56b6c2',
    },
  },
  highContrast: {
    name: 'highContrast',
    label: 'High Contrast',
    colors: {
      base: '#000000',          // Pure black background
      mantle: '#0a0a0a',        // Slightly lighter black for layering
      crust: '#141414',         // Even lighter black for depth
      text: '#ffffff',          // Pure white text
      subtext0: '#e0e0e0',      // Very light grey for secondary text
      subtext1: '#f0f0f0',      // Almost white for important secondary text
      surface0: '#1a1a1a',      // Dark surface for contrast
      surface1: '#2a2a2a',      // Lighter surface for hover states
      surface2: '#3a3a3a',      // Even lighter surface for active states
      overlay0: '#4a4a4a',      // Medium grey for overlays
      overlay1: '#5a5a5a',      // Lighter grey for overlay hover states
      blue: '#00ffff',          // Cyan for primary actions
      lavender: '#ff00ff',      // Magenta for accents
      sapphire: '#00ccff',      // Bright blue for links
      sky: '#00ffee',           // Bright cyan for highlights
      red: '#ff0000',           // Pure red for errors/warnings
      maroon: '#ff3333',        // Lighter red for secondary warnings
      peach: '#ffaa00',         // Bright orange for notifications
      yellow: '#ffff00',        // Pure yellow for important highlights
      green: '#00ff00',         // Pure green for success states
      teal: '#00ffcc',          // Bright teal for special actions
    },
  },
};
