export interface ColumnDefinition {
  id: string;
  label: string;
  visible: boolean;
  required?: boolean;
}

export interface StatusOption {
  value: string;
  label: string;
}

export interface RefreshRateOption {
  value: number;
  label: string;
}

export interface ToolBarProps {
  searchTerm: string;
  statusFilter: string;
  itemsPerPage: number;
  currentPage: number;
  totalPages: number;
  totalResults: number;
  columns: ColumnDefinition[];
  refreshRate: number;
  isFrozen: boolean;
}
