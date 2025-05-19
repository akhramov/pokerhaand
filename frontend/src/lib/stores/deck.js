// src/lib/stores/deck.js
import { writable } from 'svelte/store';
import { get } from 'svelte/store';

export const currentDeck = writable(null);
export const currentOffset = writable(0);
export const currentHand = writable(null);
export const error = writable(null);

const API_BASE = 'http://localhost:8080/api/v1';

export async function createNewDeck() {
  error.set(null);
  currentHand.set(null);

  try {
    const response = await fetch(`${API_BASE}/decks`, {
      method: 'POST'
    });

    if (!response.ok) throw new Error('Failed to create deck');

    const { id } = await response.json();
    currentDeck.set(id);
    currentOffset.set(0);
    return id;
  } catch (err) {
    error.set(err.message);
    throw err;
  }
}

export async function fetchHand(offset = 0) {
  error.set(null);

  try {
    const deckId = get(currentDeck);
    if (!deckId) throw new Error('No deck created');

    const response = await fetch(`${API_BASE}/decks/${deckId}?offset=${offset}`);

    if (!response.ok) throw new Error('Failed to fetch hand');

    const hand = await response.json();
    currentHand.set(hand);
    currentOffset.set(offset);
    return hand;
  } catch (err) {
    error.set(err.message);
    throw err;
  }
}
