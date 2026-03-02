import { useEffect, useCallback } from 'react';

interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  action: () => void;
  description: string;
}

export function useKeyboardShortcuts(shortcuts: KeyboardShortcut[] = [], enabled = true) {
  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    if (!enabled) return;

    // Don't trigger shortcuts when typing in inputs
    const target = event.target as HTMLElement;
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
      return;
    }

    for (const shortcut of shortcuts) {
      const ctrlMatch = shortcut.ctrl ? (event.ctrlKey || event.metaKey) : !(event.ctrlKey || event.metaKey);
      const shiftMatch = shortcut.shift ? event.shiftKey : !event.shiftKey;
      const altMatch = shortcut.alt ? event.altKey : !event.altKey;
      const keyMatch = event.key.toLowerCase() === shortcut.key.toLowerCase();

      if (keyMatch && ctrlMatch && shiftMatch && altMatch) {
        event.preventDefault();
        shortcut.action();
        break;
      }
    }
  }, [shortcuts, enabled]);

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
}

export const DEFAULT_SHORTCUTS = {
  play: { key: ' ', description: 'Play/Pause' },
  stop: { key: 'Escape', description: 'Stop' },
  save: { key: 's', ctrl: true, description: 'Save project' },
  open: { key: 'o', ctrl: true, description: 'Open project' },
  new: { key: 'n', ctrl: true, description: 'New project' },
  undo: { key: 'z', ctrl: true, description: 'Undo' },
  redo: { key: 'y', ctrl: true, description: 'Redo' },
  copy: { key: 'c', ctrl: true, description: 'Copy notes' },
  paste: { key: 'v', ctrl: true, description: 'Paste notes' },
  delete: { key: 'Delete', description: 'Delete selected' },
  selectAll: { key: 'a', ctrl: true, description: 'Select all' },
  zoomIn: { key: '=', ctrl: true, description: 'Zoom in' },
  zoomOut: { key: '-', ctrl: true, description: 'Zoom out' },
  quantize: { key: 'q', description: 'Quantize selected' },
};
