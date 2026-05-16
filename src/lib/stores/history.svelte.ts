export class HistoryStore<T> {
	#states = $state<T[]>([]);
	#index = $state(-1);
	#maxSize?: number;

	constructor(maxSize?: number) {
		this.#maxSize = maxSize;
	}

	get canUndo() {
		return this.#index > 0;
	}

	get canRedo() {
		return this.#index < this.#states.length - 1;
	}

	get current() {
		if (this.#index < 0 || this.#index >= this.#states.length) return undefined;
		return this.#states[this.#index];
	}

	pushState(state: T) {
		this.#states.length = this.#index + 1;
		this.#states.push(state);
		this.#index++;

		if (this.#maxSize && this.#states.length > this.#maxSize) {
			this.#states.shift();
			this.#index--;
		}
	}

	undo(): T | undefined {
		if (!this.canUndo) return undefined;
		this.#index--;
		return this.current;
	}

	redo(): T | undefined {
		if (!this.canRedo) return undefined;
		this.#index++;
		return this.current;
	}

	clear() {
		this.#states = [];
		this.#index = -1;
	}
}
