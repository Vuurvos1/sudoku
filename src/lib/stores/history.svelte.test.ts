import { describe, it, expect } from 'vitest';
import { HistoryStore } from './history.svelte';

describe('HistoryStore', () => {
	it('starts empty with nothing to undo or redo', () => {
		const history = new HistoryStore<number>();
		expect(history.canUndo).toBe(false);
		expect(history.canRedo).toBe(false);
		expect(history.current).toBeUndefined();
	});

	it('tracks current state after pushState', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		expect(history.current).toBe(1);
		expect(history.canUndo).toBe(false);
		expect(history.canRedo).toBe(false);
	});

	it('can undo after multiple pushes', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		history.pushState(2);
		history.pushState(3);

		expect(history.current).toBe(3);
		expect(history.canUndo).toBe(true);

		expect(history.undo()).toBe(2);
		expect(history.current).toBe(2);

		expect(history.undo()).toBe(1);
		expect(history.current).toBe(1);
	});

	it('cannot undo past the first state', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		history.pushState(2);

		history.undo();
		expect(history.canUndo).toBe(false);
		expect(history.undo()).toBeUndefined();
		expect(history.current).toBe(1);
	});

	it('can redo after undo', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		history.pushState(2);
		history.pushState(3);

		history.undo();
		history.undo();

		expect(history.canRedo).toBe(true);
		expect(history.redo()).toBe(2);
		expect(history.redo()).toBe(3);
	});

	it('cannot redo past the latest state', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		history.pushState(2);

		expect(history.canRedo).toBe(false);
		expect(history.redo()).toBeUndefined();
		expect(history.current).toBe(2);
	});

	it('clears forward history on pushState after undo', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		history.pushState(2);
		history.pushState(3);

		history.undo(); // back to 2
		history.pushState(4); // branch: forward history (3) is gone

		expect(history.current).toBe(4);
		expect(history.canRedo).toBe(false);
		expect(history.undo()).toBe(2);
		expect(history.undo()).toBe(1);
	});

	it('clear resets everything', () => {
		const history = new HistoryStore<number>();
		history.pushState(1);
		history.pushState(2);

		history.clear();

		expect(history.canUndo).toBe(false);
		expect(history.canRedo).toBe(false);
		expect(history.current).toBeUndefined();
	});

	it('respects maxSize', () => {
		const history = new HistoryStore<number>(3);
		history.pushState(1);
		history.pushState(2);
		history.pushState(3);
		history.pushState(4);

		expect(history.current).toBe(4);
		expect(history.undo()).toBe(3);
		expect(history.undo()).toBe(2);
		expect(history.canUndo).toBe(false); // 1 was evicted
	});

	it('works with object types (reference check)', () => {
		const history = new HistoryStore<{ value: number }>();
		const obj = { value: 1 };
		history.pushState(obj);

		obj.value = 99;
		// store holds a reference, so it sees the mutation
		expect(history.current?.value).toBe(99);
	});

	it('full undo then full redo restores latest state', () => {
		const history = new HistoryStore<string>();
		history.pushState('a');
		history.pushState('b');
		history.pushState('c');

		history.undo();
		history.undo();
		expect(history.current).toBe('a');

		history.redo();
		history.redo();
		expect(history.current).toBe('c');
	});
});
