/** Focus element after mount (avoids a11y autofocus warning during build). */
export function focusOnMount(node: HTMLElement) {
  queueMicrotask(() => node.focus());
}
