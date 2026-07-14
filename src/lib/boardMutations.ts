import type { BoardFull, Card, ChecklistItem, Label, ListWithCards } from "./types";

/**
 * Deep clone board state for optimistic rollback.
 * NOTE: Do NOT use structuredClone — Svelte 5 $state proxies throw
 * "The object can not be cloned."
 */
export function cloneBoard(data: BoardFull): BoardFull {
  return JSON.parse(JSON.stringify(data)) as BoardFull;
}

/** Apply card move locally (optimistic). Returns new board or null if card missing. */
export function applyMoveCard(
  data: BoardFull,
  cardId: string,
  toListId: string,
  position: number,
): BoardFull | null {
  const next = cloneBoard(data);
  let card: Card | null = null;

  for (const list of next.lists) {
    const idx = list.cards.findIndex((c) => c.id === cardId);
    if (idx >= 0) {
      card = list.cards[idx];
      list.cards.splice(idx, 1);
      break;
    }
  }

  if (!card) return null;

  const target = next.lists.find((l) => l.id === toListId);
  if (!target) return null;

  card = {
    ...card,
    list_id: toListId,
    position,
    updated_at: new Date().toISOString(),
  };
  target.cards.push(card);
  target.cards.sort((a, b) => a.position - b.position);

  return next;
}

export function applyAddCard(data: BoardFull, card: Card): BoardFull {
  const next = cloneBoard(data);
  const list = next.lists.find((l) => l.id === card.list_id);
  if (list) {
    list.cards.push({
      ...card,
      labels: card.labels ?? [],
      checklist: card.checklist ?? [],
    });
    list.cards.sort((a, b) => a.position - b.position);
  }
  return next;
}

export function applyDeleteCard(data: BoardFull, cardId: string): BoardFull {
  const next = cloneBoard(data);
  for (const list of next.lists) {
    list.cards = list.cards.filter((c) => c.id !== cardId);
  }
  return next;
}

export function applyUpdateCard(data: BoardFull, card: Card): BoardFull {
  const next = cloneBoard(data);
  for (const list of next.lists) {
    const idx = list.cards.findIndex((c) => c.id === card.id);
    if (idx >= 0) {
      list.cards[idx] = card;
      break;
    }
  }
  return next;
}

export function applyAddList(data: BoardFull, list: ListWithCards): BoardFull {
  const next = cloneBoard(data);
  next.lists.push(list);
  next.lists.sort((a, b) => a.position - b.position);
  return next;
}

export function applyRenameList(data: BoardFull, listId: string, title: string): BoardFull {
  const next = cloneBoard(data);
  const list = next.lists.find((l) => l.id === listId);
  if (list) list.title = title;
  return next;
}

export function applyDeleteList(data: BoardFull, listId: string): BoardFull {
  const next = cloneBoard(data);
  next.lists = next.lists.filter((l) => l.id !== listId);
  return next;
}

export function applyReorderList(
  data: BoardFull,
  listId: string,
  position: number,
): BoardFull {
  const next = cloneBoard(data);
  const list = next.lists.find((l) => l.id === listId);
  if (list) {
    list.position = position;
    next.lists.sort((a, b) => a.position - b.position);
  }
  return next;
}

export function applyRenameBoard(data: BoardFull, name: string): BoardFull {
  const next = cloneBoard(data);
  next.board.name = name;
  next.board.updated_at = new Date().toISOString();
  return next;
}

export function applyAddLabel(data: BoardFull, label: Label): BoardFull {
  const next = cloneBoard(data);
  next.labels = [...next.labels, label].sort((a, b) =>
    a.name.localeCompare(b.name, "pt-BR"),
  );
  return next;
}

export function applyUpdateLabel(data: BoardFull, label: Label): BoardFull {
  const next = cloneBoard(data);
  next.labels = next.labels.map((l) => (l.id === label.id ? label : l));
  for (const list of next.lists) {
    for (const card of list.cards) {
      card.labels = card.labels.map((l) => (l.id === label.id ? label : l));
    }
  }
  return next;
}

export function applyDeleteLabel(data: BoardFull, labelId: string): BoardFull {
  const next = cloneBoard(data);
  next.labels = next.labels.filter((l) => l.id !== labelId);
  for (const list of next.lists) {
    for (const card of list.cards) {
      card.labels = card.labels.filter((l) => l.id !== labelId);
    }
  }
  return next;
}

export function applySetCardLabels(
  data: BoardFull,
  cardId: string,
  labels: Label[],
): BoardFull {
  const next = cloneBoard(data);
  for (const list of next.lists) {
    const idx = list.cards.findIndex((c) => c.id === cardId);
    if (idx >= 0) {
      list.cards[idx] = {
        ...list.cards[idx],
        labels: [...labels].sort((a, b) => a.name.localeCompare(b.name, "pt-BR")),
        updated_at: new Date().toISOString(),
      };
      break;
    }
  }
  return next;
}

function mapCard(
  data: BoardFull,
  cardId: string,
  fn: (card: Card) => Card,
): BoardFull {
  const next = cloneBoard(data);
  for (const list of next.lists) {
    const idx = list.cards.findIndex((c) => c.id === cardId);
    if (idx >= 0) {
      list.cards[idx] = fn(list.cards[idx]);
      break;
    }
  }
  return next;
}

export function applyAddChecklistItem(
  data: BoardFull,
  item: ChecklistItem,
): BoardFull {
  return mapCard(data, item.card_id, (card) => ({
    ...card,
    checklist: [...(card.checklist ?? []), item].sort(
      (a, b) => a.position - b.position,
    ),
    updated_at: new Date().toISOString(),
  }));
}

export function applyUpdateChecklistItem(
  data: BoardFull,
  item: ChecklistItem,
): BoardFull {
  return mapCard(data, item.card_id, (card) => ({
    ...card,
    checklist: (card.checklist ?? []).map((c) => (c.id === item.id ? item : c)),
    updated_at: new Date().toISOString(),
  }));
}

export function applyDeleteChecklistItem(
  data: BoardFull,
  cardId: string,
  itemId: string,
): BoardFull {
  return mapCard(data, cardId, (card) => ({
    ...card,
    checklist: (card.checklist ?? []).filter((c) => c.id !== itemId),
    updated_at: new Date().toISOString(),
  }));
}
