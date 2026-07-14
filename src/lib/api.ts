import { invoke as tauriInvoke, isTauri } from "@tauri-apps/api/core";
import type { Board, BoardFull, Card, ChecklistItem, Label, List } from "./types";

const NOT_TAURI_MSG =
  "O app precisa rodar dentro do Tauri. Use: npm run tauri dev  (não abra só o browser em localhost:1420)";

/** True when running inside the Tauri webview (desktop shell). */
export function runningInTauri(): boolean {
  try {
    return isTauri();
  } catch {
    return false;
  }
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!runningInTauri()) {
    throw new Error(NOT_TAURI_MSG);
  }
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (e) {
    // Re-throw with clearer message if internals are missing mid-session
    const msg = e instanceof Error ? e.message : String(e);
    if (msg.includes("__TAURI_INTERNALS__") || msg.includes("reading 'invoke'")) {
      throw new Error(NOT_TAURI_MSG);
    }
    throw e;
  }
}

export const api = {
  listBoards: () => invoke<Board[]>("list_boards"),

  createBoard: (name: string) =>
    invoke<Board>("create_board", { input: { name } }),

  getBoard: (id: string) => invoke<BoardFull>("get_board", { id }),

  renameBoard: (id: string, name: string) =>
    invoke<Board>("rename_board", { input: { id, name } }),

  deleteBoard: (id: string) => invoke<void>("delete_board", { id }),

  createList: (boardId: string, title: string) =>
    invoke<List>("create_list", { input: { board_id: boardId, title } }),

  renameList: (id: string, title: string) =>
    invoke<List>("rename_list", { input: { id, title } }),

  deleteList: (id: string) => invoke<void>("delete_list", { id }),

  reorderList: (id: string, position: number) =>
    invoke<List>("reorder_list", { input: { id, position } }),

  createCard: (listId: string, title: string, description?: string) =>
    invoke<Card>("create_card", {
      input: { list_id: listId, title, description: description ?? null },
    }),

  updateCard: (id: string, title?: string, description?: string) =>
    invoke<Card>("update_card", {
      input: {
        id,
        title: title ?? null,
        description: description ?? null,
      },
    }),

  deleteCard: (id: string) => invoke<void>("delete_card", { id }),

  moveCard: (id: string, listId: string, position: number) =>
    invoke<Card>("move_card", {
      input: { id, list_id: listId, position },
    }),

  listLabels: (boardId: string) =>
    invoke<Label[]>("list_labels", { board_id: boardId }),

  createLabel: (boardId: string, name: string, color: string) =>
    invoke<Label>("create_label", {
      input: { board_id: boardId, name, color },
    }),

  updateLabel: (id: string, name?: string, color?: string) =>
    invoke<Label>("update_label", {
      input: {
        id,
        name: name ?? null,
        color: color ?? null,
      },
    }),

  deleteLabel: (id: string) => invoke<void>("delete_label", { id }),

  setCardLabels: (cardId: string, labelIds: string[]) =>
    invoke<Card>("set_card_labels", {
      input: { card_id: cardId, label_ids: labelIds },
    }),

  createChecklistItem: (cardId: string, title: string) =>
    invoke<ChecklistItem>("create_checklist_item", {
      input: { card_id: cardId, title },
    }),

  updateChecklistItem: (id: string, patch: { title?: string; done?: boolean }) =>
    invoke<ChecklistItem>("update_checklist_item", {
      input: {
        id,
        title: patch.title ?? null,
        done: patch.done ?? null,
      },
    }),

  deleteChecklistItem: (id: string) => invoke<void>("delete_checklist_item", { id }),
};

/** Midpoint position between two neighbors (Trello-style fractional indexing). */
export function positionBetween(before?: number, after?: number): number {
  if (before === undefined && after === undefined) return 0;
  if (before === undefined) return after! - 1;
  if (after === undefined) return before + 1;
  return (before + after) / 2;
}
