import { SvelteSet } from 'svelte/reactivity';

type Pointer = Pick<PointerEvent, 'clientX' | 'clientY' | 'ctrlKey' | 'metaKey'>;

/** px the pointer has to move before a press becomes a drag that selects several cells */
const DRAG_THRESHOLD = 10;

function cellAtPoint(x: number, y: number): number | null {
	const attr = document.elementFromPoint(x, y)?.closest('[data-cell]')?.getAttribute('data-cell');
	return attr != null ? parseInt(attr) : null;
}

/**
 * Selected cells, driven by pointer events on the board: tap to select a cell (again to
 * deselect it), drag to select several, ctrl/cmd + tap to toggle one.
 */
export class CellSelection {
	readonly cells = new SvelteSet<number>();

	#cellAt: (x: number, y: number) => number | null;
	#dragging = false;
	#pending = false;
	#origin = { x: 0, y: 0 };
	// a press on an already selected cell; it resolves once we know if it's a tap or a drag
	#pendingClick: number | null = null;

	constructor(cellAt = cellAtPoint) {
		this.#cellAt = cellAt;
	}

	/** the cell, when exactly one is selected */
	get primary(): number | null {
		return this.cells.size === 1 ? (this.cells.values().next().value ?? null) : null;
	}

	clear() {
		this.cells.clear();
	}

	#selectOnly(index: number) {
		this.cells.clear();
		this.cells.add(index);
	}

	pointerDown(ev: Pointer, index: number) {
		if (ev.ctrlKey || ev.metaKey) {
			if (this.cells.has(index)) this.cells.delete(index);
			else this.cells.add(index);
			return;
		}

		this.#pending = true;
		this.#origin = { x: ev.clientX, y: ev.clientY };

		if (this.cells.has(index)) {
			this.#pendingClick = index;
		} else {
			this.#pendingClick = null;
			this.#selectOnly(index);
		}
	}

	pointerMove(ev: Pointer) {
		if (!this.#dragging && !this.#pending) return;

		if (!this.#dragging) {
			const dx = ev.clientX - this.#origin.x;
			const dy = ev.clientY - this.#origin.y;
			if (dx * dx + dy * dy < DRAG_THRESHOLD * DRAG_THRESHOLD) return;
			this.#dragging = true;
			this.#pending = false;
		}

		const index = this.#cellAt(ev.clientX, ev.clientY);

		// the drag left the cell it started on: start a fresh selection from there
		if (this.#pendingClick !== null && index !== this.#pendingClick) {
			this.#selectOnly(this.#pendingClick);
			this.#pendingClick = null;
		}

		if (index !== null) this.cells.add(index);
	}

	pointerUp() {
		if (this.#pendingClick !== null) {
			// tapping the only selected cell deselects it, tapping one of several keeps just it
			if (this.cells.size === 1 && this.cells.has(this.#pendingClick)) this.cells.clear();
			else this.#selectOnly(this.#pendingClick);
			this.#pendingClick = null;
		}
		this.#dragging = false;
		this.#pending = false;
	}
}
