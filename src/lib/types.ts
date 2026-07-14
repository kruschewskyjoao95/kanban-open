export interface Board {
  id: string;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface List {
  id: string;
  board_id: string;
  title: string;
  position: number;
  created_at: string;
}

export interface Label {
  id: string;
  board_id: string;
  name: string;
  color: string;
  created_at: string;
}

export interface ChecklistItem {
  id: string;
  card_id: string;
  title: string;
  done: boolean;
  position: number;
  created_at: string;
}

export interface Card {
  id: string;
  list_id: string;
  title: string;
  description: string;
  position: number;
  created_at: string;
  updated_at: string;
  labels: Label[];
  checklist: ChecklistItem[];
}

export interface ListWithCards extends List {
  cards: Card[];
}

export interface BoardFull {
  board: Board;
  labels: Label[];
  lists: ListWithCards[];
}

/** Palette for new labels (hex). */
export const LABEL_COLORS = [
  "#ef4444",
  "#f97316",
  "#eab308",
  "#22c55e",
  "#14b8a6",
  "#3b82f6",
  "#8b5cf6",
  "#ec4899",
  "#64748b",
] as const;
