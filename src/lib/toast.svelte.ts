export type ToastKind = "success" | "error" | "info";

export type ToastItem = {
  id: number;
  kind: ToastKind;
  message: string;
};

let seq = 0;

class ToastBus {
  items = $state<ToastItem[]>([]);

  push(kind: ToastKind, message: string, ms = 3200) {
    const id = ++seq;
    this.items = [...this.items, { id, kind, message }];
    window.setTimeout(() => this.dismiss(id), ms);
  }

  success(message: string) {
    this.push("success", message);
  }

  error(message: string) {
    this.push("error", message, 4500);
  }

  info(message: string) {
    this.push("info", message);
  }

  dismiss(id: number) {
    this.items = this.items.filter((t) => t.id !== id);
  }
}

export const toasts = new ToastBus();

/** Normaliza erros do invoke Tauri / JS. */
export function errMsg(e: unknown): string {
  if (typeof e === "string") return e;
  if (e instanceof Error) return e.message;
  return String(e);
}
