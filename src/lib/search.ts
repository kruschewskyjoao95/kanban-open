import type { Card, ListWithCards } from "./types";

/** Case-insensitive match on title, description, or label names. */
export function matchesCard(card: Card, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;

  if (card.title.toLowerCase().includes(q)) return true;
  if (card.description.toLowerCase().includes(q)) return true;
  if (card.labels?.some((l) => l.name.toLowerCase().includes(q))) return true;
  if (card.checklist?.some((i) => i.title.toLowerCase().includes(q))) return true;

  return false;
}

export function filterListCards(list: ListWithCards, query: string): ListWithCards {
  if (!query.trim()) return list;
  return {
    ...list,
    cards: list.cards.filter((c) => matchesCard(c, query)),
  };
}

export function countMatchingCards(lists: ListWithCards[], query: string): number {
  if (!query.trim()) {
    return lists.reduce((n, l) => n + l.cards.length, 0);
  }
  return lists.reduce(
    (n, l) => n + l.cards.filter((c) => matchesCard(c, query)).length,
    0,
  );
}
