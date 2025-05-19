// src/lib/stores/history.js
import { writable, get } from 'svelte/store';

export const historyItems = writable([]);
export const historyNextOffset = writable(0);
export const historyError = writable(null);

const API_BASE = 'http://localhost:8080/api/v1';

export async function fetchHistory(offset = 0) {
  historyError.set(null);

  try {
    const response = await fetch(`${API_BASE}/history?offset=${offset}`);

    if (!response.ok) throw new Error('Failed to fetch history');

    const data = await response.json();

    if (offset === 0) {
      // Ved ny lasting erstatt items
      historyItems.set(data.items);
    } else {
      // Append items på eksisterende liste
      historyItems.update(items => [...items, ...data.items]);
    }

    historyNextOffset.set(data.next_offset ?? null);
  } catch (err) {
    historyError.set(err.message);
  }
}
