/** Shared drag payload for cards. */
export type CardDrag = {
  cardId: string;
  fromListId: string;
};

/** Shared drag payload for lists (columns). */
export type ListDrag = {
  listId: string;
};

type ActiveDrag =
  | { kind: "card"; data: CardDrag }
  | { kind: "list"; data: ListDrag };

/**
 * In-memory drag state — WebKitGTK often cannot read custom MIME types
 * during dragover/drop. Same-window state always works.
 */
let active: ActiveDrag | null = null;

/** Subscribers notified when card-drag starts/ends (for stable drop UI). */
const cardDragListeners = new Set<(dragging: boolean) => void>();

export function onCardDragChange(fn: (dragging: boolean) => void): () => void {
  cardDragListeners.add(fn);
  fn(active?.kind === "card");
  return () => cardDragListeners.delete(fn);
}

function notifyCardDrag() {
  const dragging = active?.kind === "card";
  for (const fn of cardDragListeners) fn(dragging);
}

const DRAG_SCALE = 0.5;

export function clearDrag() {
  const wasCard = active?.kind === "card";
  active = null;
  if (wasCard) notifyCardDrag();
}

export function isCardDrag(_e?: DragEvent): boolean {
  return active?.kind === "card";
}

export function isListDrag(_e?: DragEvent): boolean {
  return active?.kind === "list";
}

export function getCardDrag(_e?: DragEvent): CardDrag | null {
  return active?.kind === "card" ? active.data : null;
}

export function getListDrag(_e?: DragEvent): ListDrag | null {
  return active?.kind === "list" ? active.data : null;
}

function trySetScaledDragImage(e: DragEvent, source: HTMLElement, scale = DRAG_SCALE) {
  try {
    if (!e.dataTransfer) return;

    const rect = source.getBoundingClientRect();
    if (rect.width < 2 || rect.height < 2) return;

    const ghost = document.createElement("div");
    ghost.textContent = source.innerText?.trim().slice(0, 80) || "…";
    ghost.setAttribute("aria-hidden", "true");

    const w = Math.max(80, Math.round(rect.width * scale));
    ghost.style.cssText = [
      "position: fixed",
      "left: -9999px",
      "top: 0",
      `width: ${w}px`,
      "padding: 6px 8px",
      "font: 11px/1.3 system-ui, sans-serif",
      "color: #e2e8f0",
      "background: #273449",
      "border: 1px solid #334155",
      "border-radius: 5px",
      "box-shadow: 0 4px 14px rgba(0,0,0,0.4)",
      "opacity: 0.95",
      "pointer-events: none",
      "z-index: 99999",
      "white-space: nowrap",
      "overflow: hidden",
      "text-overflow: ellipsis",
    ].join(";");

    document.body.appendChild(ghost);
    e.dataTransfer.setDragImage(ghost, Math.round(w / 2), 12);

    requestAnimationFrame(() => {
      requestAnimationFrame(() => ghost.remove());
    });
  } catch {
    // ignore
  }
}

export function setCardDrag(e: DragEvent, data: CardDrag, source?: HTMLElement) {
  active = { kind: "card", data };
  notifyCardDrag();

  if (e.dataTransfer) {
    try {
      e.dataTransfer.setData(
        "text/plain",
        JSON.stringify({ kind: "card", ...data }),
      );
      e.dataTransfer.effectAllowed = "move";
    } catch {
      // ignore
    }
  }

  if (source) {
    try {
      source.blur();
    } catch {
      /* ignore */
    }
    trySetScaledDragImage(e, source, DRAG_SCALE);
  }
}

export function setListDrag(e: DragEvent, data: ListDrag, source?: HTMLElement) {
  active = { kind: "list", data };
  notifyCardDrag(); // clears card-drag UI if any

  if (e.dataTransfer) {
    try {
      e.dataTransfer.setData(
        "text/plain",
        JSON.stringify({ kind: "list", ...data }),
      );
      e.dataTransfer.effectAllowed = "move";
    } catch {
      // ignore
    }
  }

  if (source) {
    try {
      source.blur();
    } catch {
      /* ignore */
    }
    trySetScaledDragImage(e, source, DRAG_SCALE);
  }
}

export function allowDrop(e: DragEvent) {
  e.preventDefault();
  if (e.dataTransfer) {
    try {
      e.dataTransfer.dropEffect = "move";
    } catch {
      // ignore
    }
  }
}
