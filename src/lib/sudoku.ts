// Define constants for Sudoku dimensions
const GRID_SIZE = 9;
const BOX_SIZE = 3;

// Helper function to create an empty Sudoku grid
export function createEmptyGrid(): number[][] {
	return Array.from({ length: GRID_SIZE }, () => Array(GRID_SIZE).fill(0));
}

// Helper function to check if a number is valid to be placed at a given position
function isValidPlacement(grid: number[][], row: number, col: number, num: number): boolean {
	// Check row and column
	for (let i = 0; i < GRID_SIZE; i++) {
		if (grid[row][i] === num || grid[i][col] === num) {
			return false;
		}
	}

	// Check box
	const boxStartRow = Math.floor(row / BOX_SIZE) * BOX_SIZE;
	const boxStartCol = Math.floor(col / BOX_SIZE) * BOX_SIZE;
	for (let i = boxStartRow; i < boxStartRow + BOX_SIZE; i++) {
		for (let j = boxStartCol; j < boxStartCol + BOX_SIZE; j++) {
			if (grid[i][j] === num) {
				return false;
			}
		}
	}

	return true;
}

// Main function to generate a valid filled Sudoku grid
export function generateSolvedSudoku(): number[][] {
	const grid = createEmptyGrid();

	function solveSudoku(): boolean {
		for (let row = 0; row < GRID_SIZE; row++) {
			for (let col = 0; col < GRID_SIZE; col++) {
				if (grid[row][col] === 0) {
					for (let num = 1; num <= GRID_SIZE; num++) {
						if (isValidPlacement(grid, row, col, num)) {
							grid[row][col] = num;
							if (solveSudoku()) {
								return true;
							}
							grid[row][col] = 0; // Backtrack if solution not found
						}
					}
					return false; // No valid number found for this cell
				}
			}
		}
		return true; // Sudoku solved
	}

	// Fill the first row with shuffled numbers from 1 to 9
	const shuffledNumbers = Array.from({ length: GRID_SIZE }, (_, index) => index + 1);
	shuffleArray(shuffledNumbers);
	for (let i = 0; i < GRID_SIZE; i++) {
		grid[0][i] = shuffledNumbers[i];
	}

	solveSudoku();
	return grid;
}

// Helper function to shuffle an array
function shuffleArray<T>(array: T[]): void {
	for (let i = array.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[array[i], array[j]] = [array[j], array[i]];
	}
}

export function generateSudokuPuzzle(numToRemove: number): number[][] {
	const solvedGrid = generateSolvedSudoku();
	const puzzleGrid = JSON.parse(JSON.stringify(solvedGrid)); // Clone the solved grid

	const cells = [];
	for (let row = 0; row < GRID_SIZE; row++) {
		for (let col = 0; col < GRID_SIZE; col++) {
			cells.push([row, col]);
		}
	}
	shuffleArray(cells);

	for (let i = 0; i < numToRemove; i++) {
		const [row, col] = cells[i];
		puzzleGrid[row][col] = 0;
	}

	return puzzleGrid;
}
